use anyhow::anyhow;

use crate::{Algorithm, EnigmaArgs};

use super::{
    plugboard::Plugboard, reflector::Reflector, rotor::Rotor, rotor_assembly::RotorAssembly, utils,
};

pub struct Enigma {
    reflector: Reflector,
    rotor_assembly: RotorAssembly,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn try_new(args: &EnigmaArgs) -> anyhow::Result<Enigma> {
        let is_ok = Enigma::validate_args(args);

        if is_ok {
            Ok(Enigma {
                reflector: Reflector {
                    wiring: utils::to_u8_array_26(&args.refl_wiring.as_ref().unwrap()),
                },
                rotor_assembly: RotorAssembly::new([
                    Rotor {
                        wiring: utils::to_u8_array_26(&args.rot1_wiring.as_ref().unwrap()),
                        notch_position: args.rot1_notch.as_ref().unwrap().parse().unwrap(),
                        position: args.rot1_position.as_ref().unwrap().parse().unwrap(),
                    },
                    Rotor {
                        wiring: utils::to_u8_array_26(&args.rot2_wiring.as_ref().unwrap()),
                        notch_position: args.rot2_notch.as_ref().unwrap().parse().unwrap(),
                        position: args.rot2_position.as_ref().unwrap().parse().unwrap(),
                    },
                    Rotor {
                        wiring: utils::to_u8_array_26(&args.rot3_wiring.as_ref().unwrap()),
                        notch_position: args.rot3_notch.as_ref().unwrap().parse().unwrap(),
                        position: args.rot3_position.as_ref().unwrap().parse().unwrap(),
                    },
                ]),
                plugboard: Plugboard::new(&args.plugboard.clone().unwrap_or_default()),
            })
        } else {
            Err(anyhow!("Validation failed"))
        }
    }

    fn validate_args(args: &EnigmaArgs) -> bool {
        if args.refl_wiring.is_none()
            || args.rot1_wiring.is_none()
            || args.rot1_notch.is_none()
            || args.rot1_position.is_none()
            || args.rot2_wiring.is_none()
            || args.rot2_notch.is_none()
            || args.rot2_position.is_none()
            || args.rot3_wiring.is_none()
            || args.rot3_notch.is_none()
            || args.rot3_position.is_none()
        {
            return false;
        }

        println!("All are some");

        if !utils::is_shuffled_alphabet(args.refl_wiring.as_ref().unwrap()) {
            return false;
        }

        if !utils::is_shuffled_alphabet(args.rot1_wiring.as_ref().unwrap()) {
            return false;
        }
        if !utils::is_shuffled_alphabet(args.rot2_wiring.as_ref().unwrap()) {
            return false;
        }
        if !utils::is_shuffled_alphabet(args.rot3_wiring.as_ref().unwrap()) {
            return false;
        }

        println!("4 shuffled alphabets");

        if !is_index(args.rot1_notch.as_ref().unwrap()) {
            return false;
        }
        if !is_index(args.rot2_notch.as_ref().unwrap()) {
            return false;
        }
        if !is_index(args.rot3_notch.as_ref().unwrap()) {
            return false;
        }

        println!("notches");

        if !is_index(args.rot1_position.as_ref().unwrap()) {
            return false;
        }
        if !is_index(args.rot2_position.as_ref().unwrap()) {
            return false;
        }
        if !is_index(args.rot3_position.as_ref().unwrap()) {
            return false;
        }

        println!("positions");

        fn is_index(s: &str) -> bool {
            match s.parse::<u8>() {
                Ok(n) => return n <= 25,
                Err(_) => return false,
            }
        }

        if !args
            .plugboard
            .clone()
            .unwrap_or_default()
            .split_whitespace()
            .all(|pair| (pair.len() == 2) && pair.chars().all(|c| c.is_ascii_lowercase()))
        {
            return false;
        }

        println!("plugboard");

        true
    }
}

#[test]
fn to_ascii_lowercase() {
    println!("{}", "a b".to_ascii_lowercase());
}

impl Algorithm for Enigma {
    fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut rotors = self.rotor_assembly.clone();
        let reflector = self.reflector.clone();
        let plugboard = self.plugboard.clone();

        Ok(data
            .iter()
            .filter_map(|&b| match b {
                b'A'..=b'Z' => Some(b + 32),
                b'a'..=b'z' => Some(b),
                _ => None,
            })
            .map(|letter| {
                rotors.rotate();

                let l1 = plugboard.get_output(letter);

                let l2 = rotors.get_output(l1);
                let l3 = reflector.reflect(l2);
                let l4 = rotors.get_output_inverse(l3);

                let l5 = plugboard.get_output(l4);

                l5
            })
            .collect())
    }

    fn decrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        self.encrypt(data)
    }
}
// ekmflgdqvzntowyhxuspaibrcj 8 0 ajdksiruxblhwtmcqgznpyfvoe 8 0 bdfhjlcprtxvznyeiwgakmusqo 0 0 yruhqsldpxngokmiebfzcwvjat PO ML IU KJ NH YT GB VF RE DC
