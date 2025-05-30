use super::s_boxes::{T1, T2, T3, T4};

type State = [u64; 3];
const S0: State = [
    0x0123_4567_89AB_CDEF,
    0xFEDC_BA98_7654_3210,
    0xF096_A5B4_C3B2_E187,
];

pub fn hash_data(data: &[u8]) -> Vec<u8> {
    let mut s = S0;

    let bit_len = ((data.len() as u64) * 8).to_le_bytes();
    let zero = 0u8;
    let one = 1u8;

    for chunk in data.chunks(64) {
        match chunk.len() {
            0..=55 => {
                let padded_chunk: Vec<u8> = chunk
                    .into_iter()
                    .copied()
                    .chain(std::iter::once(one))
                    .chain(std::iter::repeat(zero))
                    .take(56)
                    .chain(bit_len.into_iter())
                    .collect();

                s = compress(s, &padded_chunk.try_into().unwrap());
            }
            56..64 => {
                let padded_chunk: Vec<u8> = chunk
                    .into_iter()
                    .copied()
                    .chain(std::iter::once(one))
                    .chain(std::iter::repeat(zero))
                    .take(64)
                    .collect();

                s = compress(s, &padded_chunk.try_into().unwrap());

                let padded_length: Vec<u8> = std::iter::repeat_n(zero, 56)
                    .chain(bit_len.into_iter())
                    .collect();

                s = compress(s, &padded_length.try_into().unwrap());
            }
            64 => {
                s = compress(s, chunk.try_into().unwrap());
            }
            _ => {
                panic!()
            }
        }
    }

    if data.len() % 64 == 0 {
        let padded_chunk: Vec<u8> = std::iter::once(one)
            .chain(std::iter::repeat_n(zero, 55))
            .chain(bit_len.into_iter())
            .collect();

        s = compress(s, &padded_chunk.try_into().unwrap());
    }

    return s.iter().flat_map(|&n| n.to_le_bytes()).collect();
}

fn compress(mut state: State, raw_block: &[u8; 64]) -> State {
    let mut block: [u64; 8] = Default::default();

    // [u8; 64] u [u64; 8]
    for (i, chunk) in raw_block.chunks_exact(8).enumerate() {
        block[i] = u64::from_le_bytes(chunk.try_into().unwrap());
    }

    let [mut a, mut b, mut c] = state;

    pass(&mut a, &mut b, &mut c, &block, 5);
    key_schedule(&mut block);
    pass(&mut c, &mut a, &mut b, &block, 7);
    key_schedule(&mut block);
    pass(&mut b, &mut c, &mut a, &block, 9);

    state[0] ^= a;
    state[1] = b.wrapping_sub(state[1]);
    state[2] = c.wrapping_add(state[2]);

    state
}

fn round(a: &mut u64, b: &mut u64, c: &mut u64, x: &u64, mul: u8) {
    *c ^= *x;

    let c2: [u8; 8] = c.to_le_bytes();

    let a2 = T1[usize::from(c2[0])]
        ^ T2[usize::from(c2[2])]
        ^ T3[usize::from(c2[4])]
        ^ T4[usize::from(c2[6])];

    let b2 = T4[usize::from(c2[1])]
        ^ T3[usize::from(c2[3])]
        ^ T2[usize::from(c2[5])]
        ^ T1[usize::from(c2[7])];

    *a = a.wrapping_sub(a2);
    *b = b.wrapping_add(b2).wrapping_mul(u64::from(mul));
}

fn pass(a: &mut u64, b: &mut u64, c: &mut u64, x: &[u64; 8], mul: u8) {
    round(a, b, c, &x[0], mul);
    round(b, c, a, &x[1], mul);
    round(c, a, b, &x[2], mul);
    round(a, b, c, &x[3], mul);
    round(b, c, a, &x[4], mul);
    round(c, a, b, &x[5], mul);
    round(a, b, c, &x[6], mul);
    round(b, c, a, &x[7], mul);
}

fn key_schedule(x: &mut [u64; 8]) {
    x[0] = x[0].wrapping_sub(x[7] ^ 0xA5A5_A5A5_A5A5_A5A5);
    x[1] ^= x[0];
    x[2] = x[2].wrapping_add(x[1]);
    x[3] = x[3].wrapping_sub(x[2] ^ ((!x[1]) << 19));
    x[4] ^= x[3];
    x[5] = x[5].wrapping_add(x[4]);
    x[6] = x[6].wrapping_sub(x[5] ^ ((!x[4]) >> 23));
    x[7] ^= x[6];
    x[0] = x[0].wrapping_add(x[7]);
    x[1] = x[1].wrapping_sub(x[0] ^ ((!x[7]) << 19));
    x[2] ^= x[1];
    x[3] = x[3].wrapping_add(x[2]);
    x[4] = x[4].wrapping_sub(x[3] ^ ((!x[2]) >> 23));
    x[5] ^= x[4];
    x[6] = x[6].wrapping_add(x[5]);
    x[7] = x[7].wrapping_sub(x[6] ^ 0x0123_4567_89AB_CDEF);
}
