// crate::krhash::mod.rs

const BASE: i128 = 256;
const HASHMAX: i128 = u64::MAX as i128;

/// Computes Karp-Rabin hashes on the given ASCII bytestring with substrings of length `k`.
/// 
/// # Arguments
/// `bytes`: a reference to a slice of 8-bit unsigned integers
/// `t`: a guarantee threshold.  We want to detect all substring matches of this length or longer.
/// `k`: a noise threshold.  We want to discard all substring matches of this length or shorter.
/// 
/// Returns a slice of 32-bit unsigned integers representing each K-R hash.
pub fn hash_ascii(bytes: &[u8], t: usize, k: usize) -> Vec<u64> {
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
    let mut hashes: Vec<u64> = Vec::new();

    // One of the benefits of the Karp-Rabin hash algorithm is its relative speed as compared
    // to other hashing algorithms.  An implementation of K-R only has to compute one hash
    // at the very start, and then all further hashes are actually computed from the previous hash.
    while i < bytes.len() - k {
        let slice: &[u8] = &bytes[i..i+k+1];

        if i == 0 {
            // Compute the first K-R hash
            for (j, c) in slice.iter().enumerate() {
                let power: u32 = k as u32 - j as u32 + 1;
                hash += BASE.pow(power)*(*c as i128);
                hash %= HASHMAX;
            }
            hashes.push(hash as u64);
        } else {
            let discarded_value = bytes[i - 1];

            // Compute the `i+1`th K-R hash from the `i`th hash
            hash -= BASE.pow(k as u32 + 1)*(discarded_value as i128);

            hash += slice[k] as i128;

            hash *= BASE;

            hash %= HASHMAX;

            hashes.push(hash as u64);
        }
        
        // Increment `i`
        i += 1;
    }

    hashes
}