use std::fs;

pub fn solve(filename: &str) {
    let text = fs::read_to_string(filename).unwrap();
    let mut parsed_data: Vec<Vec<i32>> = Vec::new();
    for line in text.lines() {
        let parsed_line = line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect();
        parsed_data.push(parsed_line);
    }

    let part_one = solve_part_one(&parsed_data);
    let part_two = solve_part_two(&parsed_data);
    
    println!("Solutions to day 2:");
    println!("    Part one: {part_one}");
    println!("    Part two: {part_two}");
}

fn check_safety(line: &Vec<i32>) -> bool {
    let get_iter = || line.windows(2).map(|window| window[1] - window[0]);
    let check1 = get_iter().all(|diff| -3 <= diff && diff <= -1); 
    let check2 = get_iter().all(|diff|  1 <= diff && diff <=  3); 
    return check1 || check2;
}

fn solve_part_one(parsed_data: &Vec<Vec<i32>>) -> usize {
    parsed_data.iter().filter(|line| check_safety(line)).count()
}

fn solve_part_two(parsed_data: &Vec<Vec<i32>>) -> usize {
    let mut ans: usize = 0;
    let mut new_line: Vec<i32> = Vec::new();
    for line in parsed_data {
        for i in 0..=line.len() {
            new_line.clear();
            for j in 0..line.len() {
                if i != j {
                    new_line.push(line[j]);
                }
            }

            if check_safety(&new_line) {
                ans += 1;
                break;
            }
        }
    }
    return ans;
}

