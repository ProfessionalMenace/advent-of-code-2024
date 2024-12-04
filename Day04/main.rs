use std::fs;

fn finder(arr: &Vec<u8>, word: &str) -> i32 {
    let bytes = word.as_bytes();
    let mut ans: i32 = 0;
    let mut i: usize = 0;
    //println!("{:?}", word);
    for &b in arr {
        //print!("{}", b as char);
        if b == bytes[i] {
            i += 1;
        } else if b == bytes[0] {
            i = 1;
        } else {
            i = 0;
        }
        if i == 4 {
            //print!("!");
            ans += 1;
            i = 0;
        }
    }
    //println!();
    return ans;
}

fn solve(vec: &Vec<u8>) -> i32 {
    finder(&vec, "XMAS") + finder(&vec, "SAMX")
}

fn solve_verticals(matrix: &Vec<Vec<u8>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut columns: Vec<u8> = Vec::new();

    for col in 0..n {
        for row in 0..m {
            columns.push(matrix[row][col]);
        }
        columns.push(b'\n');
    }
    return solve(&columns);
}

fn solve_diagonals(matrix: &Vec<Vec<u8>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut diagonals: Vec<u8> = Vec::new();
    // top right 
    for i in 0..n {
        let mut row = 0;
        let mut col = i;
        while row < m && col < n {
            diagonals.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(b'\n');
    }
    // bottom left
    for i in 1..m {
        let mut row = i;
        let mut col = 0;
        while row < m && col < n {
            diagonals.push(matrix[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(b'\n');
    }
    return solve(&diagonals);
}

fn solve_rdiagonals(matrix: &Vec<Vec<u8>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut diagonals: Vec<u8> = Vec::new();
    // top left
    for i in 1..n {
        let mut row = 0;
        let mut col = n - i + 1;
        while row < m && col > 0 {
            col -= 1;
            diagonals.push(matrix[row][col]);
            row += 1;
        }
        diagonals.push(b'\n');
    }

    // bottom right
    for i in 1..m {
        let mut row = i;
        let mut col = n;
        while row < m && col > 0 {
            col -= 1;
            diagonals.push(matrix[row][col]);
            row += 1;
        }
        diagonals.push(b'\n');
    }
    return solve(&diagonals);
}

fn main() {
    let text: String = fs::read_to_string("input").unwrap();
    let matrix: Vec<Vec<u8>> = text.lines().map(|line| line.as_bytes().into()).collect();

    let mut ans: i32 = 0;
    for v in &matrix {
        ans += solve(v);
    }
    println!("horizontal: {}", ans);
    ans += solve_verticals(&matrix);
    println!("vertical: {}", ans);
    ans += solve_diagonals(&matrix);
    println!("diagonal: {}", ans);
    ans += solve_rdiagonals(&matrix);
    println!("rdiagonal: {}", ans);
}
