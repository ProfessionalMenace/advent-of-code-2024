use std::fs;

fn solve1(filename: &str) {
    let text = fs::read_to_string(filename).expect("Couldn't open the file");
    let mut ans: i64 = 0;
    for line in text.lines() {
        let numbers = line.split(&[':', ' ']).filter_map(|tok| tok.parse().ok()).collect::<Vec<i64>>();
        let mut arr: Vec<i64> = vec![numbers[0]];
        for i in (1..numbers.len()).rev() {
            for j in 0..arr.len() {
                if arr[j]%numbers[i] == 0 { arr.push(arr[j]/numbers[i]); }
                arr[j] -= numbers[i];
            }
        }
        if arr.iter().any(|&val| val == 0) {
            ans += numbers[0];
        }
    }
    println!("{}: {:?}", filename, ans);
}

fn main() {
    solve1("test");
    solve1("input");
}
