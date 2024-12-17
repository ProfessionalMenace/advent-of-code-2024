use std::fs;

#[inline(always)]
fn check_safety(line: &Vec<i32>) -> bool {
    let safety1 = line.is_sorted_by(|a,b| a < b);
    let safety2 = line.is_sorted_by(|a,b| a > b);
    let safety3 = line.windows(2)
        .map( | window | (window[0] - window[1]))
        .all( | diff | diff.abs() < 4 && diff.abs() > 0);
    return safety3 && (safety1 || safety2);
}

#[inline(always)]
fn solve1(lines: &Vec<Vec<i32>>) {
    let ans = lines
        .iter()
        .filter(|line| check_safety(&line))
        .count();
    println!("Solution 1 {}", ans);
}

#[inline(always)]
fn solve2(lines: &Vec<Vec<i32>>) {
    let ans = lines
        .iter()
        .filter(|line| {
            let mut safety: bool = false;
            for i in 0..line.len() {
                let mut vec = Vec::new();
                for (j, &num) in line.iter().enumerate() {
                    if i != j { vec.push(num); }
                }

                safety = safety || check_safety(&vec);
                if safety { 
                    break;
                }
            }
            safety
        })
        .count();
    println!("Solution 2 {}", ans);
}

fn solve_both(filename: &str) {
    let text: String = fs::read_to_string(filename).expect("File not found");

    let parsed_lines: Vec<_> = text
        .lines()
        .map(|line| { 
            line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    solve1(&parsed_lines);
    solve2(&parsed_lines);
}

fn main() {
    solve_both("test");
    solve_both("input");
}
