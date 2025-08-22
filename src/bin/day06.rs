use std::fs;
use std::collections::HashSet;

fn parse(path: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect())
        .collect()
}

fn find_start_pos(mat: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == b'^' {
                return Some((i, j));
            };
        }
    }
    return None;
}

fn check_bounds(pos: (usize, usize), mat: &Vec<Vec<u8>>) -> bool {
    let (row, col) = pos;
    return (row > 0) && (row + 1 < mat.len()) && (col > 0) && (col + 1 < mat[row].len());
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn step(pos: &(usize, usize), dir: &Direction) -> (usize, usize) {
    let (x, y) = *pos; 
    return match dir {
        Direction::North => (x - 1, y),
        Direction::East => (x, y + 1),
        Direction::South => (x + 1, y),
        Direction::West => (x, y - 1),
    };
}

fn turn_right(dir: &Direction) -> Direction {
    return match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    };
}

fn walk(pos: &mut (usize, usize), dir: &mut Direction, mat: &Vec<Vec<u8>>) {
    let next_pos = step(pos, dir);
    if mat[next_pos.0][next_pos.1] == b'#' {
        *dir = turn_right(dir);
    } else {
        *pos = next_pos; 
    }
}

fn grid_walker(mat: &Vec<Vec<u8>>, mut pos: (usize, usize), mut dir: Direction) -> HashSet<(usize, usize)> {
    // Hash Hash set is ok here
    let mut walked_path: HashSet<(usize, usize)> = HashSet::from([pos]);
    while check_bounds(pos, &mat) {
        walk(&mut pos, &mut dir, &mat);
        walked_path.insert(pos);
    }
    return walked_path;
}

fn loopy_walker(mat: &Vec<Vec<u8>>, mut pos: (usize, usize), mut dir: Direction) -> bool {
    // TODO Don't use a hash set
    let mut walked_path: HashSet<((usize, usize), Direction)> = HashSet::new();
    while check_bounds(pos, &mat) {
        if walked_path.contains(&(pos, dir)) {
            return true;
        }
        walked_path.insert((pos, dir));
        walk(&mut pos, &mut dir, &mat);
    }
    return false;
}

fn solve_part_one(mat: Vec<Vec<u8>>, pos: (usize, usize), dir: Direction) -> usize {
   grid_walker(&mat, pos, dir).len() 
}

fn solve_part_two(mut mat: Vec<Vec<u8>>, pos: (usize, usize), dir: Direction) -> usize {
    let mut ans: usize = 0;
    for (x, y) in grid_walker(&mat, pos, dir) {
        let tmp = mat[x][y];
        mat[x][y] = b'#';
        if loopy_walker(&mat, pos, dir) {
            ans += 1;
        }
        mat[x][y] = tmp;
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_part_one() {
        let grid = parse("inputs/day06.txt");
        let start_pos = find_start_pos(&grid).unwrap();

        let part_one = solve_part_one(grid.clone(), start_pos, Direction::North);
        assert_eq!(part_one, 5030);
    }

    #[test]
    fn solution_part_two() {
        let grid = parse("inputs/day06.txt");
        let start_pos = find_start_pos(&grid).unwrap();

        let part_two = solve_part_two(grid.clone(), start_pos, Direction::North);
        assert_eq!(part_two, 1928);
    }
}
