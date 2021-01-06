fn is_prime(x: u32) -> bool {
    if x == 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut current: u32 = 2;

    while count != n + 1 {
        if is_prime(current) {
            count += 1;
        }
        current += 1;
    }

    current - 1
}
