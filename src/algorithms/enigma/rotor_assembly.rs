use super::rotor::Rotor;

// pub trait RotorAssembly {
//     fn rotate(&mut self);
//     fn get_output(&self, letter: u8) -> u8;
//     fn get_output_inverse(&self, letter: u8) -> u8;
// }

#[derive(Clone)]
pub struct RotorAssembly(pub(super) [Rotor; 3]);

impl RotorAssembly {
    pub fn new(rotors: [Rotor; 3]) -> RotorAssembly {
        RotorAssembly(rotors)
    }

    pub fn rotate(&mut self) {
        for i in 0..2 {
            if self.0[i].is_aligned() || self.0[i + 1].is_aligned() {
                self.0[i].position += 1;
            }
        }
        self.0[2].position += 1;
    }

    pub fn get_output(&self, letter: u8) -> u8 {
        let l1 = self.0[2].get_output(letter);
        let l2 = self.0[1].get_output(l1);
        let l3 = self.0[0].get_output(l2);

        return l3;
    }

    pub fn get_output_inverse(&self, letter: u8) -> u8 {
        let l1 = self.0[0].get_output_inverted(letter);
        let l2 = self.0[1].get_output_inverted(l1);
        let l3 = self.0[2].get_output_inverted(l2);

        return l3;
    }
}
