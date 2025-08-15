use std::fs;

const TEST: &str = "2333133121414131402";
fn solve_part_one(disk_map: &String) -> u64 {
    let mut checksum = 0;
    let mut disk: Vec<u64> = Vec::new();
    for (i, chunk) in disk_map.as_bytes().chunks(2).enumerate() {
        println!("{i}: {}", chunk as );
    }
    return 0;    
}

fn solve_part_two(disk_map: &String) -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        // let disk_map = fs::read_to_string("inputs/day09.txt").unwrap();
        let disk_map = TEST.to_owned(); 
        let _part_one = solve_part_one(&disk_map);
        let _part_two = solve_part_two(&disk_map);
    }
}
