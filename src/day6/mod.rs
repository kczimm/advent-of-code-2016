use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy)]
pub enum Method {
    MostLikely,
    LeastLikely,
}

#[derive(Default)]
pub struct MessageDecoder {
    count: Vec<HashMap<char, usize>>,
}

impl MessageDecoder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn process(&mut self, input: &str) {
        for line in input.lines() {
            for (i, c) in line.chars().enumerate() {
                if i >= self.count.len() {
                    self.count.push(HashMap::new());
                }
                let count = self.count.get_mut(i).unwrap();
                let entry = count.entry(c).or_insert(0);
                *entry += 1;
            }
        }
    }

    pub fn error_corrected_message(&self, method: Method) -> String {
        let mut msg = String::new();
        for m in &self.count {
            let mut counts = m
                .iter()
                .map(|i| (*i.0, *i.1))
                .collect::<Vec<(char, usize)>>();
            match method {
                Method::MostLikely => {
                    counts.sort_by(|a, b| b.1.cmp(&a.1));
                }
                Method::LeastLikely => {
                    counts.sort_by(|a, b| a.1.cmp(&b.1));
                }
            }
            msg.push(counts[0].0);
        }
        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn example() {
        let mut md = MessageDecoder::new();
        md.process(INPUT);
        assert_eq!(md.error_corrected_message(Method::MostLikely), "easter");
        assert_eq!(md.error_corrected_message(Method::LeastLikely), "advent");
    }
}
