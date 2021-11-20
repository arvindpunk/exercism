// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    dir: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot{
            x: x,
            y: y,
            dir: d
        }
    }

    pub fn turn_right(self) -> Self {
        Robot{
            x: self.x,
            y: self.y,
            dir: match self.dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }
    }

    pub fn turn_left(self) -> Self {
        Robot{
            x: self.x,
            y: self.y,
            dir: match self.dir {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            }
        }
    }

    pub fn advance(self) -> Self {
        let x = match self.dir {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0
        };
        let y = match self.dir {
            Direction::North => 1,
            Direction::South => -1,
            _ => 0
        };
        Robot{
            x: self.x + x,
            y: self.y + y,
            dir: self.dir
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |state, c| match c {
            'R' => state.turn_right(),
            'L' => state.turn_left(),
            'A' => state.advance(),
            _ => state
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
