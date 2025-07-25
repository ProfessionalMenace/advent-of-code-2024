use std::fs;

pub fn solve(filename: &str) {
    let text = fs::read_to_string(filename).unwrap();
    let numbers: Vec<i32> = text
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    for pair in numbers.chunks(2) {
        left_column.push(pair[0]);
        right_column.push(pair[1]);
    }

    left_column.sort_unstable();
    right_column.sort_unstable();

    let part_one = solve_part_one(&left_column, &right_column);
    let part_two = solve_part_two(&left_column, &right_column);

    println!("Solutions to day 1:");
    println!("    Part one: {part_one}");
    println!("    Part two: {part_two}");
}

fn solve_part_one(left_column: &Vec<i32>, right_column: &Vec<i32>) -> i32 {
    left_column
        .iter()
        .zip(right_column.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn solve_part_two(left_column: &Vec<i32>, right_column: &Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    let mut count: i32 = 0;

    let mut j: usize = 0;
    for i in 0..left_column.len() {
        if i != 0 && left_column[i] == left_column[i - 1] {
            ans += count * left_column[i];
            continue;
        }
        count = 0;
        while j < right_column.len() {
            if left_column[i] == right_column[j] {
                count += 1;
            } else if left_column[i] < right_column[j] {
                break;
            }
            j += 1;
        }
        ans += count * left_column[i];
    }

    return ans;
}
