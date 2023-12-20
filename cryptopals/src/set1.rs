use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, env, fs};

use openssl::symm::{decrypt, Cipher};

#[path = "utils.rs"]
mod utils;
use utils::{compute_likely_keysize, transpose_key_blocks, xor_and_score_byte_array};


pub fn ch1() {
    println!("Set 1 - Challenge 1:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!("
        Convert hex to base64

        The string:

        49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d

        Should produce:

        SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

        So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
        Cryptopals Rule

        Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing.\n\n");
    };

    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

    let output = match hex::decode(input) {
        Ok(x) => base64::encode(x),
        Err(_) => panic!("Ruh roh"),
    };

    // Assert correctness from known output.
    assert_eq!(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        output
    );

    println!("Input: {input}\nOutput: {output}");
}

pub fn ch2() {
    println!("Set 1 - Challenge 2:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!(
            "
        Fixed XOR

        Write a function that takes two equal-length buffers and produces their XOR combination.

        If your function works properly, then when you feed it the string:

        1c0111001f010100061a024b53535009181c

        ... after hex decoding, and when XOR'd against:

        686974207468652062756c6c277320657965

        ... should produce:

        746865206b696420646f6e277420706c6179\n\n"
        );
    };

    let input_str1 = "1c0111001f010100061a024b53535009181c";
    let hex_str1 = hex::decode(input_str1).unwrap();

    let input_str2 = "686974207468652062756c6c277320657965";
    let hex_str2 = hex::decode(input_str2).unwrap();

    // for input string perform byte level xor
    let plain_str = hex_str1
        .iter()
        .zip(hex_str2.iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>();

    let output = hex::encode(&plain_str);

    // assert known output is achieved
    assert_eq!(output, "746865206b696420646f6e277420706c6179");

    println!("Input: {input_str1} ^ {input_str2}");
    println!("Output: {output}");
}

pub fn ch3() {
    println!("Set 1 - Challenge 3:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!(
        "Single-byte XOR cipher

        The hex encoded string:

        1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

        ... has been XOR'd against a single character. Find the key, decrypt the message.

        You can do this by hand. But don't: write code to do it for you.

        How? Devise some method for \"scoring\" a piece of English plaintext. Character frequency is a good metric. \nEvaluate each output and choose the one with the best score.\n");
    };

    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input_raw = hex::decode(input).unwrap();

    let (key, ans, highest) = xor_and_score_byte_array(input_raw);

    println!("Input: {input}");
    println!("Xor Key: {key} with score {highest}");
    println!("Answer: {}", String::from_utf8(ans).unwrap());
}

pub fn ch4() {
    println!("Set 1 Challenge 4:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!(
            "Detect single-character XOR

        One of the 60-character strings in this file has been encrypted by single-character XOR.

        Find it.

        (Your code from #3 should help.)"
        );
    };

    // open file and decode each line
    let candidates = match fs::File::open("./data/4.txt") {
        Ok(cor) => cor,
        Err(_) => panic!("Failed to read ./data/4.txt"),
    };

    let reader = BufReader::new(candidates);

    // map of line and to key, output, and score
    let mut scores: HashMap<usize, (u8, Vec<u8>, f64)> = HashMap::new();
    scores.reserve(64);

    // hex decode each line in file and score
    for (i, c) in reader.lines().enumerate() {
        let c = match hex::decode(c.unwrap()) {
            Ok(x) => x,
            Err(_) => break,
        };

        // score and save in map
        scores.insert(i, xor_and_score_byte_array(c));
    }

    // find highest score and display
    let mut line = 0_usize;
    let mut highest = 0.0;
    let mut xor_key = b'a';

    scores.iter().for_each(|(iline, (xkey, _res, score))| {
        if score > &highest {
            highest = *score;
            xor_key = *xkey;
            line = *iline;
        }
    });

    println!(
        "Result:\nLine: {line} with xor key {xor_key}\nAns: {}",
        String::from_utf8(scores.get(&line).unwrap().1.clone()).unwrap()
    );
}

