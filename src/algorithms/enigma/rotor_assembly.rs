use super::rotor::Rotor;

pub trait RotorAssembly {
    fn rotate(&mut self);
    fn get_output(&self, letter: u8) -> u8;
    fn get_output_inverse(&self, letter: u8) -> u8;
}

impl RotorAssembly for [Rotor; 3] {
    fn rotate(&mut self) {
        for i in 0..2 {
            if self[i].is_aligned() || self[i + 1].is_aligned() {
                self[i].position += 1;
            }
        }
        self[2].position += 1;
    }

    fn get_output(&self, letter: u8) -> u8 {
        let l1 = self[2].get_output(letter);
        let l2 = self[1].get_output(l1);
        let l3 = self[0].get_output(l2);

        return l3;
    }

    fn get_output_inverse(&self, letter: u8) -> u8 {
        let l1 = self[0].get_output_inverted(letter);
        let l2 = self[1].get_output_inverted(l1);
        let l3 = self[2].get_output_inverted(l2);

        return l3;
    }
}
