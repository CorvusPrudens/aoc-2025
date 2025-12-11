use std::{
    cmp::Reverse,
    fmt::Display,
    sync::{Arc, Mutex, atomic::AtomicUsize},
};

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn rect_from(a: Point, b: Point) -> (Point, Point) {
    let tl_x = a.x.min(b.x);
    let tl_y = a.y.min(b.y);

    let br_x = a.x.max(b.x);
    let br_y = a.y.max(b.y);

    (Point { x: tl_x, y: tl_y }, Point { x: br_x, y: br_y })
}

struct Frame {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![b'.'; width * height],
        }
    }

    fn get(&self, x: i64, y: i64) -> u8 {
        self.data[index(self.width, self.height, x, y).unwrap()]
    }

    fn get_mut(&mut self, x: i64, y: i64) -> &mut u8 {
        // let index = index(self.width, self.height, x, y);
        // if index.is_none() {
        //     println!(
        //         "x: {x}, y: {y}, width: {}, height: {}",
        //         self.width, self.height
        //     );
        // }
        &mut self.data[index(self.width, self.height, x, y).unwrap()]
    }
}

impl core::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;

            write!(
                f,
                "{}\n",
                std::str::from_utf8(&self.data[start..end]).unwrap()
            )?;
        }

        Ok(())
    }
}

fn index(width: usize, height: usize, x: i64, y: i64) -> Option<usize> {
    if !(0..width as i64).contains(&x) {
        return None;
    }
    if !(0..height as i64).contains(&y) {
        return None;
    }

    Some(x as usize + y as usize * width)
}

pub fn part_one(input: &str) -> impl Display {
    let corners = input
        .lines()
        .flat_map(|line| {
            let mut bits = line.split(',');

            Some(Point {
                x: bits.next()?.parse().ok()?,
                y: bits.next()?.parse().ok()?,
            })
        })
        .collect::<Vec<_>>();

    let mut largest = 0;
    for (i, a) in corners.iter().enumerate() {
        for b in corners.iter().skip(i + 1) {
            let (tl, br) = rect_from(*a, *b);

            let width = br.x - tl.x;
            let height = br.y - tl.y;

            let area = (width + 1) * (height + 1);

            if area > largest {
                largest = area;
            }
        }
    }

    largest
}

pub fn part_two(input: &str) -> impl Display {
    let mut corners = input
        .lines()
        .flat_map(|line| {
            let mut bits = line.split(',');

            Some(Point {
                x: bits.next()?.parse().ok()?,
                y: bits.next()?.parse().ok()?,
            })
        })
        .collect::<Vec<_>>();

    // process into minimum buffer
    let mut field_br = Point { x: 0, y: 0 };
    let mut field_tl = Point {
        x: i64::MAX,
        y: i64::MAX,
    };

    for point in &corners {
        if point.x < field_tl.x {
            field_tl.x = point.x;
        }
        if point.y < field_tl.y {
            field_tl.y = point.y;
        }

        if point.x > field_br.x {
            field_br.x = point.x;
        }
        if point.y > field_br.y {
            field_br.y = point.y;
        }
    }

    let width = (field_br.x - field_tl.x).max(0) as usize + 1;
    let height = (field_br.y - field_tl.y).max(0) as usize + 1;

    let mut frame = Frame::new(width, height);

    // massage coordinates into buffer's space
    for point in &mut corners {
        point.x -= field_tl.x;
        point.y -= field_tl.y;
    }

    // create lines
    let final_pair = [corners.last().copied().unwrap(), corners[0]];
    for pair in corners
        .windows(2)
        .chain(std::iter::once(final_pair.as_slice()))
    {
        let a = pair[0];
        let b = pair[1];

        let delta_x = b.x - a.x;
        if delta_x == 0 {
            let start = b.y.min(a.y);
            let end = b.y.max(a.y);
            for y in start..=end {
                *frame.get_mut(a.x, y) = b'X';
            }
        } else {
            let start = b.x.min(a.x);
            let end = b.x.max(a.x);
            for x in start..=end {
                *frame.get_mut(x, a.y) = b'X';
            }
        }
    }

    println!("created lines");

    // fill
    for (i, pair) in corners
        .windows(2)
        .chain(std::iter::once(final_pair.as_slice()))
        .enumerate()
    {
        let a = pair[0];
        let b = pair[1];

        // fill pct
        println!(
            "rasterizing {:.2}%",
            (i as f64 / corners.len() as f64) * 100.0
        );

        if a.x < b.x {
            for x in a.x..=b.x {
                for y in a.y + 1..(frame.height as i64 - 1) {
                    if frame.get(x, y) == b'X' {
                        break;
                    } else {
                        *frame.get_mut(x, y) = b'X';
                    }
                }
            }
        }
    }

    println!("finished rasterizing");

    // test
    let mut pairs = Vec::new();
    for (i, a) in corners.iter().enumerate() {
        for (j, b) in corners.iter().enumerate().skip(i + 1) {
            let (tl, br) = rect_from(*a, *b);

            let width = br.x - tl.x;
            let height = br.y - tl.y;

            let area = (width + 1) * (height + 1);

            pairs.push((area, i, j));
        }
    }
    pairs.sort_unstable_by_key(|p| Reverse(p.0));

    // find largest one entirely inside polygon
    let total = pairs.len() as f64;
    'outer: for (index, (area, i, j)) in pairs.into_iter().enumerate() {
        if (index % 1000) == 0 {
            println!("tested {:.2}%", (index as f64 / total) * 100.0);
        }

        let a = corners[i];
        let b = corners[j];

        let (tl, br) = rect_from(a, b);

        // we only need to check the bounding line segments
        for x in tl.x..=br.x {
            if frame.get(x, tl.y) != b'X' {
                continue 'outer;
            }

            if frame.get(x, br.y) != b'X' {
                continue 'outer;
            }
        }

        for y in tl.y..=br.y {
            if frame.get(tl.x, y) != b'X' {
                continue 'outer;
            }

            if frame.get(br.x, y) != b'X' {
                continue 'outer;
            }
        }

        return area;
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nine() {
        let input = include_str!("../../inputs/nine_test.txt");

        assert_eq!("50", part_one(input).to_string().as_str());
    }

    #[test]
    fn nine2() {
        let input = include_str!("../../inputs/nine_test.txt");

        assert_eq!("24", part_two(input).to_string().as_str());
    }
}
