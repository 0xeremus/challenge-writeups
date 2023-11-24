/*
 * The challenge relies on a UNIX Epoch stored at the start of the flag.enc file.
 * Once this is recovered then you can reverse the main function.
 *
 * Which generates a rand stream this is XOR'd with the flag byte and then the byte order is
 * modified according to a (rand_value & 7)
 *
 * The below program recovers the rand stream, then inverts these two operations.
 *
 * First reorder bytes, then XOR with rand value.
 */
use byteorder::{LittleEndian, ReadBytesExt};
use libc;
use std::fs::File;
use std::io::Read;
use std::num::Wrapping;

fn main() {
    // Read flag to buffer.
    let mut flag = Vec::<u8>::new();
    let mut flag_file = File::open("../flag.enc").unwrap();
    let _ = flag_file.read_to_end(&mut flag);

    //get unix time stamp.
    let time = flag
        .as_slice()
        .read_u32::<LittleEndian>()
        .expect("Failed to parse time.");
    println!("Time: {}", time);

    let rem_flag = &flag[4..];
    let plaintext_flag = decrypt_flag(time, rem_flag);

    println!("Flag: {:?}", String::from_utf8_lossy(&plaintext_flag));
}

fn decrypt_flag(time: u32, cypher_flag: &[u8]) -> Vec<u8> {
    unsafe {
        libc::srand(time);
    };
    let mut flag = Vec::<u8>::new();

    let mut rand_stream = Vec::new();
    for _ in 0..cypher_flag.len() {
        let r = unsafe { libc::rand() } as u8;
        rand_stream.push(unsafe { libc::rand() & 7 } as u8);
        rand_stream.push(r);
    }

    let mut rand_iter = rand_stream.iter();
    for &i in cypher_flag {
        let mut inter = i;

        let rand_val = (rand_iter.next().unwrap() & 7) as u8;
        inter = inter >> rand_val | inter << (8 - rand_val);

        let rand_val = *rand_iter.next().unwrap() as u8;
        inter = inter ^ rand_val as u8;
        flag.push(inter);
    }

    flag
}

#[allow(dead_code)]
fn print_rand_stream(flag_len: usize) -> ! {
    let mut rand_vals = Vec::new();

    for _ in 0..flag_len {
        rand_vals.push(unsafe { libc::rand() } as u8);
        rand_vals.push(unsafe { libc::rand() & 7 } as u8);
    }
    println!("{:?}", rand_vals);
    std::process::exit(0);
}
