pub fn u64_to_bytes(val: u64) -> [u8; 8] {
    [
        (val >> 56) as u8,
        (val >> 48) as u8,
        (val >> 40) as u8,
        (val >> 32) as u8,
        (val >> 24) as u8,
        (val >> 16) as u8,
        (val >> 8) as u8,
        val as u8,
    ]
}

pub fn vec_u8_to_u64(bytes: &[u8]) -> Option<u64> {
    if bytes.len() != 8 {
        return None; // Ensure that the input has exactly 8 bytes
    }

    let mut result: u64 = 0;
    for &byte in bytes {
        result = (result << 8) | u64::from(byte);
    }

    Some(result)
}

pub struct BitVector {
    bits: Vec<bool>,
}

impl BitVector {
    pub fn new() -> Self {
        BitVector { bits: Vec::new() }
    }

    pub fn add_bit(&mut self, bit: bool) {
        self.bits.push(bit);
    }

    pub fn is_full(&self) -> bool {
        self.bits.len() == 64
    }

    pub fn to_u64(&self) -> u64 {
        let mut result: u64 = 0;
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                result |= 1u64 << i;
            }
        }
        result
    }
}
