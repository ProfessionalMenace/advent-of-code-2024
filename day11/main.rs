use std::fs;

#[inline(always)]
fn split(num: u64) -> Option<(u64, u64)> {
    let mut temp = num;
    let mut digits = 0;

    while temp > 0 {
        temp /= 10;
        digits += 1;
    }

    if digits%2 == 1 {
        return None
    }

    let div = 10u64.pow(digits/2);
    return Some((num / div, num % div));

}

fn solve(stone: u64, count: u32) -> u64 {
    if count == 0 {
        return 1;
    }

    if stone == 0 {
        return solve(1, count - 1);
    }

    if let Some((s1, s2)) = split(stone) {
        return solve(s1, count - 1) + solve(s2, count - 1);
    }

    return solve(2024*stone, count - 1)
}

fn solve_file(filename: &str) -> u64 {
    let text: String = fs::read_to_string(filename).expect("File not found");
    let ans: u64 = text
        .trim()
        .split(' ')
        .map(|tok| tok.parse::<u64>().expect("Parsing Err"))
        .map(|num| solve(num, 25))
        .sum();

    return ans;
}

fn main() {
    println!("{}", solve_file("input"));
}   
