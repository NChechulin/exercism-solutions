// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_delta_coordinates(&self) -> (i32, i32) {
        match *self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };

        self
    }

    pub fn turn_left(mut self) -> Self {
        for _ in 0..3 {
            self = self.turn_right();
        }

        self
    }

    pub fn advance(mut self) -> Self {
        let deltas = self.direction.get_delta_coordinates();
        self.x += deltas.0;
        self.y += deltas.1;
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for cmd in instructions.chars() {
            match cmd {
                'R' => self = self.turn_right(),
                'L' => self = self.turn_left(),
                'A' => self = self.advance(),
                _ => {}
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
