use std::fs;

fn main() {
    let text: String = fs::read_to_string("input").expect("Couldn't open a file!");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for (i, line) in text.split_whitespace().enumerate() {
        let parsed = line.parse().expect("Parse error!");
        if i % 2 != 0 {
            left.push(parsed);
        } else {
            right.push(parsed);
        }
    }

    if left.len() != right.len() {
        panic!("Invalid left/right column sizes");
    }
 
    left.sort_unstable();
    right.sort_unstable();
    
    let ans1: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    
    let mut curr: i32 = 0;
    let mut ans2: i32 = 0;
    let mut count: i32 = 0;
    let mut rpos: usize = 0;
    for &l in &left {
        if curr == l {
            ans2 += count;
        } else {
            count = 0;
            curr = l;
            while rpos < length {
                if l == right[rpos] { count += l; }
                else if l < right[rpos] { break; }
                rpos += 1;
            }
            ans2 += count;
        }
    }

    println!("{} {}", ans1, ans2);
}
