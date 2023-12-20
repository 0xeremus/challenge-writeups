use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

#[path = "utils.rs"]
mod utils;
use utils::{decrypt_cbc, pkcs_padding};

pub fn ch9() {
    if let Ok(_) = env::var("DESCRIBE") {
        println!("Set 2 - Challenge 9:
        Implement PKCS#7 padding

        A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext. 
        But we almost never want to transform a single block; we encrypt irregularly-sized messages.

        One way we account for irregularly-sized messages is by padding, 
        creating a plaintext that is an even multiple of the blocksize. 
        The most popular padding scheme is called PKCS#7.

        So: pad any block to a specific block length, 
        by appending the number of bytes of padding to the end of the block. 
        
        For instance,

        'YELLOW SUBMARINE'

        ... padded to 20 bytes would be:

        'YELLOW SUBMARINE\\x04\\x04\\x04\\x04'\n\n"
        );
    };

    let block_boundary = 20_usize;
    let input = b"YELLOW SUBMARINE".to_vec();
    let output = pkcs_padding(block_boundary, &input);

    assert_eq!(output, b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec(),);

    println!(
        "Input: {}\nBlock Boundry: {block_boundary}\nOutput: YELLOW SUBMARINE\\x04\\x04\\x04\\x04\n", 
        String::from_utf8(input).unwrap());
}

pub fn ch10() {
    if let Ok(_) = env::var("DESCRIBE") {
        println!("Set 2 - Challenge 10:
        Implement CBC mode

        CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, 
        despite the fact that a block cipher natively only transforms individual blocks.

        In CBC mode, each ciphertext block is added to the next plaintext block before the next call to the cipher core.

        The first plaintext block, which has no associated previous ciphertext block, is added to a 
        \"fake 0th ciphertext block\" called the initialization vector, or IV.

        Implement CBC mode by hand by taking the ECB function you wrote earlier, 
        making it encrypt instead of decrypt (verify this by decrypting whatever you 
        encrypt to test), and using your XOR function from the previous exercise to combine them.

        The file here is intelligible (somewhat) when CBC decrypted against \"YELLOW SUBMARINE\" 
        with an IV of all ASCII 0 (\\x00\\x00\\x00 &c) 
        ");
    };

    let key = b"YELLOW SUBMARINE";
    let iv = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    // open file and decode each line
    let ciphertext_file = match fs::File::open("./data/10.txt") {
        Ok(cor) => cor,
        Err(_) => panic!("Failed to read ./data/4.txt"),
    };
    let reader = BufReader::new(ciphertext_file);

    let mut ciphertext: Vec<u8> = Vec::new();
    for line in reader.lines() {
        let mut temp = base64::decode(line.unwrap()).unwrap();
        ciphertext.append(&mut temp);
    }

    // decrypt each block
    let chal_cont = decrypt_cbc(key, iv, &ciphertext);

    // display output
    println!(
        "Decrypted file:\n{}\n",
        String::from_utf8(chal_cont).unwrap()
    );
}
