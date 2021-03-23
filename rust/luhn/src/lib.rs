/// Check a Luhn checksum.

pub fn code_to_digits(code: String) -> Vec<u32> {
    let code = code.chars().filter(|c| c.is_digit(10)).collect::<String>();
    code.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn count_spaces(code: String) -> usize {
    code.chars().filter(|c| c.is_whitespace()).collect::<String>().len() as usize
}

pub fn is_valid(code: &str) -> bool {
    let digits = code_to_digits(String::from(code));
    
    if digits.len() + count_spaces(String::from(code)) != code.len() {
        return false;
    }

    // double every second digit
    let digits: Vec<u32> = digits
        .iter()
        .rev()
        .enumerate()
        .map(|p| match p.0 % 2 {
            0 => *p.1,
            1 => {
                let mut ans: u32 = *p.1 * 2;
                if ans > 9 {
                    ans -= 9;
                }

                ans
            }
            _ => 0,
        })
        .rev()
        .collect();

    for c in &digits {
        println!("{}", c);
    }

    digits.len() > 1 && digits.iter().sum::<u32>() % 10 == 0
}
