use std::fmt::Display;

#[derive(Debug)]
enum Operator {
    Mult,
    Add,
}

enum Row {
    Values(Vec<i64>),
    Operators(Vec<Operator>),
}

fn advance_by(input: &mut &[u8], bytes: usize) {
    *input = &mut &input[bytes..];
}

fn parse_ascii_number(number: &mut &[u8]) -> i64 {
    let mut value = 0;

    while matches!(number[0], b'0'..=b'9') {
        value *= 10;
        value += (number[0] - b'0') as i64;
        advance_by(number, 1);
        if number.is_empty() {
            break;
        }
    }

    value
}

fn parse_number(input: &mut &[u8]) -> Option<i64> {
    if input.is_empty() {
        return None;
    }

    while input[0] == b' ' {
        advance_by(input, 1);
        if input.is_empty() {
            return None;
        }
    }

    Some(parse_ascii_number(input))
}

fn parse_operator(input: &mut &[u8]) -> Option<Operator> {
    while input[0] == b' ' {
        advance_by(input, 1);
        if input.is_empty() {
            return None;
        }
    }

    let operator = match input[0] {
        b'+' => {
            advance_by(input, 1);
            Operator::Add
        }
        b'*' => {
            advance_by(input, 1);
            Operator::Mult
        }
        _ => return None,
    };

    Some(operator)
}

fn parse_line(mut input: &[u8]) -> Row {
    // probe the line
    let is_values = input
        .iter()
        .find_map(|c| match *c {
            b'*' | b'+' => Some(false),
            b'0'..b'9' => Some(true),
            _ => None,
        })
        .unwrap();

    if is_values {
        let mut values = Vec::new();
        let input = &mut input;
        while let Some(number) = parse_number(input) {
            values.push(number)
        }
        Row::Values(values)
    } else {
        let mut ops = Vec::new();
        let input = &mut input;
        while let Some(number) = parse_operator(input) {
            ops.push(number)
        }
        Row::Operators(ops)
    }
}

pub fn part_one(input: &str) -> impl Display {
    let mut values = Vec::new();
    let mut operators = Vec::new();
    for line in input.as_bytes().split(|c| *c == b'\n').map(parse_line) {
        match line {
            Row::Values(v) => values.push(v),
            Row::Operators(mut o) => {
                operators.append(&mut o);
            }
        }
    }

    let mut total = 0;
    for col in 0..values[0].len() {
        match operators[col] {
            Operator::Add => {
                let mut value = 0;
                for j in 0..values.len() {
                    value += values[j][col];
                }
                total += value;
            }
            Operator::Mult => {
                let mut value = 1;
                for j in 0..values.len() {
                    value *= values[j][col];
                }
                total += value;
            }
        }
    }

    total
}

#[derive(Debug)]
struct Column {
    range: (usize, usize),
    operator: Operator,
}

fn op(op: u8) -> Option<Operator> {
    match op {
        b'*' => Some(Operator::Mult),
        b'+' => Some(Operator::Add),
        _ => None,
    }
}

// what was even the point of all the above???
fn column_spans(input: &[u8]) -> (usize, Vec<Column>) {
    let mut columns = Vec::new();

    // find the first operator
    let op_index = input
        .iter()
        .position(|c| matches!(*c, b'*' | b'+'))
        .unwrap();

    let mut last_index = 0;
    let mut current_op = op(input[op_index]).unwrap();
    for i in op_index + 1..input.len() {
        if let Some(op) = op(input[i]) {
            columns.push(Column {
                range: (last_index, i - op_index),
                operator: current_op,
            });

            current_op = op;
            last_index = i - op_index;
        }
    }

    columns.push(Column {
        range: (last_index, input.len() - op_index),
        operator: current_op,
    });

    (op_index, columns)
}

pub fn part_two(input: &str) -> impl Display {
    let input = input.as_bytes();

    let (op_start, columns) = column_spans(input);
    let lines = input[..op_start - 1]
        .split(|c| *c == b'\n')
        .collect::<Vec<_>>();

    let mut total = 0;
    for Column {
        range: (start, end),
        operator,
    } in columns
    {
        let mut values = Vec::new();
        // scan rows in reverse order
        for col in (start..end).rev() {
            let mut number: i64 = 0;
            let mut any = false;

            for line in &lines {
                if matches!(line[col], b'0'..=b'9') {
                    number *= 10;
                    number += (line[col] - b'0') as i64;
                    any = true;
                }
            }

            if any {
                values.push(number);
            }
        }

        match operator {
            Operator::Add => {
                total += values.into_iter().sum::<i64>();
            }
            Operator::Mult => {
                let mut mult = 1;
                for value in values {
                    mult *= value;
                }
                total += mult;
            }
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_six() {
        let input = include_str!("../../../inputs/example.txt");
        assert_eq!("4277556", part_one(input).to_string().as_str());
    }

    #[test]
    fn test_six_two() {
        let input = include_str!("../../../inputs/example.txt");
        assert_eq!("3263827", part_two(input).to_string().as_str());
    }
}
