#[derive(Debug, Clone)]
pub struct Reflector {
    pub wiring: [u8; 26],
}

impl Reflector {
    pub fn reflect(&self, letter: u8) -> u8 {
        assert!(letter.is_ascii_lowercase());

        let index = (letter - b'a') as usize;
        assert!(index < 26);

        return self.wiring[index];

        // if index < 13 {
        //     return self.wiring[index];
        // }
        //
        // let index = self.wiring.iter().position(|el| *el == letter).unwrap();
        // return ('a' as u8) + (index as u8);
    }
}
