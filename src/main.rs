// crate::main
// Main executable for Moss Oxide

use std::io;
use std::io::Write;

use hash::krhash::hash_ascii;

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

    let hashes: Vec<u64> = hash_ascii(&bytes, guarantee_threshold, noise_threshold);

    println!("\nINPUT\n{}", input);
    println!("RAW BYTES\n{:#?}\n", bytes);
    println!("HASHES\n{:#?}\n", hashes);
}