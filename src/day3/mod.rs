pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct Triangle(usize, usize, usize);

impl Triangle {
    fn is_possible(&self) -> bool {
        (self.0 + self.1 > self.2) && (self.1 + self.2 > self.0) && (self.0 + self.2 > self.1)
    }
}

impl From<&str> for Triangle {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        let c = parts.next().unwrap().parse().unwrap();

        Self(a, b, c)
    }
}

fn into_vertical_triangles(input: &str) -> Vec<Triangle> {
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|part| part.parse().unwrap())
        .collect();

    let mut triangles = Vec::with_capacity(numbers.len() / 3);

    let mut i = 0;
    let mut count = 0;
    for _ in 0..triangles.capacity() {
        let a = numbers[i];
        let b = numbers[i + 3];
        let c = numbers[i + 6];
        triangles.push(Triangle(a, b, c));

        count += 1;
        if count == 3 {
            count = 0;
            i += 7;
        } else {
            i += 1;
        }
    }

    triangles
}

pub fn num_possible_vertical_triangles(input: &str) -> usize {
    into_vertical_triangles(input)
        .iter()
        .filter(|&t| t.is_possible())
        .count()
}

pub fn num_possible_triangles(input: &str) -> usize {
    input
        .lines()
        .filter(|s| Triangle::from(*s).is_possible())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_triangle() {
        assert!(!Triangle::from("5 10 25").is_possible());
    }

    #[test]
    fn vertical_triangles() {
        assert_eq!(
            num_possible_vertical_triangles(
                "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603",
            ),
            6
        );
    }
}
