pub fn verse(n: u32) -> String {
    if n == 2 {
        return String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    } else if n == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    } else if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else {
        let n_str = n.to_string();
        let part1 = " bottles of beer on the wall, ".to_owned();
        let part2 = " bottles of beer.\nTake one down and pass it around, ".to_owned();
        let nm1_str = (n - 1).to_string();
        let part3 = " bottles of beer on the wall.\n";

        let mut ans = String::new();
        ans.push_str(&n_str);
        ans.push_str(&part1);
        ans.push_str(&n_str);
        ans.push_str(&part2);
        ans.push_str(&nm1_str);
        ans.push_str(&part3);

        return ans;
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ans = String::new();
    println!("{} {}", start, end);

    for i in end..start + 1 {
        let mut temp = verse(start + end - i);
        if i != start {
            temp.push_str("\n");
        }
        ans.push_str(&temp);
    }

    ans
}
