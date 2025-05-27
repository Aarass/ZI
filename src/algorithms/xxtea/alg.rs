use crate::Algorithm;

const DELTA: u32 = 0x9e3779b9;

#[allow(clippy::upper_case_acronyms)]
pub struct XXTEA {}

impl Algorithm for XXTEA {
    fn encrypt(&self, data: &[u8], key: String) -> anyhow::Result<Vec<u8>> {
        let key = key.as_bytes();
        let res = to_bytes(&encrypt_(to_u32(&data, true), &to_u32(&key, false)), false);

        Ok(res)
    }

    fn decrypt(&self, data: &[u8], key: String) -> anyhow::Result<Vec<u8>> {
        let key = key.as_bytes();
        let res = to_bytes(&decrypt_(to_u32(&data, false), &to_u32(&key, false)), true);

        Ok(res)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct XXTEA_CFB {}

const BLOCK_SIZE: usize = 8;
const IV: [u8; 10] = [0u8; 10];

impl Algorithm for XXTEA_CFB {
    fn encrypt(&self, data: &[u8], key: String) -> anyhow::Result<Vec<u8>> {
        let key = key.as_bytes();

        let mut res = Vec::with_capacity(data.len());

        let mut prev = IV[0..BLOCK_SIZE].to_vec();

        for plainblock in data.chunks(BLOCK_SIZE) {
            let intermidiate =
                to_bytes(&encrypt_(to_u32(&prev, false), &to_u32(&key, false)), false);

            assert_eq!(intermidiate.len(), BLOCK_SIZE);

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

    fn decrypt(&self, data: &[u8], key: String) -> anyhow::Result<Vec<u8>> {
        let key = key.as_bytes();

        let mut res = Vec::with_capacity(data.len());

        let mut prev = IV[0..BLOCK_SIZE].to_vec();

        for cipherblock in data.chunks(BLOCK_SIZE) {
            let intermidiate =
                to_bytes(&encrypt_(to_u32(&prev, false), &to_u32(&key, false)), false);

            assert_eq!(intermidiate.len(), BLOCK_SIZE);

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

#[test]
fn cfb() {
    let alg = XXTEA_CFB {};

    let data = "Hellouw there".as_bytes();
    let key: &str = "SecretKey";

    let encrypted = alg.encrypt(data, key.to_owned()).unwrap();

    println!("Encrypted data: {:?}", encrypted);
    println!("Encrypted data: {:?}", String::from_utf8_lossy(&encrypted));

    let decrypted = alg.decrypt(&encrypted, key.to_owned()).unwrap();

    println!("Decrypted data: {:?}", decrypted);
    println!("Decrypted data: {:?}", String::from_utf8_lossy(&decrypted));
}

fn encrypt_(mut v: Vec<u32>, k: &[u32]) -> Vec<u32> {
    let length: u32 = v.len() as u32;
    let key = fixk(k);

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

fn decrypt_(mut v: Vec<u32>, k: &[u32]) -> Vec<u32> {
    let length: u32 = v.len() as u32;
    let key = fixk(k);

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

pub fn encrypt(data: &[u8], key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    to_bytes(&encrypt_(to_u32(&data, true), &to_u32(&key, false)), false)
}

pub fn decrypt(data: &[u8], key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    to_bytes(&decrypt_(to_u32(&data, false), &to_u32(&key, false)), true)
}

pub fn encrypt_raw(data: &[u8], key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    to_bytes(&encrypt_(to_u32(&data, false), &to_u32(&key, false)), false)
}

pub fn decrypt_raw(data: &[u8], key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    to_bytes(&decrypt_(to_u32(&data, false), &to_u32(&key, false)), false)
}

fn to_bytes(v: &[u32], include_length: bool) -> Vec<u8> {
    let length: u32 = v.len() as u32;

    let mut n: u32 = length << 2;

    if include_length {
        let m: u32 = v[length as usize - 1];

        n = n - 4;

        assert!(!((m < n - 3) || (m > n)));

        n = m;
    }

    let mut bytes: Vec<u8> = vec![0; n as usize];

    for i in 0..n {
        bytes[i as usize] = (v[(i >> 2) as usize] >> ((i & 3) << 3)) as u8;
    }

    return bytes;
}

fn to_u32(bytes: &[u8], include_length: bool) -> Vec<u32> {
    let length: u32 = bytes.len() as u32;

    let mut n: u32 = length >> 2;

    if length & 3 != 0 {
        n = n + 1;
    }

    let mut v;

    if include_length {
        v = vec![0; n as usize + 1];
        v[n as usize] = length;
    } else {
        v = vec![0; n as usize];
    }

    for i in 0..length {
        v[(i >> 2) as usize] |= (bytes[i as usize] as u32) << ((i & 3) << 3) as u32;
    }

    return v;
}

fn mx(sum: u32, y: u32, z: u32, p: u32, e: u32, k: &[u32; 4]) -> u32 {
    ((z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4))
        ^ ((sum ^ y).wrapping_add(k[(p & 3 ^ e) as usize] ^ z))
}

fn fixk(k: &[u32]) -> [u32; 4] {
    let mut out = [0u32; 4];
    for (i, &val) in k.iter().take(4).enumerate() {
        out[i] = val;
    }
    out
}

#[test]
#[ignore]
fn xxtea_1() {
    let starting = "Hellouw".as_bytes();

    let u32s = to_u32(&starting, true);
    let bytes = to_bytes(&u32s, true);

    assert_eq!(starting, bytes);

    let u32s = to_u32(&starting, false);
    let bytes = to_bytes(&u32s, false);

    assert_ne!(starting, bytes);
}

#[test]
#[ignore]
fn xxtea_2() {
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
#[ignore]
fn xxtea_3() {
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
    assert!(data.eq(&result));
}
