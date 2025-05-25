pub const fn to_u8_array_13(s: &str) -> [u8; 13] {
    assert!(s.len() == 13);

    let bytes = s.as_bytes();
    let mut arr = [0u8; 13];
    let mut i = 0;
    while i < 13 {
        arr[i] = bytes[i];
        i += 1;
    }

    return arr;
}

pub const fn to_u8_array_26(s: &str) -> [u8; 26] {
    assert!(s.len() == 26);

    let bytes = s.as_bytes();
    let mut arr = [0u8; 26];
    let mut i = 0;
    while i < 26 {
        arr[i] = bytes[i];
        i += 1;
    }

    return arr;
}
