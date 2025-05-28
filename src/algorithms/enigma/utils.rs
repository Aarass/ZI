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

pub fn is_shuffled_alphabet(s: &str) -> bool {
    if s.len() != 26 {
        return false;
    }

    let mut seen = [false; 26];

    for c in s.chars() {
        if !c.is_ascii_lowercase() {
            return false;
        }
        let idx = (c as u8 - b'a') as usize;
        if seen[idx] {
            return false;
        }
        seen[idx] = true;
    }

    true
}
