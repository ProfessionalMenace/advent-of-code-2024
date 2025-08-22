use std::fs;

fn parse(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn solve_part_one(mat: &Vec<Vec<char>>, word: &str) -> usize {
    let search: Vec<char> = word.chars().collect();
    let rows = mat.len();
    let cols = mat[0].len();
    let mut ans: usize = 0;

    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            if mat[row][col] != search[0] {
                continue;
            }
            for (di, dj) in &DIRECTIONS {
                let mut check = true;
                for k in 1..search.len() {
                    let i = (row as isize) + (k as isize) * di;
                    let j = (col as isize) + (k as isize) * dj;
                    let in_bounds: bool =
                        i >= 0 && i < (rows as isize) && j >= 0 && j < (cols as isize);
                    if !in_bounds || mat[i as usize][j as usize] != search[k] {
                        check = false;
                        break;
                    }
                }
                if check {
                    ans += 1;
                }
            }
        }
    }
    return ans;
}

fn solve_part_two(mat: &Vec<Vec<char>>) -> usize {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut ans: usize = 0;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            if mat[i][j] == 'A' {
                let check1 = mat[i - 1][j - 1] == 'M' && mat[i + 1][j + 1] == 'S';
                let check2 = mat[i - 1][j - 1] == 'S' && mat[i + 1][j + 1] == 'M';
                let check3 = mat[i - 1][j + 1] == 'M' && mat[i + 1][j - 1] == 'S';
                let check4 = mat[i - 1][j + 1] == 'S' && mat[i + 1][j - 1] == 'M';
                if (check1 || check2) && (check3 || check4) {
                    ans += 1;
                }
            }
        }
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_part_one() {
        let parsed_text = parse("inputs/day04.txt");

        let part_one = solve_part_one(&parsed_text, "XMAS");
        assert_eq!(part_one, 2458);
    }

    #[test]
    fn solution_part_two() {
        let parsed_text = parse("inputs/day04.txt");

        let part_two = solve_part_two(&parsed_text);
        assert_eq!(part_two, 1945);
    }
}
