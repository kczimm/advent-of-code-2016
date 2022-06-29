pub const INPUT: &str = include_str!("input.txt");

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
}
