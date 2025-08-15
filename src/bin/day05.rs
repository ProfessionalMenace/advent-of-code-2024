use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn parse(path: &str) -> (Vec<Vec<i32>>, HashMap<i32, HashSet<i32>>) {
    let text = fs::read_to_string(path).unwrap();
    let (rules, updates) = text.split_once("\n\n").unwrap();

    let parsed_rules: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once('|').unwrap();
            (lhs.parse().unwrap(), rhs.parse().unwrap())
        })
        .collect();

    let parsed_updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| line.split(',').map(|tok| tok.parse().unwrap()).collect())
        .collect();

    let mut page_ordering: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (first, second) in parsed_rules {
        page_ordering
            .entry(first)
            .or_insert_with(HashSet::new)
            .insert(second);
    }

    return (parsed_updates, page_ordering);
}

fn solve_part_one(updates: Vec<Vec<i32>>, lookup: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut ans: i32 = 0;
    for line in updates {
        let result = line.is_sorted_by(|a, b| lookup.get(a).is_some_and(|res| res.contains(b)));
        if result {
            ans += line[line.len() / 2]
        };
    }
    return ans;
}

fn solve_part_two(updates: Vec<Vec<i32>>, lookup: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut ans: i32 = 0;
    for mut line in updates {
        if line.is_sorted_by(|a, b| lookup.get(a).is_some_and(|res| res.contains(b))) {
            continue;
        }
        line.sort_unstable_by(|a, b| {
            if a == b {
                return Ordering::Equal;
            };
            if lookup.get(a).is_some_and(|res| res.contains(b)) {
                return Ordering::Less;
            };
            if lookup.get(b).is_some_and(|res| res.contains(a)) {
                return Ordering::Greater;
            };
            panic!("Incomparable pair of elemenets");
        });
        ans += line[line.len() / 2];
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let (page_orderings, rules) = parse("inputs/day05.txt");

        let part_one = solve_part_one(page_orderings.clone(), &rules);
        assert_eq!(part_one, 5208);

        let part_two = solve_part_two(page_orderings.clone(), &rules);
        assert_eq!(part_two, 6732);
    }
}
