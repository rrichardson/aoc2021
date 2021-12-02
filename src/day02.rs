use std::str::FromStr;
use std::io::Error as IoError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub aim: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0, aim: 0 }
    }

    fn update(self, dir: Direction) -> Self {
        dbg!(match dir {
            Direction::Forward(i) => { Position { x: self.x + i, y: self.y - (self.aim * i), aim: self.aim } },
            Direction::Up(i) => { Position { x: self.x, y: self.y, aim: self.aim - i } },
            Direction::Down(i) => { Position { x: self.x, y: self.y, aim: self.aim + i } },
        })
    }

    pub fn run<I>(lines: I) -> Position
        where I: Iterator<Item=Result<String, IoError>>
    {
        lines
            .filter_map(|line| line.ok().and_then(|l| if l.is_empty() { None } else { Some(l) }))
            .map(|l| l.parse::<Direction>().unwrap())
            .fold(Position::new(), |pos, dir|  pos.update(dir))
    }

}

pub enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Direction {
    type Err = DirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr: Vec<&str> =
            s.trim()
            .split(' ')
            .collect();
        let distance = instr[1].parse::<i32>()?;
        match instr[0] {
            "forward" => Ok(Direction::Forward(distance)),
            "down" => Ok(Direction::Down(distance)),
            "up" => Ok(Direction::Up(distance)),
            _ => Err(DirError::ParseStrError(format!("direction {} not understood", instr[0])))
        }
    }
}


#[derive(Error, Debug)]
pub enum DirError {
    #[error("Failed to parse int from instruction")]
    ParseIntError(#[from] ParseIntError),
    #[error("Failed to parse direction from instruction")]
    ParseStrError(String),
}


#[cfg(test)]
mod test {
    use super::Position;
    use std::io::Error as IoError;

    #[test]
    fn test1() {
        let dirs: Vec<Result<String, IoError>> = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
            .into_iter()
            .map(|s| Ok(String::from(s)))
            .collect();
        let position = Position::run(dirs.into_iter());
        assert_eq!(900, position.x * (-position.y));
    }
}