pub fn ch5() {
    println!("Set 1 Challenge 5:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!(
        "Implement repeating-key XOR

        Here is the opening stanza of an important work of the English language:

        Burning 'em, if you ain't quick and nimble
        I go crazy when I hear a cymbal

        Encrypt it, under the key \"ICE\", using repeating-key XOR.

        In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.

        It should come out to:

        0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

        Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail.
        Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.
        ");
    };

    let key = "ICE";
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let input_raw = Vec::<u8>::from(input);

    // xor each keysize of input with key and collect in output vec
    let xor_vec = input_raw
        .chunks(3)
        .flat_map(|x| {
            x.iter()
                .zip(key.as_bytes())
                .map(|(x, y)| x ^ y)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    // hex encode for display and verification
    let output = hex::encode(xor_vec);

    // assert output is known output
    assert_eq!(output, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");

    println!("Input:\n{input} ^ {key} yields:\n{output}");
}

pub fn ch6() {
    println!("Set 1 Challenge 6:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!("
        Break repeating-key XOR
        It is officially on, now.

        This challenge isn't conceptually hard, but it involves actual error-prone coding. The other challenges in this set are there to bring you up to speed. This one is there to qualify you. If you can do this one, you're probably just fine up to Set 6.

        There's a file here. It's been base64'd after being encrypted with repeating-key XOR.

        Decrypt it.

        Here's how:

            Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
            Write a function to compute the edit distance/Hamming distance between two strings. The Hamming distance is just the number of differing bits. The distance between:

            this is a test

            and

            wokka wokka!!!

            is 37. Make sure your code agrees before you proceed.
            For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
            The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances.
            Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
            Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.
            Solve each block as if it was single-character XOR. You already have code to do this.
            For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.

        This code is going to turn out to be surprisingly useful later on. Breaking repeating-key XOR (\"Vigenere\") statistically is obviously an academic exercise, a \"Crypto 101\" thing. But more people \"know how\" to break it than can actually break it, and a similar technique breaks something much more important.
        No, that's not a mistake.

        We get more tech support questions for this challenge than any of the other ones. We promise, there aren't any blatant errors in this text. In particular: the \"wokka wokka!!!\" edit distance really is 37.");
    };

    // read in and base64 decode our challenge content
    let candidates = match fs::File::open("./data/6.txt") {
        Ok(cor) => cor,
        Err(_) => panic!("Failed to read ./data/6.txt"),
    };

    let reader = BufReader::new(candidates);
    let mut base64_content = String::new();

    for line in reader.lines() {
        let partial = match line {
            Ok(l) => match l.strip_suffix("\n") {
                Some(ls) => ls.to_string(),
                None => l,
            },
            Err(_) => break,
        };
        base64_content = base64_content + &partial;
    }
    let raw_content = base64::decode(base64_content).expect("Failed to decode base64 content");

    // compute our likely key size
    let keysize = compute_likely_keysize(&raw_content);

    // transpose blocks for key size.
    let transposed_blocks: HashMap<usize, Vec<u8>> = transpose_key_blocks(keysize, &raw_content);

    // find best scored single byte xor for each block
    let mut final_key = [0_u8; 29];
    for (key, transpose) in transposed_blocks {
        let (val, _res, _score) = xor_and_score_byte_array(transpose);
        final_key[key] = val;
    }

    // Decode ciphertext
    let plaintext = raw_content
        .chunks(keysize)
        .flat_map(|x| {
            x.iter()
                .zip(&final_key)
                .map(|(a, b)| a ^ b)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    let plaintext_str =
        String::from_utf8(plaintext).expect("Failed to convert plaintext to string");

    // convert final key to string for display
    let final_key_str =
        String::from_utf8(final_key.to_vec()).expect("Failed to convert final key to string");

    println!("Key: {final_key_str}");
    println!("Decoded Ciphertext:\n{plaintext_str}")
}

pub fn ch7() {
    println!("Set 1 Challenge 7:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!("AES in ECB mode

        The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode under the key

        \"YELLOW SUBMARINE\".

        (case-sensitive, without the quotes; exactly 16 characters; I like \"YELLOW SUBMARINE\" because it's exactly 16 bytes long, and now you do too).

        Decrypt it. You know the key, after all.

        Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.");
    };
    // read in file and decode
    // read in and base64 decode our challenge content
    let candidates = match fs::File::open("./data/7.txt") {
        Ok(cor) => cor,
        Err(_) => panic!("Failed to read ./data/7.txt"),
    };
    let reader = BufReader::new(candidates);

    let mut base64_content = String::new();

    for line in reader.lines() {
        let partial = match line {
            Ok(l) => match l.strip_suffix("\n") {
                Some(ls) => ls.to_string(),
                None => l,
            },
            Err(_) => break,
        };
        base64_content += &partial;
    }

    // All the pieces we need for the decryption
    let raw_content = base64::decode(base64_content).expect("Failed to decode base64 content");
    let key = b"YELLOW SUBMARINE";
    let mut plaintext: Vec<u8> = Vec::new();
    let cipher = Cipher::aes_128_ecb();

    // get plaintext block or report error
    let block = decrypt(cipher, key, None, &raw_content);
    match block {
        Ok(c) => plaintext.append(&mut c.clone()),
        Err(e) => println!("{e}"),
    }

    // provide output
    println!(
        "Decrypting 7.txt with key {}",
        String::from_utf8(key.to_vec()).expect("failure to transform key to string")
    );
    println!(
        "plaintext:\n{}",
        String::from_utf8(plaintext).expect("failure to transform to plaintext to string")
    );
}

pub fn ch8() {
    println!("Set 1 Challenge 8:");
    if let Ok(_) = env::var("DESCRIBE") {
        println!("
        Detect AES in ECB mode

        In this file are a bunch of hex-encoded ciphertexts.

        One of them has been encrypted with ECB.

        Detect it.

        Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte plaintext block will always produce the same 16 byte ciphertext.
        ");
    }

    // load candidate str from file
    let candidates = match fs::File::open("./data/8.txt") {
        Ok(cor) => cor,
        Err(_) => panic!("Failed to read ./data/8.txt"),
    };
    let reader = BufReader::new(candidates).lines();

    // save our scores to detect lowest (turns out rest of strings are all unique)
    let mut scores: HashMap<usize, usize> = HashMap::new();

    // decode string and count unique blocks
    for (count, line) in reader.enumerate() {
        let mut counter = HashSet::<&[u8]>::new();
        if let Ok(l) = line {
            let t_line = hex::decode(l).unwrap();
            t_line.chunks(16).for_each(|x| {
                counter.insert(&x);
            });
            scores.insert(count, counter.len());
        }
    }

    // Get most likely score
    let mut lowest = 100;
    let mut line = 0_usize;
    for (i, score) in scores {
        if score < lowest {
            lowest = score;
            line = i + 1;
        }
    }

    // display results
    println!("With Score {lowest} we think line {line} is ECB encoded");
    println!("Detecting ECB encrypted block from 8.txt");
}
