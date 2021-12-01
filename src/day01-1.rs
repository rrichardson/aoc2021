use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("data/day1-1.txt")?;
    let depths = lines.flat_map(|v| dbg!(v.ok()?.parse::<i32>().ok())).collect();
    println!("{}", count_increases(&depths));
    Ok(())
}


fn count_increases(input: &Vec<i32>) -> usize {
    input.windows(2).filter(|a| a[0] < a[1]).count()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
