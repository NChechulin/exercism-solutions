pub fn factors(mut n: u64) -> Vec<u64> {
    let mut ans: Vec<u64> = vec![];

    let mut divisor = 2;
    while divisor * divisor <= n && n != 1 {
        while n % divisor == 0 {
            n /= divisor;
            ans.push(divisor);
        }

        divisor += 1;
    }

    if n != 1 {
        ans.push(n);
    }

    ans
}
