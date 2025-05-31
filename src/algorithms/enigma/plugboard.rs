use super::utils::to_u8_array_26;

#[derive(Clone)]
pub struct Plugboard {
    wiring: [u8; 26],
}

impl Plugboard {
    pub fn new(pairs: &str) -> Self {
        assert!(pairs.is_ascii());

        let mut wiring = to_u8_array_26("abcdefghijklmnopqrstuvwxyz");

        if pairs.is_empty() {
            return Plugboard { wiring };
        }

        pairs.split(' ').for_each(|pair| {
            assert_eq!(pair.len(), 2);

            let pair = pair.to_ascii_lowercase();
            let mut chars = pair.chars();

            let first = (chars.next().unwrap() as u8) - b'a';
            let second = (chars.next().unwrap() as u8) - b'a';

            wiring.swap(first as usize, second as usize);
        });

        Plugboard { wiring }
    }

    pub fn get_output(&self, letter: u8) -> u8 {
        assert!(letter.is_ascii_lowercase());

        let index = letter - b'a';

        return self.wiring[index as usize];
    }
}
