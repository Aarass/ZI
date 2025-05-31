use anyhow::anyhow;

use crate::{algorithms::Algorithm, gui::state::args::EnigmaArgs};

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

//
//
//
//
//
#[cfg(test)]
mod tests {
    use crate::algorithms::enigma::plugboard::Plugboard;
    use crate::algorithms::enigma::{
        alg::Enigma, reflector::Reflector, rotor::Rotor, rotor_assembly::RotorAssembly, utils,
    };
    use crate::gui::state::args::EnigmaArgs;

    fn expected_output(input: &[u8]) -> Vec<u8> {
        input
            .iter()
            .filter_map(|&b| match b {
                b'A'..=b'Z' => Some(b + 32),
                b'a'..=b'z' => Some(b),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn test_alg() {
        let str = "Hello asdjfk df asdf asd";
        let input = str.as_bytes();

        let enigma = Enigma::try_new(&EnigmaArgs::default()).unwrap();
        let encrypted = enigma.encrypt(&input).unwrap();
        let decrypted = enigma.encrypt(&encrypted).unwrap();

        assert_eq!(decrypted, expected_output(input));

        println!("{} {}", str, String::from_utf8(decrypted).unwrap());
    }

    #[test]
    fn test_rotor_simple() {
        let rotor = Rotor {
            wiring: utils::to_u8_array_26("ekmflgdqvzntowyhxuspaibrcj"),
            notch_position: 0,
            position: 0,
        };

        assert_eq!(rotor.get_output('a' as u8), 'e' as u8);
        assert_eq!(rotor.get_output('b' as u8), 'k' as u8);

        assert_eq!(rotor.get_output_inverted('e' as u8), 'a' as u8);
        assert_eq!(rotor.get_output_inverted('k' as u8), 'b' as u8);
    }

    #[test]
    fn test_rotor_with_offset() {
        let mut rotor = Rotor {
            wiring: utils::to_u8_array_26("ekmflgdqvzntowyhxuspaibrcj"),
            notch_position: 0,
            position: 1,
        };

        assert_eq!(rotor.get_output('a' as u8), 'k' as u8);
        assert_eq!(rotor.get_output('b' as u8), 'm' as u8);

        assert_eq!(rotor.get_output_inverted('k' as u8), 'a' as u8);
        assert_eq!(rotor.get_output_inverted('m' as u8), 'b' as u8);

        rotor.position = 2;
        assert_eq!(rotor.get_output_inverted('k' as u8), 'z' as u8);
    }

    #[test]
    fn test_assembly() {
        // Example
        // https://www.codesandciphers.org.uk/enigma/example1.htm

        let rotors = RotorAssembly::new([
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
        ]);

        let reflector = Reflector {
            wiring: utils::to_u8_array_26("yruhqsldpxngokmiebfzcwvjat"),
        };

        let l1 = rotors.get_output('g' as u8);
        assert_eq!(l1, 'f' as u8);

        let l2 = reflector.reflect(l1);
        assert_eq!(l2, 's' as u8);

        let l3 = rotors.get_output_inverse(l2);
        assert_eq!(l3, 'p' as u8);
    }

    #[test]
    fn test_rotor_alignment() {
        let mut rotor = Rotor {
            wiring: utils::to_u8_array_26("ekmflgdqvzntowyhxuspaibrcj"),
            notch_position: 0,
            position: 0,
        };

        rotor.notch_position = 7;
        rotor.position = 1;
        assert!(rotor.is_aligned() == false);

        rotor.notch_position = 7;
        rotor.position = 0;
        assert!(rotor.is_aligned());

        rotor.notch_position = 8;
        rotor.position = 1;
        assert!(rotor.is_aligned());

        rotor.notch_position = 6;
        rotor.position = 25;
        assert!(rotor.is_aligned());
    }

    // #[test]
    // fn test_rotating() {
    //     let mut rotors = RotorAssembly::new([
    //         Rotor {
    //             wiring: utils::to_u8_array_26("ekmflgdqvzntowyhxuspaibrcj"),
    //             notch_position: 0,
    //             position: 0,
    //         },
    //         Rotor {
    //             wiring: utils::to_u8_array_26("ajdksiruxblhwtmcqgznpyfvoe"),
    //             notch_position: 8,
    //             position: 7,
    //         },
    //         Rotor {
    //             wiring: utils::to_u8_array_26("bdfhjlcprtxvznyeiwgakmusqo"),
    //             notch_position: 8,
    //             position: 7,
    //         },
    //     ]);
    //
    //     rotors.rotate();
    //     assert_eq!(rotors.0[2].position, 8);
    //     assert_eq!(rotors.0[1].position, 0);
    //     assert_eq!(rotors.0[0].position, 0);
    //
    //     rotors.rotate();
    //     assert_eq!(rotors.0[2].position, 9);
    //     assert_eq!(rotors.0[1].position, 1);
    //     assert_eq!(rotors.0[0].position, 0);
    //
    //     rotors.rotate();
    //     assert_eq!(rotors.0[2].position, 10);
    //     assert_eq!(rotors.0[1].position, 2);
    //     assert_eq!(rotors.0[0].position, 1);
    // }

    #[test]
    fn test_rotating_intense() {
        let mut rotors = RotorAssembly::new([
            Rotor {
                wiring: utils::to_u8_array_26("bdfhjlcprtxvznyeiwgakmusqo"),
                notch_position: 0,
                position: 0,
            },
            Rotor {
                wiring: utils::to_u8_array_26("ajdksiruxblhwtmcqgznpyfvoe"),
                notch_position: 8,
                position: 0,
            },
            Rotor {
                wiring: utils::to_u8_array_26("ekmflgdqvzntowyhxuspaibrcj"),
                notch_position: 8,
                position: 0,
            },
        ]);

        for _ in 0..26 {
            rotors.rotate();
        }

        assert_eq!(rotors.0[2].position, 26);
        assert_eq!(rotors.0[1].position, 2);
        assert_eq!(rotors.0[0].position, 1);
    }

    #[test]
    fn test_rotate_and_output() {
        let mut rotors = RotorAssembly::new([
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
        ]);

        let l1 = rotors.0[2].get_output('g' as u8);
        let l2 = rotors.0[1].get_output(l1);
        let l3 = rotors.0[0].get_output(l2);

        assert_eq!(l1, 'c' as u8);
        assert_eq!(l2, 'd' as u8);
        assert_eq!(l3, 'f' as u8);

        rotors.rotate();

        let l1 = rotors.0[2].get_output('g' as u8);
        let l2 = rotors.0[1].get_output(l1);
        let l3 = rotors.0[0].get_output(l2);

        assert_eq!(l1, 'p' as u8);
        assert_eq!(l2, 'c' as u8);
        assert_eq!(l3, 'm' as u8);

        rotors.rotate();

        let l1 = rotors.0[2].get_output('g' as u8);
        let l2 = rotors.0[1].get_output(l1);
        let l3 = rotors.0[0].get_output(l2);

        assert_eq!(l1, 'r' as u8);
        assert_eq!(l2, 'g' as u8);
        assert_eq!(l3, 'd' as u8);
    }

    #[test]
    fn test_reflector() {
        let reflector = Reflector {
            wiring: utils::to_u8_array_26("yruhqsldpxngokmiebfzcwvjat"),
        };

        assert_eq!(reflector.reflect('a' as u8), 'y' as u8);
        assert_eq!(reflector.reflect('y' as u8), 'a' as u8);

        assert_eq!(reflector.reflect('g' as u8), 'l' as u8);
        assert_eq!(reflector.reflect('l' as u8), 'g' as u8);

        assert_eq!(reflector.reflect('r' as u8), 'b' as u8);
        assert_eq!(reflector.reflect('b' as u8), 'r' as u8);
    }

    #[test]
    fn test_plugboard() {
        let plugboard = Plugboard::new("PO ML IU KJ NH YT GB VF RE DC");

        assert_eq!(plugboard.get_output('a' as u8), 'a' as u8);
        assert_eq!(plugboard.get_output('k' as u8), 'j' as u8);
        assert_eq!(plugboard.get_output('c' as u8), 'd' as u8);
    }
}
