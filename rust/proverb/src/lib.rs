pub fn build_proverb(list: &[&str]) -> String {
    let mut ans = String::new();

    for i in 1..list.len() {
        ans.push_str(&format!(
            "For want of a {0} the {1} was lost.\n",
            list[i - 1],
            list[i]
        ));
    }

    if list.len() != 0 {
        ans.push_str(&format!("And all for the want of a {0}.", list[0]));
    }

    ans
}
