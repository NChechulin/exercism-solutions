pub fn my_modulo(mut x: i8, modulo: i8) -> u8 {
    x %= &modulo;
    while x < 0 {
        x += &modulo;
    }
    x as u8
}

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c.is_alphabetic() {
            false => c,
            true => {
                let first = match c.is_ascii_lowercase() {
                    true => 'a' as u8,
                    false => 'A' as u8,
                };
                (first + (c as u8 + my_modulo(key, 26) - first) % 26) as char
            }
        })
        .collect()
}
