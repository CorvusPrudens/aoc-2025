use std::{collections::HashMap, fmt::Display};

struct Field {
    data: Vec<u8>,
    width: u16,
    height: u16,
}

impl Field {
    pub fn new(input: &str) -> Self {
        let width = input.as_bytes().iter().position(|c| *c == b'\n').unwrap() + 1;
        let height = input.len() / width;

        Self {
            data: input.as_bytes().to_vec(),
            width: width as u16,
            height: (height + 1) as u16,
        }
    }

    pub fn get(&self, x: u16, y: u16) -> u8 {
        let x = x as usize;
        let y = y as usize;
        self.data[x + y * self.width as usize]
    }

    pub fn set(&mut self, x: u16, y: u16, value: u8) {
        let x = x as usize;
        let y = y as usize;
        self.data[x + y * self.width as usize] = value;
    }
}

impl core::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width - 1;

            write!(
                f,
                "{}\n",
                std::str::from_utf8(&self.data[start as usize..end as usize]).unwrap()
            )?;
        }

        Ok(())
    }
}

pub fn part_one(input: &str) -> impl Display {
    let mut field = Field::new(input);

    // set first beam
    for x in 0..field.width - 1 {
        if field.get(x, 0) == b'S' {
            field.set(x, 1, b'|');
        }
    }

    let mut splits = 0;
    for y in 2..field.height {
        for x in 0..field.width - 1 {
            match (field.get(x, y - 1), field.get(x, y)) {
                (b'|', b'.') => {
                    field.set(x, y, b'|');
                }
                (b'|', b'^') => {
                    field.set(x - 1, y, b'|');
                    field.set(x + 1, y, b'|');
                    splits += 1;
                }
                _ => {}
            }
        }
    }

    splits
}

#[derive(Debug, Default)]

struct Node {
    children: Vec<(u16, u16)>,
}

pub fn part_two(input: &str) -> impl Display {
    let mut tree: HashMap<(u16, u16), Node> = HashMap::new();
    let mut field = Field::new(input);

    // set first beam
    let mut first_node = (0u16, 0u16);
    for x in 0..field.width - 1 {
        if field.get(x, 0) == b'S' {
            field.set(x, 1, b'|');
            tree.insert(
                (x, 1),
                Node {
                    children: Vec::new(),
                },
            );
            first_node = (x, 1);
        }
    }

    for y in 2..field.height {
        for x in 0..field.width - 1 {
            match (field.get(x, y - 1), field.get(x, y)) {
                (b'|', b'.') => {
                    let parent = tree.get_mut(&(x, y - 1)).unwrap();
                    parent.children.push((x as u16, y as u16));
                    tree.entry((x, y)).or_default();
                    field.set(x, y, b'|');
                }
                (b'|', b'|') => {
                    tree.get_mut(&(x, y - 1)).unwrap().children.push((x, y));
                }
                (b'|', b'^') => {
                    tree.get_mut(&(x, y - 1))
                        .unwrap()
                        .children
                        .extend([(x - 1, y), (x + 1, y)]);
                    tree.entry((x - 1, y)).or_default();
                    tree.entry((x + 1, y)).or_default();

                    field.set(x - 1, y, b'|');
                    field.set(x + 1, y, b'|');
                }
                _ => {}
            }
        }
    }

    // now do a search over the tree
    fn descend_tree(
        tree: &HashMap<(u16, u16), Node>,
        index: (u16, u16),
        searched: &mut HashMap<(u16, u16), usize>,
    ) -> usize {
        let node = &tree[&index];

        match node.children.as_slice() {
            &[] => 1,
            slice => {
                let mut leaves = 0;
                for node in slice {
                    match searched.get(node).copied() {
                        Some(l) => {
                            leaves += l;
                        }
                        None => {
                            let l = descend_tree(tree, *node, searched);
                            leaves += l;
                            searched.insert(*node, l);
                        }
                    }
                }
                leaves
            }
        }
    }

    descend_tree(&tree, first_node, &mut HashMap::default())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree() {
        let input = include_str!("../../../inputs/seven_test.txt");
        assert_eq!("21", part_one(input).to_string().as_str());
    }

    #[test]
    fn tree_2() {
        let input = include_str!("../../../inputs/seven_test.txt");
        assert_eq!("40", part_two(input).to_string().as_str());
    }
}
