use std::fs;

fn parse(path: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(": ").unwrap();
            let parsed_lhs = lhs.parse().unwrap();
            let parsed_rhs = rhs
                .split_whitespace()
                .map(|tok| tok.parse().unwrap())
                .collect();
            (parsed_lhs, parsed_rhs)
        })
        .collect()
}

fn checked_div(target: i64, val: i64) -> Option<i64> {
    if target % val == 0 {
        Some(target / val)
    } else {
        None
    }
}

fn checked_add(target: i64, val: i64) -> Option<i64> {
    if target >= val {
        Some(target - val)
    } else {
        None
    }
}

// reverse concat
fn checked_con(mut num: i64, mut val: i64) -> Option<i64> {
    while (num > 0) && (val > 0) && (num % 10) == (val % 10) {
        num /= 10;
        val /= 10;
    }

    if val == 0 {
        Some(num)
    } else {
        None
    }
}

fn combinator_1(target: i64, idx: usize, values: &Vec<i64>) -> bool {
    let val = values[idx];
    if idx == 0 {
        return val == target;
    };
    let check_add = if let Some(res) = checked_add(target, val) {
        combinator_1(res, idx - 1, values)
    } else {
        false
    };
    let check_div = check_add
        || if let Some(res) = checked_div(target, val) {
            combinator_1(res, idx - 1, values)
        } else {
            false
        };
    return check_div;
}

fn combinator_2(target: i64, idx: usize, values: &Vec<i64>) -> bool {
    let val = values[idx];
    if idx == 0 {
        return val == target;
    };
    let check_add = if let Some(res) = checked_add(target, val) {
        combinator_2(res, idx - 1, values)
    } else {
        false
    };
    let check_div = check_add
        || if let Some(res) = checked_div(target, val) {
            combinator_2(res, idx - 1, values)
        } else {
            false
        };
    let check_con = check_div
        || if let Some(res) = checked_con(target, val) {
            combinator_2(res, idx - 1, values)
        } else {
            false
        };
    return check_con;
}

fn solve_part_one(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut ans: i64 = 0;
    for (target, values) in data {
        if combinator_1(*target, values.len() - 1, &values) {
            ans += *target;
        }
    }
    return ans;
}

fn solve_part_two(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut ans: i64 = 0;
    for (target, values) in data {
        if combinator_2(*target, values.len() - 1, &values) {
            ans += *target;
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_part_one() {
        let parsed_lines = parse("inputs/day07.txt");

        let part_one = solve_part_one(&parsed_lines);
        assert_eq!(part_one, 42283209483350);
    }

    #[test]
    fn solution_part_two() {
        let parsed_lines = parse("inputs/day07.txt");

        let part_two = solve_part_two(&parsed_lines);
        assert_eq!(part_two, 1026766857276279);
    }
}
