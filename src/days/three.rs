use std::fmt::Display;

fn digit_from_ascii(byte: u8) -> usize {
    (byte - b'0') as usize
}

fn max_in_range(range: &[u8]) -> (usize, u8) {
    let mut max_index = 0;
    let mut max_value = 0;

    for (i, &value) in range.iter().enumerate() {
        if value > max_value {
            max_index = i;
            max_value = value;
        }
    }

    (max_index, max_value)
}

fn max_n_digits(line: &[u8], digits: usize) -> usize {
    if digits == 1 {
        return digit_from_ascii(max_in_range(line).1);
    }

    let end_bound = line.len() - (digits - 1);

    let (max_index, max_value) = max_in_range(&line[..end_bound]);
    let rest = max_n_digits(&line[max_index + 1..], digits - 1);

    digit_from_ascii(max_value) * 10usize.pow(digits as u32 - 1) + rest
}

pub fn part_one(input: &str) -> impl Display {
    let bytes = input.as_bytes();

    bytes
        .split(|c| *c == b'\n')
        .map(|line| max_n_digits(line, 2))
        .sum::<usize>()
}

pub fn part_two(input: &str) -> impl Display {
    let bytes = input.as_bytes();

    bytes
        .split(|c| *c == b'\n')
        .map(|line| max_n_digits(line, 12))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bigg() {
        assert_eq!(max_n_digits(b"1234", 2), 34);
        assert_eq!(max_n_digits(b"12345", 3), 345);
        assert_eq!(max_n_digits(b"512345", 3), 545);
        assert_eq!(max_n_digits(b"512345", 4), 5345);
        assert_eq!(max_n_digits(b"512345", 4), 5345);
        assert_eq!(max_n_digits(b"1512345", 4), 5345);
        assert_eq!(max_n_digits(b"1515234", 4), 5534);
        assert_eq!(max_n_digits(b"54321", 4), 5432);
        assert_eq!(max_n_digits(b"515", 2), 55);
        assert_eq!(max_n_digits(b"655006", 3), 656);
    }
}
