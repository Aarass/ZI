use crate::algorithms::Algorithm;
use crate::gui::state::args::{XxteaArgs, XxteaCfbArgs};
use anyhow::{anyhow, Ok};

const DELTA: u32 = 0x9e3779b9;

pub struct Xxtea {
    key: [u32; 4],
}

impl Xxtea {
    pub fn try_new(args: &XxteaArgs) -> anyhow::Result<Xxtea> {
        return Ok(Xxtea {
            key: fix_key(&to_u32(
                args.key
                    .as_ref()
                    .ok_or(anyhow!("Validation failed"))?
                    .as_bytes(),
                false,
            )),
        });
    }
}

impl Algorithm for Xxtea {
    fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = to_u32(&data, true);
        let encrypted = encrypt_(data, &self.key);

        Ok(to_bytes(&encrypted, false))
    }

    fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let data = to_u32(&data, false);
        let decrypted = decrypt_(data, &self.key);

        Ok(to_bytes(&decrypted, true))
    }
}

pub struct XxteaCfb {
    iv: Vec<u8>,
    block_size: usize,
    key: [u32; 4],
}

impl XxteaCfb {
    pub fn try_new(args: &XxteaCfbArgs) -> anyhow::Result<XxteaCfb> {
        if args.iv.is_none() || args.block_size.is_none() || args.key.is_none() {
            return Err(anyhow!("Some fields are missing"));
        }

        let block_size = args.block_size.as_ref().unwrap().parse::<usize>()?;

        if block_size < 8 {
            return Err(anyhow!("Block Size must be 8 or more"));
        }

        let iv = args.iv.as_ref().unwrap().to_owned().into_bytes();

        if iv.is_empty() {
            return Err(anyhow!("IV is empty"));
        }

        if iv.len() < block_size {
            return Err(anyhow!("IV must be at least Block Size long"));
        }

        return Ok(XxteaCfb {
            iv,
            block_size,
            key: fix_key(&to_u32(
                args.key
                    .as_ref()
                    .ok_or(anyhow!("Validation failed"))?
                    .as_bytes(),
                false,
            )),
        });
    }
}

impl Algorithm for XxteaCfb {
    fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut res = Vec::with_capacity(data.len());

        let mut prev = self.iv[0..self.block_size].to_vec();

        for plainblock in data.chunks(self.block_size) {
            let intermidiate = to_bytes(&encrypt_(to_u32(&prev, false), &self.key), false);

            assert_eq!(intermidiate.len(), self.block_size);

            let ciphertext: Vec<u8> = plainblock
                .iter()
                .zip(intermidiate)
                .map(|(a, b)| a ^ b)
                .collect();

            res.extend(ciphertext.iter());
            prev = ciphertext;
        }

        Ok(res)
    }

    fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut res = Vec::with_capacity(data.len());

        let mut prev = self.iv[0..self.block_size].to_vec();

        for cipherblock in data.chunks(self.block_size) {
            let intermidiate = to_bytes(&encrypt_(to_u32(&prev, false), &self.key), false);

            assert_eq!(intermidiate.len(), self.block_size);

            let plaintext: Vec<u8> = cipherblock
                .iter()
                .zip(intermidiate)
                .map(|(a, b)| a ^ b)
                .collect();

            res.extend(plaintext.iter());
            prev = cipherblock.to_vec();
        }

        Ok(res)
    }
}

fn encrypt_(mut v: Vec<u32>, key: &[u32; 4]) -> Vec<u32> {
    let length: u32 = v.len() as u32;

    let n: u32 = length - 1;

    let mut e: u32;
    let mut y: u32;
    let mut z = v[n as usize];
    let mut q: u32 = 6 + 52 / length;

    let mut sum: u32 = 0;

    while q > 0 {
        sum = sum.wrapping_add(DELTA);
        e = sum >> 2 & 3;

        for p in 0..n {
            y = v[(p as usize) + 1];
            v[p as usize] = v[p as usize].wrapping_add(mx(sum, y, z, p as u32, e, &key));
            z = v[p as usize];
        }

        y = v[0];
        v[n as usize] = v[n as usize].wrapping_add(mx(sum, y, z, n, e, &key));
        z = v[n as usize];
        q = q - 1;
    }

    return v;
}

fn decrypt_(mut v: Vec<u32>, key: &[u32; 4]) -> Vec<u32> {
    let length: u32 = v.len() as u32;

    let n: u32 = length - 1;

    let mut e: u32;
    let mut y: u32 = v[0];
    let mut z;
    let q: u32 = 6 + 52 / length;

    let mut sum: u32 = q.wrapping_mul(DELTA);

    while sum != 0 {
        e = sum >> 2 & 3;
        let mut p: usize = n as usize;

        while p > 0 {
            z = v[p - 1];
            v[p] = v[p].wrapping_sub(mx(sum, y, z, p as u32, e, &key));
            y = v[p];
            p = p - 1;
        }

        z = v[n as usize];
        v[0] = v[0].wrapping_sub(mx(sum, y, z, 0, e, &key));
        y = v[0];

        sum = sum.wrapping_sub(DELTA);
    }

    return v;
}

fn mx(sum: u32, y: u32, z: u32, p: u32, e: u32, k: &[u32; 4]) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3 ^ e) as usize] ^ z))
}

