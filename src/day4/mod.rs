use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
const REGEX: &str = r"^((?:[a-z]+)(?:-[a-z]+)+)-(\d{3})\[(\w{5})\]$";

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX).unwrap());

pub struct Room {
    encrypted_name: String,
    sector_id: usize,
    checksum: String,
}

impl Room {
    fn from(text: &str, re: &Regex) -> Self {
        let cap = re.captures(text).unwrap();

        Self {
            encrypted_name: String::from(&cap[1]),
            sector_id: (&cap[2]).parse().expect("sector id parsed"),
            checksum: String::from(&cap[3]),
        }
    }

    fn is_real(&self) -> bool {
        let mut letter_frequency = HashMap::new();
        self.encrypted_name
            .chars()
            .filter(|c| *c != '-')
            .for_each(|c| {
                *letter_frequency.entry(c).or_insert(0) += 1;
            });

        let mut letter_frequency: Vec<_> = letter_frequency.iter().collect();
        letter_frequency.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

        for (a, b) in letter_frequency[0..5]
            .iter()
            .map(|t| *t.0)
            .zip(self.checksum.chars())
        {
            if a != b {
                return false;
            }
        }
        true
    }

    fn decrypt_name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    shift_char(c, self.sector_id)
                }
            })
            .collect()
    }
}

pub fn sum_of_sector_ids_of_real_rooms(input: &str) -> usize {
    input
        .lines()
        .filter_map(|s| {
            let room = Room::from(s, &RE);
            if room.is_real() {
                Some(room.sector_id)
            } else {
                None
            }
        })
        .sum()
}

pub fn sector_id_of_northpole_object_store(input: &str) -> usize {
    input
        .lines()
        .filter_map(|s| {
            let room = Room::from(s, &RE);
            if room.is_real() {
                Some(room)
            } else {
                None
            }
        })
        .find(|r| r.decrypt_name() == "northpole object storage")
        .unwrap()
        .sector_id
}

fn shift_char(c: char, amount: usize) -> char {
    let a = 'a' as usize;
    let z = 'z' as usize;
    char::from_u32(((c as usize + amount - a) % (z - a + 1) + a) as _).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_cipher() {
        let room = Room {
            encrypted_name: "qzmt-zixmtkozy-ivhz".to_owned(),
            sector_id: 343,
            checksum: "".to_owned(),
        };

        assert_eq!(room.decrypt_name(), "very encrypted name");
    }

    #[test]
    fn test_room_from() {
        let room = Room::from("aaaaa-bbb-z-y-x-123[abxyz]", &RE);
        assert_eq!(room.encrypted_name, "aaaaa-bbb-z-y-x");
        assert_eq!(room.sector_id, 123);
        assert_eq!(room.checksum, "abxyz");
    }

    #[test]
    fn examples() {
        assert!(Room::from("aaaaa-bbb-z-y-x-123[abxyz]", &RE).is_real());
        assert!(Room::from("a-b-c-d-e-f-g-h-987[abcde]", &RE).is_real());
        assert!(Room::from("not-a-real-room-404[oarel]", &RE).is_real());
        assert!(!Room::from("totally-real-room-200[decoy]", &RE).is_real());
    }
}
