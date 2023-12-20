use lazy_static::lazy_static;
use std::collections::HashMap;

use openssl::{
    error::ErrorStack,
    symm::{decrypt, encrypt, Cipher},
};

lazy_static! {
    // frequency scores fo teh english language
    static ref LETTER_FREQ: HashMap<u8, f64> = HashMap::from([
        (b'a', 0.08167),
        (b'b', 0.01492),
        (b'c', 0.02782),
        (b'd', 0.04253),
        (b'e', 0.1270),
        (b'f', 0.02228),
        (b'g', 0.02015),
        (b'h', 0.06094),
        (b'i', 0.06966),
        (b'j', 0.00153),
        (b'k', 0.00772),
        (b'l', 0.04025),
        (b'm', 0.02406),
        (b'n', 0.06749),
        (b'o', 0.07507),
        (b'p', 0.01929),
        (b'q', 0.00095),
        (b'r', 0.05987),
        (b's', 0.06327),
        (b't', 0.09056),
        (b'u', 0.02758),
        (b'v', 0.00978),
        (b'w', 0.02360),
        (b'x', 0.00150),
        (b'y', 0.01974),
        (b'z', 0.00074),
    ]);
}

// Implements an englishness test where a score of 1 is english and 0 is not english.
// based off stackoverflow article it implements the 'Bhattacharyya Coefficient' and links to the below github
// https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis
#[allow(dead_code)]
pub fn score_as_english_text(candidate: &[u8]) -> f64 {
    // frequencey of the english language

    // how many times each letter is in candidate
    let mut population: HashMap<u8, f64> = HashMap::new();
    population.reserve(26);

    candidate.iter().for_each(|c| {
        population.insert(*c, candidate.iter().filter(|x| **x == *c).count() as f64);
    });

    //calculate 'Bhattacharyya Coefficient'
    population
        .iter()
        .map(|(x, y)| match LETTER_FREQ.get(x) {
            Some(n) => ((n * *y) / (candidate.len() as f64)).sqrt(),
            None => 0.0,
        })
        .sum()
}

#[allow(dead_code)]
/// Function to brute force a single byte xor'd piece of text and then score against the english language.
pub fn xor_and_score_byte_array(input_raw: Vec<u8>) -> (u8, Vec<u8>, f64) {
    // map of possible single byte xor key and output
    let mut mapping: HashMap<u8, Vec<u8>> = HashMap::new();
    mapping.reserve(255);

    // For each key in value 0 to 255 compute output
    (0..=255).for_each(|i| {
        mapping.insert(
            i,
            input_raw.iter().map(|x| x ^ i as u8).collect::<Vec<u8>>(),
        );
    });

    // find highest scored output against english language frequency
    let mut highest = 0_f64;
    let mut key = b'a';

    mapping.iter().for_each(|(k, v)| {
        let s = score_as_english_text(v);
        if s > highest {
            highest = s;
            key = *k;
        }
    });

    let ans = mapping.get(&key).unwrap().to_vec();
    (key, ans, highest)
}

#[allow(dead_code)]
pub fn hamming_distance(input1: &[u8], input2: &[u8]) -> u64 {
    // Calculate bit level hamming distance between the input bytes
    input1
        .iter()
        .zip(input2.iter())
        .map(|(x, y)| (x ^ y).count_ones() as u8)
        .fold(0, |x, y| x + (y as u64))
}

#[allow(dead_code)]
pub fn compute_likely_keysize(ctext: &[u8]) -> usize {
    // map of keylengths to scores normalised hamming distance
    let mut scores = HashMap::<usize, f64>::new();

    // for keysize 2 to 40 check possible edit distances
    for i in 2..=40 {
        let mut iter = ctext.chunks(i);
        let mut acc = 0_f64;
        let mut round = 0;

        // compute hamming distance over 20 samples
        for y in 0..20 {
            let candidate_1 = match iter.next() {
                Some(x) => x,
                None => break,
            };

            let candidate_2 = match iter.next() {
                Some(x) => x,
                None => break,
            };

            // make sure we are dividing by actual number of rounds (incase inputs to short)
            round = y;

            // sum and normalize hamming distance
            acc += hamming_distance(candidate_1, candidate_2) as f64 / i as f64;
        }

        // compute average hamming distance across rounds
        if round != 0 {
            acc /= round as f64;
            scores.insert(i, acc);
        }
    }

    // return the lowest edit distance
    let mut topscore = 100_f64;
    let mut topscore_key = 0;

    scores.iter().skip(1).for_each(|(key, score)| {
        if *score < topscore {
            topscore = *score;
            topscore_key = *key;
        }
    });

    topscore_key
}

