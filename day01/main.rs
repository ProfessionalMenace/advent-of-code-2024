use std::fs;

#[inline(always)]
fn solve1(left: &Vec<i32>, right: &Vec<i32>) {
    let ans: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    println!("Solution 1 {}", ans);
}

#[inline(always)]
fn solve2(left: &Vec<i32>, right: &Vec<i32>) {
    let mut curr: i32 = 0;
    let mut ans: i32 = 0;
    let mut count: i32 = 0;
    let mut rpos: usize = 0;
    for &l in left {
        if curr == l {
            ans += count;
        } else {
            count = 0;
            curr = l;
            while rpos < right.len() {
                if l == right[rpos] { count += l; }
                else if l < right[rpos] { break; }
                rpos += 1;
            }
            ans += count;
        }
    }
    println!("Solution 2 {}", ans)
}

#[inline(always)]
fn solve_both(filename: &str) {
    let text: String = fs::read_to_string(filename).expect("File not found");
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

    solve1(&left, &right);
    solve2(&left, &right);
}

fn main() {
    solve_both("input");
}
