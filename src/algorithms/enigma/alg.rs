use anyhow::anyhow;

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
        if !key.is_ascii() || key.len() < 119 {
            return Err(anyhow!("Key must be valid ascii"));
        }

        let key = key.to_ascii_lowercase();

        if key.len() < 119 {
            return Err(anyhow!("Key length is too low"));
        }

        let mut parts = key.split_whitespace();

        fn get_wiring<'a>(part: &'a str) -> Result<&'a str, anyhow::Error> {
            if part.len() != 26 || !part.chars().all(|c| c.is_ascii_alphabetic()) {
                return Err(anyhow!("Wiring is in wrong format"));
            }

            Ok(part)
        }

        fn get_notch(part: &str) -> Result<u8, anyhow::Error> {
            if !part.chars().all(|c| c.is_ascii_digit()) {
                return Err(anyhow!("Notch should be all digits"));
            }

            let notch: u8 = part.parse()?;

            if notch > 25 {
                return Err(anyhow!("Notch should be in the range 0 to 25"));
            }

            Ok(notch)
        }

        fn get_position(part: &str) -> Result<u8, anyhow::Error> {
            if !part.chars().all(|c| c.is_ascii_digit()) {
                return Err(anyhow!("Wrong format"));
            }

            let position: u8 = part.parse()?;

            if position > 25 {
                return Err(anyhow!("Wrong format"));
            }

            Ok(position)
        }

        let p1 = get_wiring(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;
        let p2 = get_notch(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;
        let p3 = get_position(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;

        let r1 = Rotor {
            wiring: utils::to_u8_array_26(p1),
            notch_position: p2,
            position: p3 as usize,
        };

        let p1 = get_wiring(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;
        let p2 = get_notch(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;
        let p3 = get_position(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;

        let r2 = Rotor {
            wiring: utils::to_u8_array_26(p1),
            notch_position: p2,
            position: p3 as usize,
        };

        let p1 = get_wiring(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;
        let p2 = get_notch(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;
        let p3 = get_position(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;

        let r3 = Rotor {
            wiring: utils::to_u8_array_26(p1),
            notch_position: p2,
            position: p3 as usize,
        };

        let mut rotors = [r1, r2, r3];

        let p1 = get_wiring(&parts.next().ok_or(anyhow!("Unexpected end of iterator"))?)?;

        let reflector = Reflector {
            wiring: utils::to_u8_array_26(p1),
        };

        let plugboard = Plugboard::new(&parts.collect::<Vec<&str>>().join(" "));

        Ok(data
            .to_ascii_lowercase()
            .iter()
            .map(|letter| {
                rotors.rotate();

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
// ekmflgdqvzntowyhxuspaibrcj 8 0 ajdksiruxblhwtmcqgznpyfvoe 8 0 bdfhjlcprtxvznyeiwgakmusqo 0 0 yruhqsldpxngokmiebfzcwvjat PO ML IU KJ NH YT GB VF RE DC
