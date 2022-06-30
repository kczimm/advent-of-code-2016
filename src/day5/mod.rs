use md5;

pub struct PasswordCalculator<'a> {
    door_id: &'a str,
    iterations: usize,
    index: usize,
}

impl<'a> PasswordCalculator<'_> {
    const LENGTH: usize = 8;

    pub fn calculate(self) -> String {
        self.into_iter().collect()
    }
}

impl<'a> From<&'a str> for PasswordCalculator<'a> {
    fn from(door_id: &'a str) -> Self {
        Self {
            door_id: door_id,
            iterations: 0,
            index: 0,
        }
    }
}

impl<'a> Iterator for PasswordCalculator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations == Self::LENGTH {
            return None;
        } else {
            self.iterations += 1;
        }

        let c = loop {
            self.index += 1;
            let data = format!("{}{}", self.door_id, self.index);
            let digest = md5::compute(data.clone());
            let digest_hex = format!("{:x}", digest);
            if digest_hex.starts_with("00000") {
                break digest_hex.chars().nth(5).unwrap();
            }
        };

        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash() {
        let digest = md5::compute(format!("{}{}", "abc", 5017308));
        assert!(format!("{:x}", digest).starts_with("000008f82"));

        let digest = md5::compute(format!("{}{}", "abc", 3231929));
        assert!(format!("{:x}", digest).starts_with("00000"));
    }

    #[test]
    #[ignore]
    fn part() {
        assert_eq!(PasswordCalculator::from("abc").next(), Some('1'));
    }

    #[test]
    #[ignore]
    fn example() {
        assert_eq!(PasswordCalculator::from("abc").calculate(), "18f47a30");
    }
}
