use super::buffer::Buffer;

use std::iter::repeat;

pub fn brute_force_repeating_xor(encrypted_buf: &Buffer) -> (Buffer, Buffer) {
    let keysizes = guess_keysizes(encrypted_buf);

    let mut results = keysizes[0..=3].iter().map(|(keysize, _)| {
        let col_bufs = make_col_bufs(encrypted_buf, *keysize as u64);
        let key = Buffer::from(col_bufs.iter().map(|single_buf| {
            let (_score, _decrypted, letter) = determine_character_cypher(&single_buf);
            letter
        }).collect::<Vec<u8>>());
        let decrypted = encrypted_buf.repeating_xor(&key);
        (keysize, key, decrypted)
    }).collect::<Vec<_>>();

    let (_, r_key, r_decrypted) = results.remove(0);
    (r_key, r_decrypted)
}

fn make_col_bufs(buf: &Buffer, keysize: u64) -> Vec<Buffer> {
    let mut col_bufs = (0..keysize).into_iter().map(|_| {
        Buffer(Vec::new())
    }).collect::<Vec<Buffer>>();

    let mut counter: usize = 0;
    for (ind, byte) in buf.into_iter().enumerate() {
        col_bufs[counter].append(byte);
        counter = (counter + 1) % keysize as usize;
    };
    col_bufs
}

fn hamming_distance(buf1: &Buffer, buf2: &Buffer) -> u64 {
    let Buffer(vec1) = buf1;
    let Buffer(vec2) = buf2;
    vec1.iter().zip(vec2.iter()).fold(0, | acc, (first, second)| {
        acc + (bits(first ^ second) as u64)
    })
}

fn bits(num: u8) -> u8 {
    let mut counter = 0;
    let mut n = num;
    while n > 0 {
        let n_1 = n - 1;
        n = n_1 & n;
        counter += 1;
    }
    counter
}

fn guess_keysizes(buf: &Buffer) -> Vec<(usize, f64)> {
    let mut return_vec = Vec::new();
    (2..=40).into_iter().for_each( |keylen| {
        let num_buffers = buf.len() / keylen;
        let mut total_hamming = 0;
        for buf_num in 0..num_buffers/2 {
            let start = buf_num * keylen;
            let middle = start + keylen;
            let end = middle + keylen;
            let buf1 = Buffer(buf.0[start..middle].to_vec());
            let buf2 = Buffer(buf.0[middle..end].to_vec());
            total_hamming += hamming_distance(&buf1, &buf2);
        }

        return_vec.push((keylen, ((total_hamming as f64) / (num_buffers / 2) as f64) / keylen as f64));
    });
    return_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    return_vec
}

pub fn determine_character_cypher(buf: &Buffer) -> (f64, String, u8) {
    let buflen = buf.len();
    let mut max_score = 0.0;
    let mut character = 0;
    let mut max_buffer = Buffer(Vec::new());
    for byte in 0..256 {
        let cha = byte as u8;
        let char_buf = &Buffer(repeat(cha).take(buflen).collect::<Vec<u8>>());
        let xored = buf ^ char_buf;
        let score = score_buffer(&xored);
        if score > max_score {
            max_score = score;
            character = cha;
            max_buffer = xored;
        }
    }
    (max_score, max_buffer.to_string(), character)
}

/// Take in a buffer and give a score referencing how likely it is to be English
pub fn score_buffer(buf: &Buffer) -> f64 {
    let freq = buf
                .into_iter()
                .map(u8_to_index)
                .fold(vec![0;27], |mut acc, chr| {
                    acc[chr] = acc[chr] + 1;
                    acc
                });

    let length = buf.len();
    let score = freq.into_iter()
        .map(|freq| {
            (freq as f64) / length as f64
        }).zip(SCORES)
        .fold(1.0, |acc, (actual, expected)| {
            // Our scorer is "product of squares of (1 - absolute difference)"
            // This gives us the property that bigger deviations have outsized effect
            acc * (1.0 - (actual - expected).abs().powi(2))
        });

    // Deal with the problem that cap and lc will both give the same score by scoring 0
    // if there are more caps than lc letters
    let num_lower = &buf.into_iter()
        .map(is_lower)
        .fold(0, |acc, is_low| {
            if is_low {
                acc + 1
            } else {
                acc
            }
        });
    if num_lower * 2 > length {
        score
    } else {
        score - 0.0000001
    }
}

fn u8_to_index(num: u8) -> usize {
    match num {
        65..=90 => (num - 65) as usize,
        97..=122 => (num - 97) as usize,
        _ => 26
    }
}

fn is_lower(num: u8) -> bool {
    match num {
        97..=122 => true,
        _ => false,
    }
}

const SCORES: &[f64; 27] = &[
    0.0855, // a
    0.0160, // b
    0.0316, // c
    0.1210, // d
    0.0218, // e
    0.0209, // f
    0.0496, // g
    0.0496, // h
    0.0733, // i
    0.0022, // j
    0.0081, // k
    0.0421, // l
    0.0253, // m
    0.0717, // n
    0.0747, // o
    0.0207, // p
    0.0010, // q
    0.0633, // r
    0.0673, // s
    0.0894, // t
    0.0268, // u
    0.0106, // v
    0.0183, // w
    0.0019, // x
    0.0172, // y
    0.0011, // z
    0.0, // other
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_col_bufs() {
        let mut testbuf = Buffer::from("01234501234501234501234501234");
        let bufs = make_col_bufs(&testbuf, 6);
        bufs.iter().enumerate().for_each(|(ind, buf)| {
            if ind < 5 {
                let inds = repeat(testbuf.0.remove(0)).take(5).collect::<Vec<_>>();
                assert_eq!(buf.to_string(), Buffer(inds).to_string())
            } else {
                let inds = repeat(testbuf.0.remove(0)).take(4).collect::<Vec<_>>();
                assert_eq!(buf.to_string(), Buffer(inds).to_string());
            }
        });
    }
}