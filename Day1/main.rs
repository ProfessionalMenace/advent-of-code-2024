use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "input";
    let mut file = match File::open(filename) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file,
    };

    let mut text = String::new();
    if let Err(why) = file.read_to_string(&mut text) {
        panic!("couldn't read file {}", why);
    };

    let lines: Vec<&str> = text.lines().flat_map(|line| line.split_whitespace()).collect();
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    let mut i: usize = 0;
    for line in lines {
        let parsed = match line.parse() {
        Ok(x) => x,
        Err(why) => panic!("couldn't parse a line: {}", why),
        };
        if i % 2 != 0 {
            left.push(parsed);
        }
        else {
            right.push(parsed);
        }
        i += 1;
    }

    left.sort();
    right.sort();

    let ans1: i64 = left.iter()
        .zip(right.iter())
        .map(|(&x, &y)| (x - y).abs())
        .sum(); 

   println!("{}", ans1);

   let mut curr: i64 = 0;
   let mut ans2: i64 = 0;
   let mut ansp: i64 = 0;

   for &l in &left {
      if curr == l {
         ans2 += ansp;
      } else {
         ansp = 0;
         curr = l; 
         for &r in &right {        
            if l == r {
               ansp += l;
            }
         }
         ans2 += ansp;
      }
   }

   println!("{}", ans2);
}
