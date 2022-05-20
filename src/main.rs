// crate::main
// Main executable for Moss Oxide

use std::io;
use std::io::Write;
use std::env;
use std::fs;
use std::process::exit;

use moss::fingerprint::{
    fingerprint,
};

use colored::*;


enum MossStatus {
    Ok,
    FileError,
    CLIError,
}


/// Quits execution without panics.  Allows the user to terminate the process.
fn quit(status: MossStatus) -> ! {
    match status {
        MossStatus::Ok => {
            println!("");
        },
        MossStatus::FileError => {
            println!("\n{} {}", "Fatal".bold().red(), "Could not find the file specified");
        },
        MossStatus::CLIError => {
            println!("\n{} {}", "Help".bold().yellow(), "\n\nMoss Oxide\nA very simple software similarity comparison tool.\n\nUSAGE\nmsx <FILE 1> <FILE 2>\n");
        }
    }

    exit(0);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        quit(MossStatus::CLIError);
    }
    let file1: String = args[1].clone();
    let file2: String = args[2].clone();
    
    io::stdout().flush().unwrap();

    // TODO: Implement better error handling
    let input1 = match fs::read_to_string(file1) {
        Ok(s) => s,
        Err(_) => quit(MossStatus::FileError),
    };
    let input2 = match fs::read_to_string(file2) {
        Ok(s) => s,
        Err(_) => quit(MossStatus::FileError),
    };

    // TODO: Clean up newlines (CRLF vs. LF)

    let bytes1: &[u8] = input1.as_bytes();
    let bytes2: &[u8] = input2.as_bytes();
    let guarantee_threshold: usize = 8;
    let noise_threshold: usize = 4;

    let fingerprint1: Vec<u64> = fingerprint(&bytes1, guarantee_threshold, noise_threshold);
    let fingerprint2: Vec<u64> = fingerprint(&bytes2, guarantee_threshold, noise_threshold);

    // TODO: Implememnt better fingerprint comparison techniques
    let mut count = 0;
    for value in &fingerprint1 {
        if fingerprint2.contains(value) {
            count += 1;
        }
    }
    for value in &fingerprint2 {
        if fingerprint1.contains(value) {
            count += 1;
        }
    }
    count /= 2;

    let similarity = 100.0*((count as f64*count as f64)/(fingerprint1.len() as f64*fingerprint2.len() as f64));

    println!("\nComparison: {:.4}% similar", similarity);

    quit(MossStatus::Ok);
}