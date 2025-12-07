use std::fmt::Display;

pub fn part_one(input: &str) -> impl Display {
    let mut result = 0;

    let mut number_rows = Vec::new();
    for line in input.as_bytes().split(|c| *c == b'\n') {
        if !matches!(line[0], b'*' | b'+') {
            number_rows.push(parse_numbers(line).collect::<Vec<_>>());
        } else {
            for (col, operator) in parse_operators(line).enumerate() {
                result += operator.execute(number_rows.iter().map(|v| v[col]));
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> impl Display {
    let mut result = 0;

    let input = input.as_bytes();
    let lines = input.split(|c| *c == b'\n').collect::<Vec<_>>();
    let operator_line = lines.last().unwrap();

    let mut numbers = Vec::new();
    for column in (0..lines[0].len()).rev() {
        let mut digits = ColumnIter::new(&lines[..lines.len() - 1], column)
            .filter(|c| matches!(*c, b'0'..=b'9'))
            .peekable();
        if digits.peek().is_none() {
            continue;
        }

        numbers.push(parse_number(digits));

        if let Some(op) = parse_operator(operator_line[column]) {
            result += op.execute(numbers.drain(..));
        }
    }

    result
}

struct ColumnIter<'a> {
    lines: &'a [&'a [u8]],
    line_index: usize,
    col_index: usize,
}

impl<'a> ColumnIter<'a> {
    pub fn new(lines: &'a [&'a [u8]], column: usize) -> Self {
        Self {
            lines,
            line_index: 0,
            col_index: column,
        }
    }
}

impl Iterator for ColumnIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.get(self.line_index)?;
        let item = line.get(self.col_index)?;
        self.line_index += 1;
        Some(*item)
    }
}

enum Operator {
    Mult,
    Add,
}

impl Operator {
    fn execute(&self, values: impl Iterator<Item = i64>) -> i64 {
        match self {
            Self::Add => values.sum::<i64>(),
            Self::Mult => values.fold(1, |accum, v| accum * v),
        }
    }
}

fn split_whitespace(seq: &[u8]) -> impl Iterator<Item = &'_ [u8]> {
    seq.split(|c| *c == b' ').filter(|seq| !seq.is_empty())
}

fn parse_number(bytes: impl Iterator<Item = u8>) -> i64 {
    let mut value = 0;
    for byte in bytes {
        value *= 10;
        value += (byte - b'0') as i64;
    }
    value
}

fn parse_numbers(line: &[u8]) -> impl Iterator<Item = i64> {
    split_whitespace(line).map(|seq| parse_number(seq.iter().copied()))
}

fn parse_operator(byte: u8) -> Option<Operator> {
    match byte {
        b'*' => Some(Operator::Mult),
        b'+' => Some(Operator::Add),
        _ => None,
    }
}

fn parse_operators(line: &[u8]) -> impl Iterator<Item = Operator> {
    split_whitespace(line).flat_map(|seq| parse_operator(seq[0]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn six_p1() {
        let input = include_str!("../../../inputs/example.txt");
        assert_eq!("4277556", part_one(input).to_string().as_str());
    }

    #[test]
    fn six_p2() {
        let input = include_str!("../../../inputs/example.txt");
        assert_eq!("3263827", part_two(input).to_string().as_str());
    }
}
