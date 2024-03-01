use lazy_static::lazy_static;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref PREVIOUS_TIMESTAMP: std::sync::Mutex<(u64, u32)> = std::sync::Mutex::new((0, 0));
}

/* Personalization string combines a fixed string ("kissa123", Finnish for cat123) and both seconds and nanoseconds
   of current timestamp. This ensures that the personalization string is unique for each call.
*/
pub fn generate_personalization_string() -> [u8; 32] {
    let mut personalization_string: [u8; 32] = [0; 32];

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp_secs = timestamp.as_secs();
    let timestamp_nanos = timestamp.subsec_nanos();

    // Compare with previous timestamp
    let mut prev_timestamp = PREVIOUS_TIMESTAMP.lock().unwrap();
    if timestamp_secs < prev_timestamp.0
        || (timestamp_secs == prev_timestamp.0 && timestamp_nanos <= prev_timestamp.1)
    {
        eprintln!("Error: time went backwards. Exiting.");
        std::process::exit(1);
    }

    *prev_timestamp = (timestamp_secs, timestamp_nanos);

    // Copy the string bytes
    let hardcoded_str = "kissa123";
    personalization_string[..hardcoded_str.len()].copy_from_slice(hardcoded_str.as_bytes());

    // Copy the timestamp seconds bytes
    let secs_range = hardcoded_str.len()..hardcoded_str.len() + 8;
    personalization_string[secs_range].copy_from_slice(&timestamp_secs.to_le_bytes());

    // Copy the timestamp nanoseconds bytes
    let nanos_range = hardcoded_str.len() + 8..hardcoded_str.len() + 12;
    personalization_string[nanos_range].copy_from_slice(&timestamp_nanos.to_le_bytes());

    personalization_string
}
