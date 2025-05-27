use crate::{
    algorithms::enigma::{
        alg::Enigma, plugboard::Plugboard, reflector::Reflector, rotor::Rotor,
        rotor_assembly::RotorAssembly, utils,
    },
    Algorithm,
};

#[test]
fn test_alg() {
    let input: Vec<u8> = "Hello asdjfk df asdf asd"
        .chars()
        .map(|c| c.to_ascii_lowercase() as u8)
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    println!("{}", String::from_utf8(input.clone()).unwrap());

    let enigma = Enigma {};
    let encrypted = enigma
        .encrypt(&input, String::from("ekmflgdqvzntowyhxuspaibrcj 8 0 ajdksiruxblhwtmcqgznpyfvoe 8 0 bdfhjlcprtxvznyeiwgakmusqo 0 0 yruhqsldpxngokmiebfzcwvjat PO ML IU KJ NH YT GB VF RE DC"))
        .unwrap();

    let decrypted = enigma.encrypt(&encrypted, String::from("ekmflgdqvzntowyhxuspaibrcj 8 0 ajdksiruxblhwtmcqgznpyfvoe 8 0 bdfhjlcprtxvznyeiwgakmusqo 0 0 yruhqsldpxngokmiebfzcwvjat PO ML IU KJ NH YT GB VF RE DC")).unwrap();

    assert!(decrypted.eq(&input));

    // println!("{}", String::from_utf8(encrypted.clone()).unwrap());
    // let res = String::from_utf8(decrypted).unwrap();
    // println!("{} {}", res, input);
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

#[test]
fn test_rotating() {
    let mut rotors = [
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
    ];

    rotors.rotate();
    assert_eq!(rotors[2].position, 1);
    assert_eq!(rotors[1].position, 0);
    assert_eq!(rotors[0].position, 0);

    rotors.rotate();
    assert_eq!(rotors[2].position, 2);
    assert_eq!(rotors[1].position, 1);
    assert_eq!(rotors[0].position, 0);

    rotors.rotate();
    assert_eq!(rotors[2].position, 3);
    assert_eq!(rotors[1].position, 2);
    assert_eq!(rotors[0].position, 1);
}

#[test]
fn test_rotating_intense() {
    let mut rotors = [
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
    ];

    for _ in 0..26 {
        rotors.rotate();
    }

    assert_eq!(rotors[2].position, 26);
    assert_eq!(rotors[1].position, 2);
    assert_eq!(rotors[0].position, 1);
}

#[test]
fn test_rotate_and_output() {
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

    let l1 = rotors[2].get_output('g' as u8);
    let l2 = rotors[1].get_output(l1);
    let l3 = rotors[0].get_output(l2);

    assert_eq!(l1, 'c' as u8);
    assert_eq!(l2, 'd' as u8);
    assert_eq!(l3, 'f' as u8);

    rotors.rotate();

    let l1 = rotors[2].get_output('g' as u8);
    let l2 = rotors[1].get_output(l1);
    let l3 = rotors[0].get_output(l2);

    assert_eq!(l1, 'p' as u8);
    assert_eq!(l2, 'c' as u8);
    assert_eq!(l3, 'm' as u8);

    rotors.rotate();

    let l1 = rotors[2].get_output('g' as u8);
    let l2 = rotors[1].get_output(l1);
    let l3 = rotors[0].get_output(l2);

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
