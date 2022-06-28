pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Right(isize),
    Left(isize),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let direction = chars.next().expect("no direction");
        let steps = chars.as_str().parse().expect("parse steps failed");
        match direction {
            'R' => Instruction::Right(steps),
            'L' => Instruction::Left(steps),
            d => panic!("bad direction: {d}"),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Simple;
pub struct TwiceFinder;

pub struct Walker<Type = Simple> {
    direction: Direction,
    position: (isize, isize),
    _marker: std::marker::PhantomData<Type>,
}

impl<Type> Walker<Type> {
    pub fn new() -> Self {
        Self {
            direction: Direction::North,
            position: (0, 0),
            _marker: std::marker::PhantomData,
        }
    }

    fn step(&mut self, direction: Direction, amount: isize) {
        match direction {
            Direction::North => self.position.1 += amount,
            Direction::East => self.position.0 += amount,
            Direction::South => self.position.1 -= amount,
            Direction::West => self.position.0 -= amount,
        }
        self.direction = direction;
    }

    fn distance(&self) -> isize {
        self.position.0.abs() + self.position.1.abs()
    }
}

impl Walker<Simple> {
    pub fn walk(&mut self, instructions: &str) -> isize {
        instructions
            .split(", ")
            .into_iter()
            .for_each(|instr| match instr.into() {
                Instruction::Right(amount) => {
                    self.step(
                        match self.direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        },
                        amount,
                    );
                }
                Instruction::Left(amount) => {
                    self.step(
                        match self.direction {
                            Direction::North => Direction::West,
                            Direction::East => Direction::North,
                            Direction::South => Direction::East,
                            Direction::West => Direction::South,
                        },
                        amount,
                    );
                }
            });
        self.distance()
    }
}

impl Walker<TwiceFinder> {
    pub fn walk(&mut self, instructions: &str) -> isize {
        let mut have_been = Vec::new();

        for instr in instructions.split(", ") {
            match instr.into() {
                Instruction::Right(amount) => {
                    let direction = match self.direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    };

                    for _ in 0..amount {
                        self.step(direction, 1);
                        if have_been.contains(&self.position) {
                            return self.distance();
                        } else {
                            have_been.push(self.position);
                        }
                    }
                }

                Instruction::Left(amount) => {
                    let direction = match self.direction {
                        Direction::North => Direction::West,
                        Direction::East => Direction::North,
                        Direction::South => Direction::East,
                        Direction::West => Direction::South,
                    };

                    for _ in 0..amount {
                        self.step(direction, 1);
                        if have_been.contains(&self.position) {
                            return self.distance();
                        } else {
                            have_been.push(self.position);
                        }
                    }
                }
            }
        }
        self.distance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_str() {
        let instr: Instruction = "R1".into();
        assert_eq!(instr, Instruction::Right(1));

        let instr: Instruction = "L100".into();
        assert_eq!(instr, Instruction::Left(100));
    }

    #[test]
    fn example() {
        assert_eq!(Walker::<Simple>::new().walk("R2, L3"), 5);
        assert_eq!(Walker::<Simple>::new().walk("R2, R2, R2"), 2);
        assert_eq!(Walker::<Simple>::new().walk("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn example_twice_finder() {
        assert_eq!(Walker::<TwiceFinder>::new().walk("R8, R4, R4, R8"), 4);
    }
}
