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

    let hashes: &[u32] = hash_ascii(&bytes);

    println!("\nINPUT\n{}", input);
    println!("RAW BYTES\n{:#?}\n", bytes);
    println!("HASHES\n{:#?}\n", hashes);
}


/// Computes Karp-Rabin hashes on the given ASCII bytestring with substrings of length `k`.
/// 
/// Returns a slice of 32-bit unsigned integers representing each K-R hash.
fn hash_ascii(bytes: &[u8]) -> &[u32] {
    &[0u32]
}