use std::io::Error as IoError;

mod util;
mod day02;

use day02::{ Direction };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = util::read_lines("data/day2-1.txt")?;
    let position = Position1::run(lines);
    println!("Final position: {:?}", position);
    println!("Answer {}", position.x * (-position.y));
    Ok(())
}

#[derive(Debug)]
struct Position1 {
    pub x: i32,
    pub y: i32,
}

impl Position1 {
    pub fn new() -> Self {
        Position1 { x: 0, y: 0 }
    }

    pub fn update(self, dir: Direction) -> Self {
        match dir {
            Direction::Forward(i) => { Position1 { x: self.x + i, y: self.y } },
            Direction::Up(i) => { Position1 { x: self.x, y: self.y + i } },
            Direction::Down(i) => { Position1 { x: self.x, y: self.y - i } },
        }
    }

    pub fn run<I>(lines: I) -> Position1
        where I: Iterator<Item=Result<String, IoError>>
    {
        lines
            .filter_map(|line| line.ok().and_then(|l| if l.is_empty() { None } else { Some(l) }))
            .map(|l| l.parse::<Direction>().unwrap())
            .fold(Position1::new(), |pos, dir|  pos.update(dir))
    }

}
