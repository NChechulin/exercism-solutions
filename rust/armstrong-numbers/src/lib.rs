pub fn is_armstrong_number(num: u32) -> bool {
    let mut ans: u32 = 0;
    let mut n: u32 = num;
    let power: u32 = num.to_string().len() as u32;

    while n > 0 && ans < num {
        ans += (n % 10).pow(power);
        n /= 10;
    }

    num == ans
}
