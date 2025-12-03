use std::fmt::Display;

fn parse_ranges(input: &str) -> impl Iterator<Item = (usize, usize)> {
    input.split(',').flat_map(|range| {
        let mut pair = range.split('-');
        let left = pair.next()?.parse().ok()?;
        let right = pair.next()?.parse().ok()?;

        Some((left, right))
    })
}

/// Push the ascii digits of an integer to a buffer in reverse order.
fn write_digits(mut id: usize, buffer: &mut Vec<u8>) {
    if id == 0 {
        buffer.push(b'0');
        return;
    }

    while id > 0 {
        // don't matter if it backwards
        let digit = id % 10;
        id /= 10;
        buffer.push(b'0' + digit as u8);
    }
}

fn is_invalid_id(id: usize, buffer: &mut Vec<u8>) -> bool {
    write_digits(id, buffer);

    let digits = buffer;

    if digits.is_empty() {
        return false;
    }

    // odd numbers can never be invalid
    if (digits.len() & 1) == 1 {
        return false;
    }

    let half = digits.len() / 2;

    digits[..half] == digits[half..]
}

pub fn part_one_simple(input: &str) -> impl Display {
    let mut sum = 0;
    let mut buffer = Vec::with_capacity(16);

    for (start, end) in parse_ranges(input) {
        for i in start..=end {
            if is_invalid_id(i, &mut buffer) {
                sum += i;
            }
            buffer.clear();
        }
    }

    sum
}

pub fn part_one(input: &str) -> impl Display {
    let (tx, rx) = std::sync::mpsc::sync_channel(16);

    for (start, end) in parse_ranges(input) {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut buffer = Vec::with_capacity(16);
            let mut sum = 0;

            for i in start..=end {
                if is_invalid_id(i, &mut buffer) {
                    sum += i;
                }
                buffer.clear();
            }

            tx.send(sum).unwrap();
        });
    }

    // we'll use the hangup as a termination signal
    drop(tx);

    rx.iter().sum::<usize>()
}

fn is_invalid_id2(id: usize, buffer: &mut Vec<u8>) -> bool {
    write_digits(id, buffer);
    let digits = buffer;

    if digits.is_empty() {
        return false;
    }

    // simply scan over the whole thing
    for window in 1..=digits.len() / 2 {
        // we can skip some checks
        if (digits.len() % window) != 0 {
            continue;
        }

        if digits
            .chunks(window)
            .skip(1)
            .all(|chunk| chunk == &digits[..window])
        {
            return true;
        }
    }

    false
}

pub fn part_two_simple(input: &str) -> impl Display {
    let mut sum = 0;
    let mut buffer = Vec::with_capacity(16);

    for (start, end) in parse_ranges(input) {
        for i in start..=end {
            if is_invalid_id2(i, &mut buffer) {
                sum += i;
            }
            buffer.clear();
        }
    }

    sum
}

pub fn part_two(input: &str) -> impl Display {
    let (tx, rx) = std::sync::mpsc::sync_channel(16);

    // We could probably balance these (we're probably waiting on
    // one thread much longer than the rest), but I can't be bothered.
    for (start, end) in parse_ranges(input) {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut buffer = Vec::with_capacity(16);
            let mut sum = 0;

            for i in start..=end {
                if is_invalid_id2(i, &mut buffer) {
                    sum += i;
                }
                buffer.clear();
            }

            tx.send(sum).unwrap();
        });
    }

    drop(tx);

    rx.iter().sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_invalid() {
        let mut buffer = Vec::new();

        assert!(is_invalid_id(11, &mut buffer));
        buffer.clear();
        assert!(!is_invalid_id(31, &mut buffer));
        buffer.clear();
        assert!(!is_invalid_id(301, &mut buffer));
        buffer.clear();
        assert!(is_invalid_id(301301, &mut buffer));
    }

    #[test]
    fn test_invalid2() {
        let mut buffer = Vec::new();

        assert!(is_invalid_id2(11, &mut buffer));
        buffer.clear();
        assert!(!is_invalid_id2(31, &mut buffer));
        buffer.clear();
        assert!(!is_invalid_id2(301, &mut buffer));
        buffer.clear();
        assert!(is_invalid_id2(301301, &mut buffer));
        buffer.clear();
        assert!(is_invalid_id2(301301301, &mut buffer));
        buffer.clear();
        assert!(is_invalid_id2(1111, &mut buffer));
        buffer.clear();
        assert!(!is_invalid_id2(1112, &mut buffer));
    }
}
