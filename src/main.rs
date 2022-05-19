// crate::main
// Main executable for Moss Oxide

use std::io;

fn main() {
    let mut input = String::new();

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

    println!("{}", input);
    println!("{:#?}", bytes);
}


/// Computes Karp-Rabin hashes on the given ASCII bytestring with substrings of length `k`.
/// 
/// Returns a slice of 32-bit unsigned integers representing each K-R hash.
fn hash_ascii(bytes: &[u8]) -> &[u32] {
    &[0u32]
}