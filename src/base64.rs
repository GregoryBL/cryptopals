use std::fmt;

pub trait ToBase64 {
    fn write_base64<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;
}

pub const BASE_64_TO_CHAR: &[u8; 64] = &[
    65, // input 0 (0x0) => 'A' (0x41)
    66, // input 1 (0x1) => 'B' (0x42)
    67, // input 2 (0x2) => 'C' (0x43)
    68, // input 3 (0x3) => 'D' (0x44)
    69, // input 4 (0x4) => 'E' (0x45)
    70, // input 5 (0x5) => 'F' (0x46)
    71, // input 6 (0x6) => 'G' (0x47)
    72, // input 7 (0x7) => 'H' (0x48)
    73, // input 8 (0x8) => 'I' (0x49)
    74, // input 9 (0x9) => 'J' (0x4A)
    75, // input 10 (0xA) => 'K' (0x4B)
    76, // input 11 (0xB) => 'L' (0x4C)
    77, // input 12 (0xC) => 'M' (0x4D)
    78, // input 13 (0xD) => 'N' (0x4E)
    79, // input 14 (0xE) => 'O' (0x4F)
    80, // input 15 (0xF) => 'P' (0x50)
    81, // input 16 (0x10) => 'Q' (0x51)
    82, // input 17 (0x11) => 'R' (0x52)
    83, // input 18 (0x12) => 'S' (0x53)
    84, // input 19 (0x13) => 'T' (0x54)
    85, // input 20 (0x14) => 'U' (0x55)
    86, // input 21 (0x15) => 'V' (0x56)
    87, // input 22 (0x16) => 'W' (0x57)
    88, // input 23 (0x17) => 'X' (0x58)
    89, // input 24 (0x18) => 'Y' (0x59)
    90, // input 25 (0x19) => 'Z' (0x5A)
    97, // input 26 (0x1A) => 'a' (0x61)
    98, // input 27 (0x1B) => 'b' (0x62)
    99, // input 28 (0x1C) => 'c' (0x63)
    100, // input 29 (0x1D) => 'd' (0x64)
    101, // input 30 (0x1E) => 'e' (0x65)
    102, // input 31 (0x1F) => 'f' (0x66)
    103, // input 32 (0x20) => 'g' (0x67)
    104, // input 33 (0x21) => 'h' (0x68)
    105, // input 34 (0x22) => 'i' (0x69)
    106, // input 35 (0x23) => 'j' (0x6A)
    107, // input 36 (0x24) => 'k' (0x6B)
    108, // input 37 (0x25) => 'l' (0x6C)
    109, // input 38 (0x26) => 'm' (0x6D)
    110, // input 39 (0x27) => 'n' (0x6E)
    111, // input 40 (0x28) => 'o' (0x6F)
    112, // input 41 (0x29) => 'p' (0x70)
    113, // input 42 (0x2A) => 'q' (0x71)
    114, // input 43 (0x2B) => 'r' (0x72)
    115, // input 44 (0x2C) => 's' (0x73)
    116, // input 45 (0x2D) => 't' (0x74)
    117, // input 46 (0x2E) => 'u' (0x75)
    118, // input 47 (0x2F) => 'v' (0x76)
    119, // input 48 (0x30) => 'w' (0x77)
    120, // input 49 (0x31) => 'x' (0x78)
    121, // input 50 (0x32) => 'y' (0x79)
    122, // input 51 (0x33) => 'z' (0x7A)
    48, // input 52 (0x34) => '0' (0x30)
    49, // input 53 (0x35) => '1' (0x31)
    50, // input 54 (0x36) => '2' (0x32)
    51, // input 55 (0x37) => '3' (0x33)
    52, // input 56 (0x38) => '4' (0x34)
    53, // input 57 (0x39) => '5' (0x35)
    54, // input 58 (0x3A) => '6' (0x36)
    55, // input 59 (0x3B) => '7' (0x37)
    56, // input 60 (0x3C) => '8' (0x38)
    57, // input 61 (0x3D) => '9' (0x39)
    43, // input 62 (0x3E) => '+' (0x2B)
    47, // input 63 (0x3F) => '/' (0x2F)
];

impl <T: AsRef<[u8]>> ToBase64 for T {
    fn write_base64<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        let chunks = self.as_ref().chunks(3);
        for triple in chunks {
            match triple {
                [first, second, third] => {
                    w.write_char(BASE_64_TO_CHAR[(first >> 2) as usize] as char)?;
                    w.write_char(BASE_64_TO_CHAR[(second >> 4 | first % 4 << 4) as usize] as char)?;
                    w.write_char(BASE_64_TO_CHAR[(third >> 6 | second % 16 << 2) as usize] as char)?;
                    w.write_char(BASE_64_TO_CHAR[(third % 64) as usize] as char)?;
                },
                [first, second] => {
                    w.write_char(BASE_64_TO_CHAR[(first >> 2) as usize] as char)?; 
                    w.write_char(BASE_64_TO_CHAR[(second >> 4 | first & 4 << 4) as usize] as char)?;
                    w.write_char(BASE_64_TO_CHAR[(second % 16) as usize] as char)?;
                    w.write_str("=")?;
                },
                [first] => { 
                    w.write_char(BASE_64_TO_CHAR[(first >> 2) as usize] as char)?; 
                    w.write_char(BASE_64_TO_CHAR[(first & 4 << 4) as usize] as char)?;
                    w.write_str("=")?; 
                    w.write_str("=")?; 
                },
                [] => { },
                _ => Err(std::fmt::Error)?,
            }
        };
        Ok(())
    }
}
