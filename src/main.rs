// #![feature(nll)]

extern crate cryptopals;
extern crate base64;

use cryptopals::buffer::Buffer;
use cryptopals::hex::{FromHexString, ToHexString};
use cryptopals::utils::{determine_character_cypher, brute_force_repeating_xor};

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    // Challenge1
    if challenge1() { println!("Challenge1 success") } else { println!("Challenge1 fail") };

    // Challenge2
    if challenge2() { println!("Challenge2 success") } else { println!("Challenge2 fail") };

    // Challenge3
    if let ans3 = challenge3() { println!("3 success: {}", ans3) } else { println!("Challenge3 fail") };

    // Challenge4
    if let ans4 = challenge4() { println!("4 success: {}", ans4) } else { println!("Challenge4 fail") };

    // Challenge5
    if challenge5() { println!("Challenge5 success") } else { println!("Challenge5 fail") };

    // Challenge6
    if let ans6 = challenge6() { println!("6 success: {}", ans6) } else { println!("Challenge6 fail") };


    // Done
    print!("{}", "Done")
}

/// Convert hex to b64
fn challenge1() -> bool {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let buf = Buffer::from_hex(hex_string).unwrap();
    let result_b64string = buf.to_base64();
    let expected_b64string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    println!("ans: {}", result_b64string);
    expected_b64string == result_b64string
}

/// Xor 2 buffers
fn challenge2() -> bool {
    let buf1 = Buffer::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let buf2 = Buffer::from_hex("686974207468652062756c6c277320657965").unwrap();
    let thing = buf1 ^ buf2;
    thing.to_hex().unwrap() == "746865206b696420646f6e277420706c6179"
}

/// Break single-byte xor
fn challenge3() -> String {
    let xor_cypher = Buffer::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let (_score, decrypted, _answer) = determine_character_cypher(&xor_cypher);
    decrypted
}

/// Find single-byte xor
fn challenge4() -> String {
    let file = BufReader::new(File::open("data/challenge4.txt").unwrap());
    let lines = file.lines();
    let (_best_score, best_decrypted): (f64, String) = lines.into_iter().fold(
        (0., "".to_string()), | (best_score, best_decrypted), line | {
            let (score, decrypted, _letter) = determine_character_cypher(
                &Buffer::from_hex(&line.unwrap()).unwrap()
            );
            if score > best_score {
                (score, decrypted)
            } else {
                (best_score, best_decrypted)
            }
        }
    );
    best_decrypted
}
/// Implement repeating-key xor
fn challenge5() -> bool {
    let buf1 = Buffer::from(&"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = "ICE";
    let key_buffer = &Buffer::from(&key);

    let xored = buf1.repeating_xor(key_buffer);
    let xoredx2 = xored.repeating_xor(key_buffer);
    println!("{}", xoredx2);
    let expected_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    xored.to_hex().unwrap() == expected_hex
}

/// Break repeating-key xor
fn challenge6() -> String {
    let encrypted_buf = Buffer::from_base64(
        &std::fs::read_to_string("data/challenge6.txt").unwrap().replace("\n", "")
    );

    let (key, decrypted_buf) = brute_force_repeating_xor(&encrypted_buf);
    println!("Key: {}.", key);
    decrypted_buf.to_string()
}
