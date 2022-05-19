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

    let bytes: &[u8] = input.as_bytes();

    println!("{}", input);
    println!("{:#?}", bytes);
}