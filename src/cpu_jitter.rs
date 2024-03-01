use crate::bit_helpers::{u64_to_bytes, vec_u8_to_u64, BitVector};
use tiny_keccak::Hasher;
use tiny_keccak::Sha3;

// generate_u64_cpujitter()
// SHA3 (Keccack) is used to provide a u64 random number from 512 bits of cpujitter entropy bits.
// Rationale for this is that the cpujitter is not 100% random, but it is still a good source of entropy.
// Also, using the HMAC DRBG with the current personalization string (*that contains the timestamp*)
// would result in difficulties when estimating the randomness of the generated random numbers.
pub fn generate_u64_cpujitter() -> Option<u64> {
    // Let's take 512 (8 * 64) bits of cpujitter entropy
    let mut combined_data = Vec::new();
    for _ in 0..8 {
        if let Some(raw_value) = generate_u64_cpujitter_raw() {
            combined_data.extend_from_slice(&u64_to_bytes(raw_value));
        }
    }

    // Hash the combined data with SHA3 (Keccak)
    let mut sha3 = Sha3::v256();
    let mut hash_result = [0u8; 32];
    sha3.update(&combined_data);
    sha3.finalize(&mut hash_result);

    // Return the first 64 bits as u64
    let random_value = vec_u8_to_u64(&hash_result[..8]);
    random_value
}

/* Returns U64 from collected CPU jitter. The amount of raw entropy is around 6bits / byte. */
pub fn generate_u64_cpujitter_raw() -> Option<u64> {
    let mut bit_vector = BitVector::new();
    let mut loop_count = 0;

    loop {
        let start = std::time::Instant::now();
        let end = std::time::Instant::now();
        let time_diff1 = end.duration_since(start).as_nanos() as u64;

        let start = std::time::Instant::now();
        let end = std::time::Instant::now();
        let time_diff2 = end.duration_since(start).as_nanos() as u64;

        if time_diff1 != time_diff2 {
            if time_diff1 > time_diff2 {
                bit_vector.add_bit(true);
            } else {
                bit_vector.add_bit(false);
            }
        }

        // Check if the BitVector is full or if the loop count has reached maximum number of tries.
        if bit_vector.is_full() {
            break;
        }

        loop_count += 1;
        if loop_count >= 32768 {
            eprintln!(
                "Error: Unable to create cpu jitter entropy. System too busy or idle? Exiting."
            );
            std::process::exit(1);
        }
    }

    let result = bit_vector.to_u64();
    Some(result)
}
