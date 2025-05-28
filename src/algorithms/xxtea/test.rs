mod {

}
#[test]
#[ignore]
fn xxtea() {
    let key: &str = "SecretKey";

    let data: [u8; 5] = [11, 13, 0, 14, 15];

    println!("Data: {:?}", data);

    let encrypted_data = encrypt_raw(&data.to_vec(), &key);

    println!("Encrypted data: {:?}", encrypted_data);

    let decrypted_data = decrypt_raw(&encrypted_data, &key);

    println!("Decrypted data: {:?}", decrypted_data);

    assert!(data.iter().eq(decrypted_data[0..data.len()].iter()));
}

#[test]
#[ignore]
fn xxtea_2() {
    let key: &str = "SecretKey";

    let data = "Hellouw";

    println!("Data: {:?}", data);

    let encrypted_data = encrypt(data, &key);

    println!("Encrypted data: {:?}", encrypted_data);

    let decrypted_data = String::from_utf8(decrypt(&encrypted_data, key)).unwrap();

    println!("Decrypted data: {:?}", decrypted_data);

    assert!(data.eq(&decrypted_data));
}

#[test]
fn xxtea_3() {
    let starting = "Hellouw".as_bytes();

    let u32s = to_u32(&starting, true);
    let bytes = to_bytes(&u32s, true);

    assert_eq!(starting, bytes);

    let u32s = to_u32(&starting, false);
    let bytes = to_bytes(&u32s, false);

    assert_ne!(starting, bytes);
}
