pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => Some(
            1 + match n % 2 {
                0 => collatz(n / 2).unwrap(),
                1 => collatz(3 * n + 1).unwrap(),
                _ => 0,
            },
        ),
    }
}
