pub const INPUT: &str = include_str!("input.txt");

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            c => panic!("bad move character: {c}"),
        }
    }
}

pub struct Keypad {
    position: (usize, usize),
}

impl Keypad {
    pub fn new() -> Self {
        Self { position: (1, 1) }
    }

    fn number(&self) -> usize {
        match self.position {
            (0, 0) => 1,
            (1, 0) => 2,
            (2, 0) => 3,
            (0, 1) => 4,
            (1, 1) => 5,
            (2, 1) => 6,
            (0, 2) => 7,
            (1, 2) => 8,
            (2, 2) => 9,
            p => unreachable!("bad key position: {:?}", p),
        }
    }

    fn press_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.position.1 > 0 {
                    self.position.1 -= 1;
                }
            }

            Direction::Down => {
                if self.position.1 < 2 {
                    self.position.1 += 1;
                }
            }
            Direction::Left => {
                if self.position.0 > 0 {
                    self.position.0 -= 1;
                }
            }
            Direction::Right => {
                if self.position.0 < 2 {
                    self.position.0 += 1;
                }
            }
        }
    }

    pub fn apply_directions(&mut self, directions: &str) -> Vec<usize> {
        directions
            .lines()
            .map(|l| {
                l.chars().for_each(|c| self.press_direction(c.into()));
                self.number()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut keypad = Keypad::new();
        assert_eq!(
            keypad.apply_directions(
                "ULL
RRDDD
LURDL
UUUUD",
            ),
            [1, 9, 8, 5]
        );
    }
}
