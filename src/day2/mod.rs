use std::marker::PhantomData;

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

pub struct Basic;
pub struct Crazy;

pub struct Keypad<Design = Basic> {
    position: (isize, isize),
    _marker: PhantomData<Design>,
}

impl<Design> Keypad<Design> {
    pub fn new() -> Self {
        Self {
            position: (1, 1),
            _marker: PhantomData,
        }
    }

    pub fn apply_directions(&mut self, directions: &str) -> String {
        directions
            .lines()
            .map(|l| {
                l.chars().for_each(|c| self.press_direction(c.into()));
                Self::key_char(self.position)
            })
            .collect()
    }
}

impl Keypad<Basic> {
    fn key_char(position: (isize, isize)) -> char {
        match position {
            (0, 0) => '1',
            (1, 0) => '2',
            (2, 0) => '3',
            (0, 1) => '4',
            (1, 1) => '5',
            (2, 1) => '6',
            (0, 2) => '7',
            (1, 2) => '8',
            (2, 2) => '9',
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
}

impl Keypad<Crazy> {
    fn key_char(position: (isize, isize)) -> Option<char> {
        match position {
            (2, 0) => Some('1'),
            (1, 1) => Some('2'),
            (2, 1) => Some('3'),
            (3, 1) => Some('4'),
            (0, 2) => Some('5'),
            (1, 2) => Some('6'),
            (2, 2) => Some('7'),
            (3, 2) => Some('8'),
            (4, 2) => Some('9'),
            (1, 3) => Some('A'),
            (2, 3) => Some('B'),
            (3, 3) => Some('C'),
            (2, 4) => Some('D'),
            _ => None,
        }
    }

    fn press_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                let mut position = self.position;
                position.1 -= 1;
                if let Some(position) = Self::<Crazy>::key_char
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
            "1985"
        );
    }
}
