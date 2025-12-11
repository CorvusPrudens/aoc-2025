use std::fmt::Display;

pub fn part_one(input: &str) -> impl Display {
    42
}

pub fn part_two(input: &str) -> impl Display {
    42
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ten() {
        let input = include_str!("../../inputs/ten_test.txt");
        assert_eq!("42", part_one(input).to_string().as_str());
    }

    #[test]
    fn ten2() {
        let input = include_str!("../../inputs/ten_test.txt");
        assert_eq!("42", part_two(input).to_string().as_str());
    }
}
