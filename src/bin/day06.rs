use std::fs;
use std::collections::HashSet;

fn parse(path: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.bytes().collect())
        .collect()
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct GridWalker {
    pub row: usize,
    pub col: usize,
    pub dir: Direction,
}

impl GridWalker {
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn step_forward(&mut self) {
        match self.dir {
            Direction::North => self.row -= 1,
            Direction::East  => self.col += 1,
            Direction::South => self.row += 1,
            Direction::West  => self.col -= 1,
        }
    }
}

fn find_start_pos(mat: &Vec<Vec<u8>>) -> Option<GridWalker> {
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == b'^' {
                return Some(GridWalker{row: i, col: j, dir: Direction::North});
            };
        }
    }
    return None;
}

fn check_bounds(mat: &Vec<Vec<u8>>, walker: &GridWalker) -> bool {
    return (walker.row > 0) 
        && (walker.row + 1 < mat.len()) 
        && (walker.col > 0) 
        && (walker.col + 1 < mat[walker.row].len());
}

fn walk(mat: &Vec<Vec<u8>>, walker: &mut GridWalker) {
    let mut next_pos = walker.clone();
    next_pos.step_forward();
    if mat[next_pos.row][next_pos.col] == b'#' {
        walker.turn_right();
    } else {
        *walker = next_pos; 
    }
}

fn unique_pos(mat: &Vec<Vec<u8>>, mut walker: GridWalker) -> HashSet<(usize, usize)> {
    let mut walked_path: HashSet<(usize, usize)> = HashSet::new();
    walked_path.insert((walker.row, walker.col));
    while check_bounds(&mat, &walker) {
        walk(&mat, &mut walker);
        walked_path.insert((walker.row, walker.col));
    }
    return walked_path;
}

fn detect_loop(mat: &Vec<Vec<u8>>, mut walker: GridWalker) -> bool {
    let mut walked_path: HashSet<GridWalker> = HashSet::new();
    while check_bounds(&mat, &walker) {
        if walked_path.contains(&walker) {
            return true;
        }
        walked_path.insert(walker);
        walk(&mat, &mut walker);
    }
    return false;
}

fn solve_part_one(mat: Vec<Vec<u8>>, walker: GridWalker) -> usize {
   unique_pos(&mat, walker).len()
}

fn solve_part_two(mut mat: Vec<Vec<u8>>, walker: GridWalker) -> usize {
    let mut ans: usize = 0;
    for (x, y) in unique_pos(&mat, walker) {
        let tmp = mat[x][y];
        mat[x][y] = b'#';
        if detect_loop(&mat, walker) {
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

        let part_one = solve_part_one(grid.clone(), start_pos);
        assert_eq!(part_one, 5030);
    }

    #[test]
    fn solution_part_two() {
        let grid = parse("inputs/day06.txt");
        let start_pos = find_start_pos(&grid).unwrap();

        let part_two = solve_part_two(grid.clone(), start_pos);
        assert_eq!(part_two, 1928);
    }
}
