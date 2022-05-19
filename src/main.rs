// crate::main
// Main executable for Moss Oxide

use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("\nEnter input string> ");
    io::stdout().flush().unwrap();

    let input_result = io::stdin().read_line(&mut input);

    match input_result {
        Ok(_) => {},
        Err(_) => {
            // TODO: Implement more robust error handling
            todo!();
        }
    }

    // TODO: Clean up newlines (CRLF vs. LF)
    let bytes: &[u8] = input.as_bytes();
    let guarantee_threshold: usize = 8;
    let noise_threshold: usize = 4;

    let hashes: Vec<i128> = hash_ascii(&bytes, guarantee_threshold, noise_threshold);

    println!("\nINPUT\n{}", input);
    println!("RAW BYTES\n{:#?}\n", bytes);
    println!("HASHES\n{:#?}\n", hashes);
}


const BASE: i128 = 256;


/// Computes Karp-Rabin hashes on the given ASCII bytestring with substrings of length `k`.
/// 
/// # Arguments
/// `bytes`: a reference to a slice of 8-bit unsigned integers
/// `t`: a guarantee threshold.  We want to detect all substring matches of this length or longer.
/// `k`: a noise threshold.  We want to discard all substring matches of this length or shorter.
/// 
/// Returns a slice of 32-bit unsigned integers representing each K-R hash.
fn hash_ascii(bytes: &[u8], t: usize, k: usize) -> Vec<i128> {
    let mut i = 0;

    // Define a "window size"
    let w: usize = t - k + 1;

    let mut bytes: Vec<u8> = bytes.to_vec();

    // Pad the bytestring with zeros if it's less than the window size
    if bytes.len() < w {
        let more_bytes: Vec<u8> = vec![0u8; w - bytes.len()];
        bytes.extend(more_bytes);
    }

    let mut hash: i128 = 0;
    let mut hashes: Vec<i128> = Vec::new();

    // One of the benefits of the Karp-Rabin hash algorithm is its relative speed as compared
    // to other hashing algorithms.  An implementation of K-R only has to compute one hash
    // at the very start, and then all further hashes are actually computed from the previous hash.
    while i < bytes.len() - k {
        let slice: &[u8] = &bytes[i..i+k+1];

        if i == 0 {
            // Compute the first K-R hash
            for (j, c) in slice.iter().enumerate() {
                let power: u32 = k as u32 - j as u32 + 1;
                hash += BASE.pow(power) + *c as i128;
                println!("Adding character '{}' by adding {} ^ {} * {} to hash", *c as char, BASE, power, c);
            }
            hashes.push(hash);
        } else {
            let discarded_value = bytes[i - 1];

            // Compute the `i+1`th K-R hash from the `i`th hash
            hash -= BASE.pow(k as u32)*(discarded_value as i128);
            println!("Discarded {} by subtracting {} ^ {} * {}", discarded_value as char, BASE, k, discarded_value);

            hash += slice[k] as i128;
            println!("Adding {} to hash", slice[k] as char);

            hash *= BASE;
            println!("Multiplying hash by {}", BASE);

            hashes.push(hash);
        }
        
        println!("Hash is now {}\n", hash);

        // Increment `i`
        i += 1;
    }

    hashes
}