// #![feature(nll)]

extern crate cryptopals;

use cryptopals::buffer::Buffer;
use cryptopals::hex::{FromHexString, ToHexString};
use cryptopals::base64::ToBase64;

use std::iter::repeat;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    // Challenge1
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let buf = Buffer::from_hex(hex_string);
    let mut string = String::new();
    let _result = buf.unwrap().write_base64(&mut string);
    let result_b64string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(string, result_b64string);

    // Challenge2
    let buf1 = Buffer::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let buf2 = Buffer::from_hex("686974207468652062756c6c277320657965").unwrap();
    let thing = buf1 ^ buf2;
    assert_eq!(thing.to_hex().unwrap(), "746865206b696420646f6e277420706c6179");

    // Challenge3
    let xor_cypher = Buffer::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    println!("{:?}", xor_cypher);
    let (score, decrypted, answer) = determine_character_cypher(&xor_cypher);
    println!("Answer: {}\nDecrypted: {}", answer, decrypted);

    // Challenge4
    let file = File::open("data/challenge4.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut best_score = 0.0;
    let mut best_decrypted = String::new();
    for line in lines {
        let (score, decrypted, letter) = determine_character_cypher(&Buffer::from_hex(&line.unwrap()).unwrap());
        if score > best_score {
            best_score = score;
            best_decrypted = decrypted;
            // println!("{}", letter);
        }
    }
    println!("Answer: {}", best_decrypted);

    // Done
    println!("{}", "Done")
}

fn determine_character_cypher(buf: &Buffer) -> (f64, String, char) {
    let chars = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
    let buflen = buf.len();
    let mut max_score = 0.0;
    let mut character = 'a';
    let mut max_buffer = Buffer(Vec::new());
    for cha in chars.chars() {
        let char_buf = &Buffer::from_string(&repeat(cha).take(buflen).collect::<String>());
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
fn score_buffer(buf: &Buffer) -> f64 {
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
        .fold(0, |mut acc, is_low| {
            if is_low {
                acc = acc + 1
            }
            acc
        });
    if num_lower * 2 > length {
        return score
    } else {
        return 0.0
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