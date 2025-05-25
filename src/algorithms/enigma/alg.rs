use crate::Algorithm;

use super::{
    plugboard::Plugboard,
    reflector::Reflector,
    rotor::Rotor,
    rotor_assembly::RotorAssembly,
    utils::{self, to_u8_array_26},
};
pub struct Enigma {}

impl Algorithm for Enigma {
    fn encrypt(&self, data: &[u8], key: String) -> anyhow::Result<Vec<u8>> {
        let _ = key;

        let mut rotors = [
            Rotor {
                wiring: utils::to_u8_array_26("ekmflgdqvzntowyhxuspaibrcj"),
                notch_position: 8,
                position: 0,
            },
            Rotor {
                wiring: utils::to_u8_array_26("ajdksiruxblhwtmcqgznpyfvoe"),
                notch_position: 8,
                position: 0,
            },
            Rotor {
                wiring: utils::to_u8_array_26("bdfhjlcprtxvznyeiwgakmusqo"),
                notch_position: 0,
                position: 0,
            },
        ];

        let reflector = Reflector {
            wiring: utils::to_u8_array_26("yruhqsldpxngokmiebfzcwvjat"),
        };

        let plugboard = Plugboard::new("PO ML IU KJ NH YT GB VF RE DC");

        Ok(data
            .to_ascii_lowercase()
            .iter()
            .map(|letter| {
                let l1 = plugboard.get_output(*letter);

                let l2 = rotors.get_output(l1);
                let l3 = reflector.reflect(l2);
                let l4 = rotors.get_output_inverse(l3);

                let l5 = plugboard.get_output(l4);

                l5
            })
            .collect())
    }

    fn decrypt(&self, data: &[u8], key: String) -> anyhow::Result<Vec<u8>> {
        self.encrypt(data, key)
    }
}
