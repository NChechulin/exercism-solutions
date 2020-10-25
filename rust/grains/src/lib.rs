pub fn square(s: u32) -> u64 {
    2 << s
}

pub fn total() -> u64 {
    (0..64).map(|x| square(x)).sum()
}
