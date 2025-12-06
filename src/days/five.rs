use core::ops::RangeInclusive;
use std::fmt::Display;

pub fn part_one(input: &str) -> impl Display {
    let input = input.as_bytes();

    let split = input
        .windows(2)
        .enumerate()
        .find_map(|(i, window)| (window == b"\n\n").then_some(i))
        .unwrap();

    let ranges = &input[..split];
    let available_ids = &input[split + 2..];

    let ranges = parse_ranges(ranges);

    // NOTE: Collecting the values before counting
    // is actually about 5% faster.
    available_ids
        .split(|c| *c == b'\n')
        .map(parse_ascii_number)
        .filter(|id| ranges.iter().any(|(l, h)| (*l..=*h).contains(&id)))
        .count()
}

pub fn part_two(input: &str) -> impl Display {
    let mut ranges = parse_ranges(input.as_bytes());

    let mut unique_fresh = 0;
    let mut observed: Vec<RangeInclusive<_>> = Vec::new();
    let mut i = 0;
    'outer: while i < ranges.len() {
        let (l, h) = ranges[i];
        let mut range = l..=h;

        loop {
            if observed.iter().any(|r| range.subset_of(r)) {
                i += 1;
                continue 'outer;
            }

            let mut has_changed = false;

            for observed in &observed {
                if observed.subset_of(&range) {
                    // Here, we split the range in two since an already-counted
                    // range is a subset.
                    if range.end() != observed.end() {
                        ranges.push((*observed.end() + 1, *range.end()));
                    }

                    range = *range.start()..=(observed.start() - 1);
                    has_changed = true;
                } else if observed.contains(&range.start()) {
                    range = (observed.end() + 1)..=*range.end();
                    has_changed = true;
                } else if observed.contains(&range.end()) {
                    range = *range.start()..=(observed.start() - 1);
                    has_changed = true;
                }
            }

            if !has_changed {
                break;
            }
        }

        unique_fresh += range.clone().count();
        observed.push(range);
        i += 1;
    }

    unique_fresh
}

type Range = RangeInclusive<u64>;

trait SubsetOf {
    fn subset_of(&self, other: &Self) -> bool;
}

impl SubsetOf for Range {
    fn subset_of(&self, other: &Self) -> bool {
        other != self && other.contains(self.start()) && other.contains(self.end())
    }
}

fn parse_ascii_number(number: &[u8]) -> u64 {
    let mut value = 0;
    for byte in number {
        value *= 10;
        value += (byte - b'0') as u64;
    }
    value
}

fn parse_ranges(ranges: &[u8]) -> Vec<(u64, u64)> {
    ranges
        .split(|c| *c == b'\n')
        .map_while(|range| {
            let mut pair = range.split(|c| *c == b'-');

            let left = parse_ascii_number(pair.next()?);
            let right = parse_ascii_number(pair.next()?);

            Some((left, right))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_five() {
        let test = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!("3", part_one(test).to_string().as_str());
    }

    #[test]
    fn test_five_two() {
        let test = "3-5\n10-14\n16-20\n12-18\n\n";
        assert_eq!("14", part_two(test).to_string().as_str());
    }

    #[test]
    fn test_five_2() {
        let test = "5-10\n4-11\n6-9\n\n";
        assert_eq!("8", part_two(test).to_string().as_str());
    }
}
