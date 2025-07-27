//#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

macro_rules! solve {
    ($day:ident) => {
        $day::solve(concat!("../inputs/", stringify!($day), ".txt"));
    };
}

fn main() {
    solve!(day1);
    solve!(day2);
    solve!(day3);
    solve!(day4);
    solve!(day5);
}
