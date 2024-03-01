pub fn generate_u64_os() -> u64 {
    let mut buf: Vec<u8> = (0..8).map(|_| 0).collect();
    getrandom::getrandom(&mut buf).unwrap();
    u64::from_be_bytes(buf.try_into().unwrap())
}
