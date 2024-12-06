use std::fs;

fn find_start(matrix: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    let rows = matrix.len() - 1;
    let cols = matrix[0].len() - 1;
    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == b'^' { return Some((i, j)); }
        }
    }
    return None;
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<u8>>) {
    for row in matrix {
        for b in row {
            print!("{}",*b as char);
        }
        println!();
    }
}

fn walk(matrix: &mut Vec<Vec<u8>>, start_row: usize, start_col: usize) {
    let mut i = start_row;
    let mut j = start_col;
    let rows = matrix.len() - 1;
    let cols = matrix[0].len() - 1;

    matrix[i][j] = b'X';
    loop {
        // up
        while i > 0 && matrix[i-1][j] != b'#' {
            i -= 1;
            matrix[i][j] = b'X';
        }
        if i == 0 { return; }

        // right
        while j < cols && matrix[i][j+1] != b'#' {
            j += 1;
            matrix[i][j] = b'X';
        }
        if j == cols { return; }

        // down 
        while i < rows && matrix[i+1][j] != b'#' {
            i += 1;
            matrix[i][j] = b'X';
        }
        if i == rows { return; }

        // left
        while j > 0 && matrix[i][j-1] != b'#' {
            j -= 1;
            matrix[i][j] = b'X';
        }
        if j == 0 { return; }
    }
}

fn count(matrix: &Vec<Vec<u8>>) -> i32 {
    let mut count: i32 = 0;
    for row in matrix {
        for b in row {
            if *b == b'X' { count += 1; }
        }
    }
    return count;
}

fn solve(filename: &str) -> i32 {
    let text = fs::read_to_string(filename).expect("File not found");
    let mut matrix: Vec<Vec<u8>> = text.lines().map(|line| line.as_bytes().into()).collect();
    if let Some(start) = find_start(&matrix) {
        walk(&mut matrix, start.0, start.1);
        return count(&matrix);
    } else {
        panic!("Couldn't find start");
    }
}

fn main() {
    assert_eq!(solve("input2"), 41);
    println!("Solution: {}", solve("input"));
}
