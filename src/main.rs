use wasm_bindgen::prelude::*;
mod wordlist;
use wordlist::ALPHABET_WORDSFI;

fn get_random_u64() -> u64 {
    let mut buf: Vec<u8> = (0..8).map(|_| 0).collect();
    getrandom::getrandom(&mut buf).unwrap();
    u64::from_be_bytes(buf.try_into().unwrap())
}

#[wasm_bindgen]
pub fn random_words(n: usize) -> String {
    let l = ALPHABET_WORDSFI.len();

    let mut output = Vec::new();
    for _ in 0..n {
        let r = get_random_u64() as usize;
        output.push(ALPHABET_WORDSFI[r % l]) // TODO: fix biased indexing
    }
    output.join("-").to_string()
}

fn main() {}
