// #![feature(nll)]

extern crate cryptopals;

use cryptopals::buffer::Buffer;
use cryptopals::hex::{FromHexString, ToHexString};
use cryptopals::base64::ToBase64;

use std::iter::repeat;

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

fn challenge1() -> bool {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let buf = Buffer::from_hex(hex_string).unwrap();
    let mut string = String::new();
    let _result = buf.write_base64(&mut string);
    let result_b64string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    println!("ans: {}", Buffer::from_base64(result_b64string));
    string == result_b64string
}

fn challenge2() -> bool {
    let buf1 = Buffer::from_hex("1c0111001f010100061a024b53535009181c").unwrap();
    let buf2 = Buffer::from_hex("686974207468652062756c6c277320657965").unwrap();
    let thing = buf1 ^ buf2;
    thing.to_hex().unwrap() == "746865206b696420646f6e277420706c6179"
}

fn challenge3() -> String {
    let xor_cypher = Buffer::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();
    let (_score, decrypted, _answer) = determine_character_cypher(&xor_cypher);
    decrypted
}

fn challenge4() -> String {
    let file = File::open("data/challenge4.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut best_score = 0.0;
    let mut best_decrypted = String::new();
    for line in lines {
        let (score, decrypted, _letter) = determine_character_cypher(&Buffer::from_hex(&line.unwrap()).unwrap());
        if score > best_score {
            best_score = score;
            best_decrypted = decrypted;
        }
    }
    best_decrypted
}

fn challenge5() -> bool {
    let buf1 = Buffer::from_string("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = "ICE";

    let xored = buf1.repeating_xor(&Buffer::from_string(key));

    let expected_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    xored.to_hex().unwrap() == expected_hex
}

fn challenge6() -> String {
    let the_string = std::fs::read_to_string("data/challenge6.txt").unwrap();
    // println!("{}", the_string);
    // let mut string = String::new();
    // Seems like base64 works fine
    // Buffer::from_string("hellohellohello").write_base64(&mut string);
    // println!("{}", Buffer::from_base64(&string));
    let base64_string = &"SW4gYSBnb3JnZSBvZiB0aGUgbW9vcnMsIG5vdCBmYXIgYXdheSBhcyB0aGUgY3JvdyBmbGllcyBm
cm9tIFBlbmRsZSBIaWxsLCBzdG9vZCBhIGdyaW0sIHJhbWJsaW5nIGhvdXNlIGtub3duIHRvIHRo
ZSBoZWF0aC1tZW4gYXMgV2luZHlob3VnaC4gSXQgaGFkIGJlZW4gZm9ydGlmaWVkIG9uY2U7IGJ1
dCBhZnRlcndhcmRzLCBpbiB0aW1lcyBvZiBlYXNlLCBzdWNjZXNzaXZlIG93bmVycyBoYWQgdGhv
dWdodCBtb3JlIG9mIGRpY2UgYW5kIGh1bnRpbmcgdGhhbiBvZiB3YXJmYXJlLCBhbmQgd2l0aGlu
LWRvb3JzIHRoZSBob3VzZSB3YXMgZnVybmlzaGVkIHdpdGggYSBjb21mb3J0IHRoYXQgYmVsaWVk
IGl0cyBsb29waG9sZWQgd2FsbHMuCiAgICBJdCBzdG9vZCBpbiB0aGUgY291bnR5IG9mIExhbmNh
c3RlciwgZmFtZWQgZm9yIGl0cyBsb3lhbHR5IGFuZCBmb3IgdGhlIGJlYXV0eSBvZiBpdHMgd29t
ZW50d28gcXVhbGl0aWVzIHRoYXQgb2Z0ZW4gcnVuIHRvZ2V0aGVyYW5kIHRoZXJlIGhhZCBiZWVu
IFJveWRzIGF0IFdpbmR5aG91Z2ggc2luY2UgTm9ybWFuIFdpbGxpYW0gZmlyc3QgcGFyY2VsbGVk
IG91dCB0aGUgQ291bnR5IFBhbGF0aW5lIGFtb25nIHRoZSBzdHJvbmcgbWVuIG9mIGhpcyBmb2xs
b3dpbmcuIFRoZSBSb3lkIHByaWRlIGhhZCBiZWVuIGRlZXAgZW5vdWdoLCB5ZXQgY2hpdmFscm91
cyBhbmQgd2FybS1oZWFydGVkLCBhcyBvZiBtZW4gd2hvc2UgaGlzdG9yeSBpcyBhbiBvcGVuIGJv
b2ssIG5vdCBmZWFyaW5nIHNjcnV0aW55IGJ1dCBhc2tpbmcgaXQuCiAgICBUaGUgaGVpciBvZiBp
dCBhbGxob3VzZSwgYW5kIG5hbWUsIGFuZCBsdXN0eSBwcmlkZWNhbWUgc3dpbmdpbmcgb3ZlciB0
aGUgbW9vci1jcmVzdCB0aGF0IGdhdmUgaGltIGEgc2lnaHQgb2YgV2luZHlob3VnaCwgbHlpbmcg
ZmFyIGJlbG93IGluIHRoZSBoYXplIG9mIHRoZSBOb3ZlbWJlciBhZnRlcm5vb24uIEl0IHdhcyBu
b3QgUnVwZXJ0cyBmYXVsdCB0aGF0IGhlIHdhcyB0aGUgaGVpciwgYW5kIGxlc3Mgc3Ryb25nIG9m
IGJvZHkgdGhhbiBvdGhlcnMgb2YgaGlzIHJhY2UuIEl0IHdhcyBub3QgaGlzIGZhdWx0IHRoYXQg
TGFkeSBSb3lkLCBoaXMgbW90aGVyLCBoYWQgZGVzcGlzZWQgaGltIGZyb20gaW5mYW5jeSwgYmVj
YXVzZSBoZSBicm9rZSB0aGUgdHJhZGl0aW9uIG9mIGhpcyBob3VzZSB0aGF0IGFsbCBpdHMgc29u
cyBtdXN0IG5lZWRzIGJlIHN0cm9uZyBhbmQgZ29vZCB0byBsb29rIGF0LgogICAgVGhlIGhlaXIg
c3Rvb2Qgb24gdGhlIHdpbmR5IHN1bW1pdCwgaGlzIGd1biB1bmRlciBoaXMgYXJtLCBhbmQgbG9v
a2VkIG92ZXIgdGhlIHJvbGxpbmcsIG5ldmVyLWVuZGluZyBzd2VlcCBvZiBoaWxscy4gVGhlIHN1
biwgYmlnIGFuZCBydWRkeSwgd2FzIGRpcHBpbmcgb3ZlciBQZW5kbGVzIHJvdW5kZWQgc2xvcGUs
IGFuZCBhbGwgdGhlIGhvbGxvd3MgaW4gYmV0d2VlbiB3ZXJlIGx1bWlub3VzIGFuZCBzdGlsbC4g
SGUgZm9yZ290IGhpcyBsb25lbGluZXNzZm9yZ290IHRoYXQgaGUgY291bGQgbm90IHNpdCBhIGhv
cnNlIHdpdGggZWFzZSBvciBwbGVhc3VyZSB0byBoaW1zZWxmOyBmb3Jnb3QgdGhhdCBoZSB3YXMg
c2h5IG9mIGhpcyBlcXVhbHMsIHNoeSBvZiB0aGUgY291bnRyeS1mb2xrIHdobyBtZXQgaGltIG9u
IHRoZSByb2FkLFsyXSB0aGF0IGhpcyBvbmUgcmVzcGl0ZSBmcm9tIHRoZSBidXJkZW4gb2YgdGhl
IGRheSB3YXMgdG8gZ2V0IHVwIGludG8gdGhlIGhpbGxzIHdoaWNoIEdvZCBoYWQgc2V0IHRoZXJl
IGZvciBhIHNhbmN0dWFyeS4KICAgIFZlcnkgc3RpbGwsIGFuZCBzdHJhaWdodCB0byBoaXMgZnVs
bCBoZWlnaHQsIHRoaXMgbWFuIG9mIGZpdmUtYW5kLXR3ZW50eSBzdG9vZCB3YXRjaGluZyB0aGUg
cGFnZWFudCBvZiB0aGUgc3VucyBkb3duLWdvaW5nLiBJdCB3YXMgaG9tZSBhbmQgbGliZXJ0eSB0
byBoaW0sIHRoaXMgcm91Z2ggbGFuZCB3aGVyZSBhbGwgd2FzIHBlYXQgYW5kIGhlYXRoZXIsIGFu
ZCB0aGUgcnVubmluZyBjcnkgb2Ygc3RyZWFtcyBhZnJhaWQgb2YgbG9uZWxpbmVzcywgYW5kIG92
ZXJoZWFkIHRoZSBzbm93LWNsb3VkcyB0aHJ1c3RpbmcgZm9yd2FyZCBmcm9tIHRoZSBlYXN0IGFj
cm9zcyB0aGUgd2VzdGVybiBzcGxlbmRvdXIgb2YgYmx1ZSwgYW5kIHJlZCwgYW5kIHNhcHBoaXJl
LgogICAgSGUgc2hpdmVyZWQgc3VkZGVubHkuIEFzIG9mIG9sZCwgaGlzIHNvdWwgd2FzIGJpZ2dl
ciB0aGFuIHRoZSBzdHJlbmd0aCBvZiBoaXMgbGVhbiBib2R5LCBhbmQgaGUgbG9va2VkIGRvd24g
YXQgV2luZHlob3VnaCB3aXRoIG1pc2dpdmluZywgZm9yIGhlIHdhcyBzcGVudCB3aXRoIGh1bmdl
ciBhbmQgbG9uZyB3YWxraW5nIG92ZXIgdGhlIGhpbGxzIGhlIGxvdmVkLiBIZSB0aG91Z2h0IG9m
IGhpcyBmYXRoZXIsIGtpbmQgYWx3YXlzIGFuZCB0b2xlcmFudCBvZiBoaXMgaGVpcnMgaW5maXJt
aXRpZXM7IG9mIGhpcyBtb3RoZXIsIGNvbGRlciB0aGFuIHdpbnRlciBvbiB0aGUgaGlsbHM7IG9m
IE1hdXJpY2UsIGhpcyB5b3VuZ2VyIGJyb3RoZXIgYnkgdGhyZWUgeWVhcnMsIHdobyBjb3VsZCBy
aWRlIHdlbGwsIGNvdWxkIHNob3cgcHJvd2VzcyBpbiBmaWVsZC1zcG9ydHMsIGFuZCBpbiBhbGwg
dGhpbmdzIGNhcnJ5IGhpbXNlbGYgbGlrZSB0aGUgdHJ1ZSBoZWlyIG9mIFdpbmR5aG91Z2guCiAg
ICBBIHF1aWNrLCB1bnJlYXNvbmluZyBoYXRyZWQgb2YgTWF1cmljZSB0b29rIGhpbSB1bmF3YXJl
c0VzYXVzIGhhdGUgZm9yIHRoZSBzdXBwbGFudGVyLiBIZSByZW1lbWJlcmVkIHRoYXQgTWF1cmlj
ZSBoYWQgbmV2ZXIga25vd24gdGhlIGZlYXJzIHRoYXQgYm9kaWx5IHdlYWtuZXNzIGJyaW5ncy4g
SW4gbnVyc2VyeSBkYXlzIGhlIGhhZCBiZWVuIHRoZSBsZWFkZXIsIGNsYWltaW5nIHRoZSB0b3lz
IGhlIGNvdmV0ZWQ7IGluIGJveWhvb2QgaGUgaGFkIGJlZW4gdGhlIGZyaWVuZCBhbmQgaW50aW1h
dGUgb2Ygb2xkZXIgbWVuLCB3aG8gbGF1Z2hlZCBhdCBoaXMgc3RyYWlnaHRmb3J3YXJkIGZlYXJs
ZXNzbmVzcywgYW5kIHRvbGQgZWFjaCBvdGhlciwgd2hpbGUgdGhlIGhlaXIgc3Rvb2QgYnkgYW5k
IGxpc3RlbmVkLCB0aGF0IE1hdXJpY2Ugd2FzIGEgcHVwIG9mIHRoZSBvbGQgYnJlZWQuCiAgICBU
aGVyZSB3YXMgY29tZm9ydCBibG93aW5nIGRvd24gdGhlIHdpbmQgdG8gUnVwZXJ0LCBoYWQgaGUg
Z3Vlc3NlZCBpdC4gVGhlIG1vb3IgbG92ZXMgaGVyIG93biwgYXMgaHVtYW4gbW90aGVycyBkbywg
YW5kIGluIGhlciB3aW50ZXItdGltZSBzaGUgbWVhbnQgdG8gcHJvdmUgaGltLiBIZSBkaWQgbm90
IGd1ZXNzIGFzIG11Y2gsIGFzIGhlIGxvb2tlZCBkb3duIG9uIHRoZSBodWRkbGVkIGNoaW1uZXkt
c3RhY2tzIG9mIFdpbmR5aG91Z2gsIGFuZCBzYXcgdGhlIGdyZXkgc21va2UgZmx5aW5nIHdpZGUg
YWJvdmUgdGhlIGdhYmxlcy4gSGlzIGhlYXJ0IHdhcyB0aGVyZSwgZG93biB5b25kZXIgd2hlcmUg
dGhlIG9sZCBob3VzZSBsYXVnaGVkIHNseWx5IHRvIGtub3cgdGhhdCBoZSB3YXMgaGVpciB0byBp
dCwgaW5zdGVhZCBvZiBNYXVyaWNlLiBJZiBvbmx5IGhlIGNvdWxkIHRha2UgaGlzIGZ1bGwgc2hh
cmUgaW4gZmllbGQtc3BvcnRzLCBhbmQgbWVldCBoaXMgZmVsbG93cyB3aXRoIHRoZSBmcmFuayBs
YXVnaCBvZiBjb21yYWRlc2hpcGlmIGhlIGhhZCBiZWVuIGxlc3Mgc2Vuc2l0aXZlIHRvIHJpZGlj
dWxlLCB0byB0aGVbM10gc2VsZi1kaXN0cnVzdCBpbmJyZWQgaW4gaGltIGJ5IExhZHkgUm95ZHMg
ZGlzZGFpbmhpcyB3b3JsZCBtaWdodCBoYXZlIHdvcm4gYSBkaWZmZXJlbnQgZmFjZSB0by1kYXku
IEhlIHN0b29wZWQgdG8gcGF0IHRoZSBzZXR0ZXIgdGhhdCBoYWQgc2hhcmVkIGEgZGF5cyBwb29y
IHNwb3J0IHdpdGggaGltLCBhbmQgdGhlbiBhZ2FpbiBoaXMgdGhvdWdodHMgd2VudCByb3Zpbmcg
ZG93biB0aGUgeWVhcnMuCiAgICBIZSBkaWQgbm90IGhlYXIgdGhlIHNvdW5kIG9mIGhvb2ZzIGJl
aGluZCBoaW0sIHRpbGwgUm9nZXIgRGVtYWluZXMgZGF1Z2h0ZXIgcm9kZSBjbG9zZSB1cCwgcmVp
bmVkIGluLCBhbmQgc2F0IHJlZ2FyZGluZyBoaW0gd2l0aCBhbiBvZGQgbG9vayBvZiBwaXR5LCBh
bmQgbGlraW5nLCBhbmQgcmVwcm9hY2gu";
    let b64bufbuf = Buffer::from_base64(&base64_string);
    println!("{}", b64bufbuf);
    // println!("yo{}", b64bufbuf);
    // let string = String::from_str(&"In a gorge of the moors, not far away as the crow flies from Pendle Hill, stood a grim, rambling house known to the heath-men as Windyhough. It had been fortified once; but afterwards, in times of ease, successive owners had thought more of dice and hunting than of warfare, and within-doors the house was furnished with a comfort that belied its loopholed walls.
    // It stood in the county of Lancaster, famed for its loyalty and for the beauty of its women—two qualities that often run together—and there had been Royds at Windyhough since Norman William first parcelled out the County Palatine among the strong men of his following. The Royd pride had been deep enough, yet chivalrous and warm-hearted, as of men whose history is an open book, not fearing scrutiny but asking it.
    // The heir of it all—house, and name, and lusty pride—came swinging over the moor-crest that gave him a sight of Windyhough, lying far below in the haze of the November afternoon. It was not Rupert’s fault that he was the heir, and less strong of body than others of his race. It was not his fault that Lady Royd, his mother, had despised him from infancy, because he broke the tradition of his house that all its sons must needs be strong and good to look at.
    // The heir stood on the windy summit, his gun under his arm, and looked over the rolling, never-ending sweep of hills. The sun, big and ruddy, was dipping over Pendle’s rounded slope, and all the hollows in between were luminous and still. He forgot his loneliness—forgot that he could not sit a horse with ease or pleasure to himself; forgot that he was shy of his equals, shy of the country-folk who met him on the road,[2] that his one respite from the burden of the day was to get up into the hills which God had set there for a sanctuary.
    // Very still, and straight to his full height, this man of five-and-twenty stood watching the pageant of the sun’s down-going. It was home and liberty to him, this rough land where all was peat and heather, and the running cry of streams afraid of loneliness, and overhead the snow-clouds thrusting forward from the east across the western splendour of blue, and red, and sapphire.
    // He shivered suddenly. As of old, his soul was bigger than the strength of his lean body, and he looked down at Windyhough with misgiving, for he was spent with hunger and long walking over the hills he loved. He thought of his father, kind always and tolerant of his heir’s infirmities; of his mother, colder than winter on the hills; of Maurice, his younger brother by three years, who could ride well, could show prowess in field-sports, and in all things carry himself like the true heir of Windyhough.
    // A quick, unreasoning hatred of Maurice took him unawares—Esau’s hate for the supplanter. He remembered that Maurice had never known the fears that bodily weakness brings. In nursery days he had been the leader, claiming the toys he coveted; in boyhood he had been the friend and intimate of older men, who laughed at his straightforward fearlessness, and told each other, while the heir stood by and listened, that Maurice was a pup of the old breed.
    // There was comfort blowing down the wind to Rupert, had he guessed it. The moor loves her own, as human mothers do, and in her winter-time she meant to prove him. He did not guess as much, as he looked down on the huddled chimney-stacks of Windyhough, and saw the grey smoke flying wide above the gables. His heart was there, down yonder where the old house laughed slyly to know that he was heir to it, instead of Maurice. If only he could take his full share in field-sports, and meet his fellows with the frank laugh of comradeship—if he had been less sensitive to ridicule, to the[3] self-distrust inbred in him by Lady Royd’s disdain—his world might have worn a different face to-day. He stooped to pat the setter that had shared a day’s poor sport with him, and then again his thoughts went roving down the years.
    // He did not hear the sound of hoofs behind him, till Roger Demaine’s daughter rode close up, reined in, and sat regarding him with an odd look of pity, and liking, and reproach.").unwrap();
    println!("{}", the_string.len());
    // let blahbuf = Buffer::from_string(&the_string);
    // println!("{}, {}", blahbuf, blahbuf.len());
    let encryptedbuf_blah = b64bufbuf.repeating_xor(&Buffer::from_string("aBcCeFgHiJkLmNoPqRsTuVwXyZAb"));
    let encryptedbuf_string = Buffer::from_base64(&the_string);
    println!("{}", encryptedbuf_string.repeating_xor(&Buffer::from_string("Terminator X: Bring the Noise")));
    let mut new_string = String::new();
    encryptedbuf_string.write_base64(&mut new_string);
    println!("cycle: {}\n\n", new_string);
    println!("encrypted: {}\n\n", &encryptedbuf_blah);
    println!("the_string: {}\n\n", &encryptedbuf_string);
    let encryptedbuf = encryptedbuf_string;
    let keysizes = guess_keysizes(&encryptedbuf);
    println!("{:?}", keysizes);
    // let keysize = keysizes[0].0;

    for (keysize, _) in keysizes[0..=3].iter() {
    // for keysize in (2..40) {
        let mut bufs = (0..*keysize).into_iter().map(|_| {
            Buffer(Vec::new())
        }).collect::<Vec<Buffer>>();

        let mut counter = 0;
        // println!("{:?}", encryptedbuf);
        for (ind, byte) in encryptedbuf.into_iter().enumerate() {
            // print!("{}", byte.to_string());
            bufs[counter].append(byte);
            counter = (counter + 1) % keysize;
        }

        let key = bufs.iter().map(|single_buf| {
            // println!("{}", buf);
            // println!("--------------------------------------------------\n\n");
            let (score, _decrypted, letter) = determine_character_cypher(&single_buf);
            // println!("Score: {}. Letter: {}: {}", score, letter, _decrypted);
            letter
        }).collect::<Buffer>();
        println!("-----------------\n{}, {}: \n{}\n------------------------", keysize, key, &encryptedbuf.repeating_xor(&key).to_string()[0..40]);
    }
    "hi".to_string()
}

fn brute_force_repeating_xor(encryptedbuf: &Buffer) -> String {
    let keysizes = guess_keysizes(&encryptedbuf);
    // println!("{:?}", keysizes);
    // let keysize = keysizes[0].0;

    let mut key = "".to_string();
    for (keysize, _) in keysizes[0..=3].iter() {
    // for keysize in (2..40) {
        let mut col_bufs = (0..*keysize).into_iter().map(|_| {
            Buffer(Vec::new())
        }).collect::<Vec<Buffer>>();

        let mut counter = 0;
        // println!("{:?}", encryptedbuf);
        for (ind, byte) in encryptedbuf.into_iter().enumerate() {
            // print!("{}", byte.to_string());
            col_bufs[counter].append(byte);
            counter = (counter + 1) % keysize;
        }

        let key = col_bufs.iter().map(|single_buf| {
            // println!("{}", buf);
            // println!("--------------------------------------------------\n\n");
            let (score, _decrypted, letter) = determine_character_cypher(&single_buf);
            // println!("Score: {}. Letter: {}: {}", score, letter, _decrypted);
            letter
        }).collect::<Buffer>();
        println!("-----------------\n{}, {}: \n{}\n------------------------", keysize, key, &encryptedbuf.repeating_xor(&key).to_string()[0..40]);
    }

    "hi".to_string()
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

fn determine_character_cypher(buf: &Buffer) -> (f64, String, u8) {
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