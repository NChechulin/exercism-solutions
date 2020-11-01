/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut digits: Vec<u8> = vec![];

    for (i, c) in isbn.chars().enumerate() {
        if c.is_numeric() {
            digits.push(c.to_string().parse::<u8>().unwrap());
        } else if c == 'X' && i == isbn.len() - 1 {
            digits.push(10);
        }
    }
 
    for i in &digits {
        print!("{} ", i);
    }

    if digits.len() != 10 {
        return false;
    }

    let mut sum: i32 = 0;

    for i in 0..10 {
        sum += (10 - i) * digits[i as usize] as i32;
    }

    sum % 11 == 0
}
 