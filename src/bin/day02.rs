use std::fs;

pub fn parse(path: &str) -> Vec<Vec<i32>> {
    let text = fs::read_to_string(path).unwrap();
    let mut parsed_data: Vec<Vec<i32>> = Vec::new();
    for line in text.lines() {
        let parsed_line = line
            .split_whitespace()
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect();
        parsed_data.push(parsed_line);
    }
    return parsed_data;
}

fn check_safety(line: &Vec<i32>) -> bool {
    let get_iter = || line.windows(2).map(|window| window[1] - window[0]);
    let check1 = get_iter().all(|diff| -3 <= diff && diff <= -1);
    let check2 = get_iter().all(|diff| 1 <= diff && diff <= 3);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let parsed_data = parse("inputs/day02.txt");
        
        let part_one = solve_part_one(&parsed_data);
        assert_eq!(part_one, 334);

        let part_two = solve_part_two(&parsed_data);
        assert_eq!(part_two, 400);
    }
}
