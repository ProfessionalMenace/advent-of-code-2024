use std::fs;
use std::time::Instant;

fn main() {
    let time_start = Instant::now();
    let text: String = fs::read_to_string("input").unwrap();

    let parsed_lines: Vec<_> = text
        .lines()
        .map(|line| { 
            line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut mask: Vec<bool> = vec![true; parsed_lines.len()];

    for (i, line) in parsed_lines.iter().enumerate() {
        let mut prev: i32 = line[0];
        let mut pdiff:i32 = line[0] - line[1];

        for &num in &line[1..] {
            let diff = prev - num;
            if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != pdiff.signum() {
                mask[i] = false;
                break;
            }
            pdiff = diff;
            prev = num;
        }
    }
    let ans1 = mask.iter().filter(|&&val| val == true).count();

    for (i, line) in parsed_lines.iter().enumerate() {
        if mask[i] == true { continue; }
        for j in 0..line.len() {
            let mut nline: Vec<i32> = Vec::new();
            for (k, num) in line.iter().enumerate() {
                if j != k {
                    nline.push(*num);
                }
            }

            let mut safety: bool = true;
            let mut prev:  i32 = nline[0];
            let mut pdiff: i32 = nline[0] - nline[1];

            for &num in &nline[1..] {
                let diff = prev - num;
                if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != pdiff.signum() {
                    safety = false;
                    break;
                }
                pdiff = diff;
                prev = num;
            }

            if safety {
                mask[i] = true;
            }
        }
    }

    let ans2 = mask.iter().filter(|&&val| val == true).count();
    println!("{} {} {:.5?}", ans1, ans2, time_start.elapsed());
}
