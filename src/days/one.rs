use std::fmt::Display;

enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    fn iter_sides(input: &str) -> impl Iterator<Item = Rotation> {
        input.lines().flat_map(|l| match l.bytes().next()? {
            b'L' => Some(Self::Left(l[1..].parse().ok()?)),
            b'R' => Some(Self::Right(l[1..].parse().ok()?)),
            _ => None,
        })
    }
}

struct Lock {
    tick: i32,
    zeroes: u32,
}

impl Default for Lock {
    fn default() -> Self {
        Self {
            tick: 50,
            zeroes: 0,
        }
    }
}

impl Lock {
    fn right(&mut self, value: u32) {
        self.tick = (self.tick + value as i32) % 100;
        if self.tick == 0 {
            self.zeroes += 1;
        }
    }

    fn left(&mut self, value: u32) {
        self.tick = (self.tick - value as i32).rem_euclid(100);
        if self.tick == 0 {
            self.zeroes += 1;
        }
    }
}

pub fn part_one(input: &str) -> impl Display {
    let mut lock = Lock::default();

    for rotation in Rotation::iter_sides(input) {
        match rotation {
            Rotation::Left(amount) => lock.left(amount),
            Rotation::Right(amount) => lock.right(amount),
        }
    }

    lock.zeroes
}

impl Lock {
    fn right_count_travel(&mut self, value: u32) {
        let crossings = (self.tick as u32 + value) / 100;
        self.zeroes += crossings;

        self.tick = (self.tick + value as i32) % 100;
    }

    fn left_count_travel(&mut self, value: u32) {
        // The extra mod 100 removes miscounts when starting at zero.
        let start = (100 - self.tick as u32) % 100;
        let crossings = (start + value) / 100;
        self.zeroes += crossings;

        self.tick = (self.tick - value as i32).rem_euclid(100);
    }
}

pub fn part_two(input: &str) -> impl Display {
    let mut lock = Lock::default();

    for rotation in Rotation::iter_sides(input) {
        match rotation {
            Rotation::Left(amount) => lock.left_count_travel(amount),
            Rotation::Right(amount) => lock.right_count_travel(amount),
        }
    }

    lock.zeroes
}
