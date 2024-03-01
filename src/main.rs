use wasm_bindgen::prelude::*;
mod cpu_jitter;
mod personalization;
mod wordlist;
use cpu_jitter::generate_u64_cpujitter;
use personalization::generate_personalization_string;
use wordlist::ALPHABET_WORDSFI;
mod bit_helpers;
use bit_helpers::{u64_to_bytes, vec_u8_to_u64};
mod hmac_drbg;
mod os_random;
use hmac_drbg::HmacDrbg;
use os_random::generate_u64_os;
use zeroize::Zeroize;

// Generate a random u64 combining three different sources
pub fn generate_random_u64() -> Option<u64> {
    // Generate a 1536 bit seed from two different random number sources.
    // Thats 8 * 64 = 512 bits from each source.
    let mut seed: Vec<u8> = Vec::new();

    for _ in 0..8 {
        let val = generate_u64_os();
        let u64_bytes = u64_to_bytes(val);
        seed.extend_from_slice(&u64_bytes);

        let val = generate_u64_cpujitter();
        let u64_bytes = u64_to_bytes(val.unwrap());
        seed.extend_from_slice(&u64_bytes);
    }

    // Generate a deterministic, but each time unique, personalization string
    let mut personalization_string: [u8; 32] = generate_personalization_string();

    // Generate the u64 random number using HMAC DRBG
    let mut drbg = HmacDrbg::new(&seed, &personalization_string);
    let random_bytes = drbg.generate_bytes(8);
    let random_value = vec_u8_to_u64(&random_bytes);

    personalization_string.zeroize();
    seed.zeroize();

    random_value
}

#[wasm_bindgen]
pub fn random_words(n: usize) -> String {
    let l = ALPHABET_WORDSFI.len();

    let mut output = Vec::new();
    for _ in 0..n {
        let r = generate_random_u64().unwrap() as usize;
        output.push(ALPHABET_WORDSFI[r % l]) // TODO: fix biased indexing
    }
    output.join("-").to_string()
}

fn main() {}
