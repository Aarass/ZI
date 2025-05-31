#[derive(Debug, Clone)]
pub struct Rotor {
    pub wiring: [u8; 26],
    pub notch_position: u8,
    pub position: usize,
}

impl Rotor {
    pub fn get_output(&self, letter: u8) -> u8 {
        assert!(letter.is_ascii_lowercase());

        let index = letter - b'a';
        let index = (index as usize + self.position) % 26;

        return self.wiring[index];
    }

    pub fn get_output_inverted(&self, letter: u8) -> u8 {
        assert!(letter.is_ascii_lowercase());

        let index = self.wiring.iter().position(|el| *el == letter).unwrap();
        let index = (index + (26 - self.position % 26)) % 26;

        return b'a' + (index as u8);
    }

    pub fn is_aligned(&self) -> bool {
        return (self.position % 26) == ((self.notch_position + 19) % 26) as usize;
    }
}
