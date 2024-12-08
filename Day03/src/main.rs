use std::fs;
use regex::Regex;

fn solve1(filename: &str) -> u64 {
    let text: String = fs::read_to_string(filename).unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut ans: u64 = 0;
    for (_, [lhs, rhs]) in re.captures_iter(&text).map(|c| c.extract()) {
        let a = lhs.parse::<u64>().expect("Parsing Err");
        let b = rhs.parse::<u64>().expect("Parsing Err");
        ans += a * b;
    }
    println!("Solution {}: {}", filename, ans);
    return ans;
}

fn main() {
    assert_eq!(solve1("test"), 161);
    solve1("input");
}
