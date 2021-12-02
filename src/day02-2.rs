
mod util;
mod day02;

use day02::{ Direction, Position };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::read_lines("data/day2-1.txt")?;
    let position = day02::Position::run(lines);
    println!("Final position: {:?}", position);
    println!("Answer {}", position.x * (-position.y));
    Ok(())
}

