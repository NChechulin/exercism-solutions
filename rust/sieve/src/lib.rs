pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sift: Vec<bool> = vec![true; (upper_bound + 2) as usize];
    let mut ans: Vec<u64> = vec![];

    for i in 2..(upper_bound + 1) {
        if sift[i as usize] {
            ans.push(i);

            for j in (2 * i..(upper_bound + 1)).step_by(i as usize) {
                sift[j as usize] = false;
            }
        }
    }

    ans
}
