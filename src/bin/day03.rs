use std::fs;

fn find_all<'a>(text: &String, search_tokens: Vec<&'a str>) -> Vec<(usize, &'a str)> {
    let mut positions: Vec<(usize, &str)> = Vec::new();
    let mut i: usize = 0;
    while i < text.len() {
        for &part in &search_tokens {
            if text[i..].starts_with(part) {
                positions.push((i + part.len(), part));
                i += part.len() - 1;
                break;
            }
        }
        i += 1;
    }
    return positions;
}

fn parse_mul(chunk: &str) -> u32 {
    let mut parsed_lhs: u32 = 0;
    let mut parsed_rhs: u32 = 0;
    let mut iter = chunk.chars();

    for c in iter.by_ref().take(4) {
        if c == ',' { break };
        if let Some(digit) = c.to_digit(10) {
            parsed_lhs = 10 * parsed_lhs + digit;
        } else {
            return 0;
        }
    }

    for c in iter.by_ref().take(4) {
        if c == ')' { break };
        if let Some(digit) = c.to_digit(10) {
            parsed_rhs = 10 * parsed_rhs + digit;
        } else {
            return 0;
        }
    }

    return parsed_lhs * parsed_rhs;
}

fn solve_part_one(text: &String) -> u32 {
    let mut ans: u32 = 0;
    let pairs = find_all(text, vec!["mul("]);
    for (i, _) in pairs {
        ans += parse_mul(&text[i..]);
    }
    return ans;
}

fn solve_part_two(text: &String) -> u32 {
    let pairs = find_all(text, vec!["mul(", "do()", "don't()"]);
    let mut ans: u32 = 0;
    let mut enable: bool = true; 
    for (i, tok) in pairs {
        match tok {
            "do()" => enable = true,
            "don't()" => enable = false,
            "mul(" => if enable { 
                ans += parse_mul(&text[i..]);
            },
            _ => panic!(),
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let text = fs::read_to_string("inputs/day03.txt").unwrap();

        let part_one = solve_part_one(&text);
        assert_eq!(part_one, 161289189);

        let part_two = solve_part_two(&text);
        assert_eq!(part_two, 83595109);
    }
}
