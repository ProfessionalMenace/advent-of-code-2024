use std::fs;
use std::time::Instant;

fn check_safety(line: &Vec<i32>) -> bool {
    let safety1 = line.is_sorted_by(|a,b| a < b);
    let safety2 = line.is_sorted_by(|a,b| a > b);
    let safety3 = line.windows(2)
        .map( | window | (window[0] - window[1]))
        .all( | diff | diff.abs() < 4 && diff.abs() > 0);
    return safety3 && (safety1 || safety2);
}

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

    let ans1 = parsed_lines
        .iter()
        .fold(0, |acc, line| {
            if check_safety(&line) { 
                acc + 1
            } else {
                acc
            }
        });

    let ans2 = parsed_lines
        .iter()
        .fold(0, |acc, line| {
            let mut safety = false;
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
            if safety {
                acc + 1
            } else {
                acc
            }
        });


        println!("{} {} {:.2?}", ans1, ans2, time_start.elapsed());
}