#[allow(dead_code)]
pub fn transpose_key_blocks(keysize: usize, ciphertext: &[u8]) -> HashMap<usize, Vec<u8>> {
    // map of index to key position across ciphertext
    let mut key_blocks: HashMap<usize, Vec<u8>> = HashMap::new();
    key_blocks.reserve(29);

    // fill map for each keyblock in ciphertext
    ciphertext.chunks(keysize).for_each(|block| {
        (0..block.len()).for_each(|i| {
            if let std::collections::hash_map::Entry::Vacant(e) = key_blocks.entry(i) {
                let inserted_vec = vec![block[i]];
                e.insert(inserted_vec);
            } else {
                key_blocks.get_mut(&i).unwrap().push(block[i]);
            }
        })
    });

    // return transposed blocks
    key_blocks
}

fn xor_blocks(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().zip(y).map(|(b1, b2)| b1 ^ b2).collect()
}

#[allow(dead_code)]
pub fn pkcs_padding(block_len: usize, plaintext: &[u8]) -> Vec<u8> {
    // create new copy of block to pad
    let mut output = Vec::from(plaintext);

    // calc padding size
    let padding_size = block_len - plaintext.chunks(block_len).last().unwrap().len();
    for _ in 0..padding_size {
        output.push(padding_size as u8);
    }

    // return with proper pkcs padding
    output
}

fn encrypt_ecb_block(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ECB cipher
    let ecb_cipher = Cipher::aes_128_ecb();

    // Encrypt plaintext block with ecb and key
    encrypt(ecb_cipher, key, None, plaintext)
}

fn decrypt_ecb_block(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    // ECB cipher
    let ecb_cipher = Cipher::aes_128_ecb();

    // Decrypt plaintext block with ecb and key
    decrypt(ecb_cipher, key, None, ciphertext)
}

#[allow(dead_code)]
pub fn encrypt_cbc(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Vec<u8> {
    todo!("BOTH THESE ENCRYPTION FUNCITONS ARE FUCKING BROKEN ???? !");

    // next block to xor with plaintext, first xor block is IV
    let mut xor_block = iv.to_vec();
    let plaintext = plaintext.to_vec();

    // // pkcs pad if not block size
    // if plaintext.len() % 32 != 0 {
    //     plaintext = pkcs_padding(32, &plaintext);
    // }

    let mut cyphertext = Vec::new();

    // Steps:
    // - break plaintext into chunks
    // - xor plaintext block N with IV if N = 1 or previous encrypted block N-1
    // - encrypt xor'd block
    // - collect into vec and return
    for block in plaintext.chunks(16) {
        let xord_plaintext = xor_blocks(block, &xor_block);

        let mut cypher_block = encrypt_ecb_block(key, &xord_plaintext).unwrap();

        xor_block = cypher_block.clone();

        cyphertext.append(&mut cypher_block);
    }

    cyphertext
}

#[allow(dead_code)]
pub fn decrypt_cbc(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    todo!("BOTH THESE ENCRYPTION FUNCITONS ARE FUCKING BROKEN ???? !");

    let mut xor_block = iv.to_vec();
    let mut plaintext: Vec<u8> = Vec::new();

    for block in ciphertext.chunks(32) {
        plaintext.append(&mut xor_blocks(
            &decrypt_ecb_block(key, block).unwrap(),
            &xor_block,
        ));
        xor_block = block.to_vec();
    }

    plaintext
}

// Test functions for various utility functions.

#[test]
fn test_hamming_distance() {
    assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37)
}

#[test]
fn test_pkcs_padding_1() {
    assert_eq!(pkcs_padding(5, b"this"), b"this\x01".to_vec());
}

#[test]
fn test_pkcs_padding_2() {
    assert_eq!(
        pkcs_padding(12, b"this will have multiple blocks"),
        b"this will have multiple blocks\x06\x06\x06\x06\x06\x06".to_vec()
    );
}

#[test]
fn test_pkcs_yello_submarine() {
    assert_eq!(
        pkcs_padding(20, b"YELLOW SUBMARINE"),
        b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec()
    );
}

#[test]
fn test_pkcs_padding_32_byte() {
    assert_eq!(
        pkcs_padding(32, b"YELLOW SUBMARINE"),
        b"YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10"
    );
}

#[test]
fn test_pkcs_padding_16_byte() {
    assert_eq!(pkcs_padding(16, b"YELLOW SUBMARINE"), b"YELLOW SUBMARINE");
}
