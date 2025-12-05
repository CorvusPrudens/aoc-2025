use std::fmt::Display;

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(data: &[u8]) -> Self {
        let width = data.iter().position(|p| *p == b'\n').unwrap();
        let data: Vec<_> = data.iter().copied().filter(|c| *c != b'\n').collect();

        Self {
            width,
            height: data.len() / width,
            data,
        }
    }

    fn get(&self, x: i16, y: i16) -> Option<u8> {
        Some(self.data[index(self.width, self.height, x, y)?])
    }

    pub fn paper_around(&self, x: i16, y: i16) -> u8 {
        let mut total = 0;
        for j in (y - 1)..=(y + 1) {
            for i in (x - 1)..=(x + 1) {
                if i == x && j == y {
                    continue;
                }

                if let Some(b'@') = self.get(i, j) {
                    total += 1;
                }
            }
        }
        total
    }

    pub fn neighbor_list(&self) -> Vec<u8> {
        let mut neighbors = Vec::new();
        neighbors.reserve_exact(self.data.len());
        for y in 0..self.height as i16 {
            for x in 0..self.width as i16 {
                let paper = self.paper_around(x, y);
                neighbors.push(paper);
            }
        }
        neighbors
    }

    pub fn remove_accessible(&mut self, neighbor_list: &mut [u8]) -> usize {
        let mut total_removed = 0;
        for (i, char) in self.data.iter_mut().enumerate() {
            if *char == b'@' && neighbor_list[i] < 4 {
                *char = b'x';
                total_removed += 1;

                let x = (i % self.width) as i16;
                let y = (i / self.width) as i16;

                // clean up neighbors
                for j in (y - 1)..=(y + 1) {
                    for i in (x - 1)..=(x + 1) {
                        if i == x && j == y {
                            continue;
                        }

                        if let Some(index) = index(self.width, self.height, i, j) {
                            neighbor_list[index] -= 1;
                        }
                    }
                }
            }
        }
        total_removed
    }
}

fn index(width: usize, height: usize, x: i16, y: i16) -> Option<usize> {
    if !(0..width as i16).contains(&x) {
        return None;
    }
    if !(0..height as i16).contains(&y) {
        return None;
    }

    Some(x as usize + y as usize * width)
}

pub fn part_one(input: &str) -> impl Display {
    Grid::new(input.as_bytes())
        // constructing the neighbor list is slightly slower but
        // the code looks nice
        .neighbor_list()
        .into_iter()
        .filter(|n| *n < 4)
        .count()
}

// 10132
pub fn part_two(input: &str) -> impl Display {
    let mut grid = Grid::new(input.as_bytes());

    let mut total_removed = 0;
    let mut neighbor_list = grid.neighbor_list();
    loop {
        let removed = grid.remove_accessible(&mut neighbor_list);
        total_removed += removed;
        if removed == 0 {
            break;
        }
    }

    total_removed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        assert_eq!("13", part_one(input).to_string().as_str());
    }
}
