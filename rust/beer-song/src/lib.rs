pub fn verse(n: u32) -> String {
    if n == 2 {
        return String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    } else if n == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    } else if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else {
        return format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        );
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