fn fix_key(key: &[u32]) -> [u32; 4] {
    let mut out = [0u32; 4];

    for (i, &val) in key.iter().take(4).enumerate() {
        out[i] = val;
    }

    return out;
}

fn to_bytes(arr: &[u32], include_length: bool) -> Vec<u8> {
    let length: u32 = arr.len() as u32;

    let mut bytes_count = length * 4;

    if include_length {
        let original_length: u32 = arr[length as usize - 1];

        {
            // Checking validity
            bytes_count = bytes_count - 4;
            assert!(!((original_length < bytes_count - 3) || (original_length > bytes_count)));
        }

        bytes_count = original_length;
    }

    let mut bytes: Vec<u8> = vec![0; bytes_count as usize];

    for i in 0..bytes_count {
        bytes[i as usize] = (arr[(i >> 2) as usize] >> ((i & 3) << 3)) as u8;
    }

    return bytes;
}

fn to_u32(bytes: &[u8], include_length: bool) -> Vec<u32> {
    let length: u32 = bytes.len() as u32;
    let n = length.div_ceil(4);

    let mut output = vec![0; n as usize + if include_length { 1 } else { 0 }];
    if include_length {
        output[n as usize] = length;
    }

    for i in 0..length {
        output[(i >> 2) as usize] |= (bytes[i as usize] as u32) << ((i & 3) << 3) as u32;
    }

    return output;
}

#[test]
fn xxtea_1() {
    let starting = "Hellouw".as_bytes();

    let u32s = to_u32(&starting, true);
    let bytes = to_bytes(&u32s, true);

    assert_eq!(starting, bytes);

    let u32s = to_u32(&starting, false);
    let bytes = to_bytes(&u32s, false);

    assert_ne!(starting, bytes);
}

#[cfg(test)]
mod tests {
    use super::{decrypt_, encrypt_, fix_key, to_bytes, to_u32, Xxtea, XxteaCfb};
    use crate::algorithms::Algorithm;
    use crate::gui::state::args::{XxteaArgs, XxteaCfbArgs};

    #[test]
    fn xxtea_2() {
        fn encrypt_raw(data: &[u8], key: &str) -> Vec<u8> {
            let key = fix_key(&to_u32(key.as_bytes(), false));

            to_bytes(&encrypt_(to_u32(&data, false), &key), false)
        }

        fn decrypt_raw(data: &[u8], key: &str) -> Vec<u8> {
            let key = fix_key(&to_u32(key.as_bytes(), false));
            to_bytes(&decrypt_(to_u32(&data, false), &key), false)
        }

        let key: &str = "SecretKey";

        let data: [u8; 5] = [11, 13, 0, 14, 15];
        println!("Data: {:?}", data);
        let encrypted_data = encrypt_raw(&data, key);
        println!("Encrypted data: {:?}", encrypted_data);

        let decrypted_data = decrypt_raw(&encrypted_data, key);
        println!("Decrypted data: {:?}", decrypted_data);

        assert!(data.iter().eq(decrypted_data[0..data.len()].iter()));
    }

    #[test]
    fn xxtea_3() {
        fn encrypt(data: &[u8], key: &str) -> Vec<u8> {
            let key = fix_key(&to_u32(key.as_bytes(), false));
            to_bytes(&encrypt_(to_u32(&data, true), &key), false)
        }

        fn decrypt(data: &[u8], key: &str) -> Vec<u8> {
            let key = fix_key(&to_u32(key.as_bytes(), false));
            to_bytes(&decrypt_(to_u32(&data, false), &key), true)
        }

        let key: &str = "SecretKey";

        let data = "Hellouw";
        println!("Data: {:?}", data);

        let encrypted_data = encrypt(&data.as_bytes(), key);
        println!("Encrypted data: {:?}", encrypted_data);
        println!(
            "Encrypted data: {:?}",
            String::from_utf8_lossy(&encrypted_data)
        );

        let decrypted_data = decrypt(&encrypted_data, key);
        println!("Decrypted data: {:?}", decrypted_data);

        let result = String::from_utf8(decrypted_data).unwrap();

        assert_eq!(data, result)
    }

    #[test]
    fn xxtea_full() {
        let alg = Xxtea::try_new(&XxteaArgs {
            key: Some("SecretKey".to_owned()),
        })
        .unwrap();

        let data = "Hellouw there".as_bytes();

        let encrypted = alg.encrypt(data).unwrap();

        println!("Encrypted data: {:?}", encrypted);
        println!("Encrypted data: {:?}", String::from_utf8_lossy(&encrypted));

        let decrypted = alg.decrypt(&encrypted).unwrap();

        println!("Decrypted data: {:?}", decrypted);
        println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));

        assert_eq!(data, decrypted)
    }

    #[test]
    fn cfb() {
        let alg = XxteaCfb::try_new(&XxteaCfbArgs {
            iv: Some("asdfas34asdfasdfasdkljsdklfj".to_owned()),
            block_size: Some("8".to_owned()),
            key: Some("SecretKey".to_owned()),
        })
        .unwrap();

        let data = "Hellouw there".as_bytes();

        let encrypted = alg.encrypt(data).unwrap();

        println!("Encrypted data: {:?}", encrypted);
        println!("Encrypted data: {:?}", String::from_utf8_lossy(&encrypted));

        let decrypted = alg.decrypt(&encrypted).unwrap();

        println!("Decrypted data: {:?}", decrypted);
        println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));

        assert_eq!(data, decrypted)
    }
}
