pub const INPUT: &str = include_str!("input.txt");

pub enum Direction {
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

type Position = (isize, isize);

pub trait Keypad {
    fn start_position() -> Position;

    fn key_char(position: Position) -> Option<char>;

    fn press_direction(position: Position, direction: Direction) -> Position {
        let mut p = position;
        match direction {
            Direction::Up => {
                p.1 -= 1;
            }
            Direction::Down => {
                p.1 += 1;
            }
            Direction::Left => {
                p.0 -= 1;
            }
            Direction::Right => {
                p.0 += 1;
            }
        }
        if Self::key_char(p).is_some() {
            p
        } else {
            position
        }
    }

    fn apply_directions(directions: &str) -> String {
        let start = Self::start_position();
        directions
            .lines()
            .map(|l| {
                let position = l
                    .chars()
                    .fold(start, |pos, c| Self::press_direction(pos, c.into()));
                Self::key_char(position).unwrap()
            })
            .collect()
    }
}

pub struct BasicKeypad;

impl Keypad for BasicKeypad {
    fn start_position() -> Position {
        (1, 1)
    }

    fn key_char(position: Position) -> Option<char> {
        match position {
            (0, 0) => Some('1'),
            (1, 0) => Some('2'),
            (2, 0) => Some('3'),
            (0, 1) => Some('4'),
            (1, 1) => Some('5'),
            (2, 1) => Some('6'),
            (0, 2) => Some('7'),
            (1, 2) => Some('8'),
            (2, 2) => Some('9'),
            _ => None,
        }
    }
}

pub struct CrazyKeypad;

impl Keypad for CrazyKeypad {
    fn start_position() -> Position {
        (0, 2)
    }

    fn key_char(position: Position) -> Option<char> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            BasicKeypad::apply_directions(
                "ULL
RRDDD
LURDL
UUUUD",
            ),
            "1985"
        );
    }
}
