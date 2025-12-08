use std::{collections::HashMap, fmt::Display};

struct Field {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn new(input: &str) -> Self {
        let width = input.as_bytes().iter().position(|c| *c == b'\n').unwrap() + 1;
        let height = input.len() / width;

        Self {
            data: input.as_bytes().to_vec(),
            width,
            height: height + 1,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[x + y * self.width] = value;
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
                std::str::from_utf8(&self.data[start..end]).unwrap()
            )?;
        }

        Ok(())
    }
}

pub fn part_one(input: &str) -> impl Display {
    let mut field = Field::new(input);

    // set first beam
    for x in 0..field.width - 1 {
        if field.data[x] == b'S' {
            field.data[x + field.width] = b'|'
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

#[derive(Debug)]
struct Node {
    position: (usize, usize),
    children: Vec<usize>,
}

pub fn part_two(input: &str) -> impl Display {
    let mut tree: Vec<Node> = Vec::new();
    let mut field = Field::new(input);

    // set first beam
    for x in 0..field.width - 1 {
        if field.data[x] == b'S' {
            tree.push(Node {
                position: (x, 1),
                children: Vec::new(),
            });
            field.data[x + field.width] = b'|'
        }
    }

    for y in 2..field.height {
        for x in 0..field.width - 1 {
            match (field.get(x, y - 1), field.get(x, y)) {
                (b'|', b'.') => {
                    let len = tree.len();
                    let parent = tree
                        .iter_mut()
                        .find(|n| n.position.0 == x && n.position.1 == y - 1)
                        .unwrap();
                    parent.children.push(len);
                    tree.push(Node {
                        position: (x, y),
                        children: Vec::new(),
                    });
                    field.set(x, y, b'|');
                }
                (b'|', b'|') => {
                    let parent_index = tree
                        .iter()
                        .position(|n| n.position.0 == x && n.position.1 == y - 1)
                        .unwrap();
                    let existing_child = tree
                        .iter()
                        .position(|n| n.position.0 == x && n.position.1 == y)
                        .unwrap();
                    tree[parent_index].children.push(existing_child);
                }
                (b'|', b'^') => {
                    let parent_index = tree
                        .iter()
                        .position(|n| n.position.0 == x && n.position.1 == y - 1)
                        .unwrap();

                    if field.get(x + 1, y) == b'|' {
                        let existing_child = tree
                            .iter()
                            .position(|n| n.position.0 == x + 1 && n.position.1 == y)
                            .unwrap();
                        tree[parent_index].children.push(existing_child);
                    } else {
                        let len = tree.len();
                        tree[parent_index].children.push(len);
                        tree.push(Node {
                            position: (x + 1, y),
                            children: Vec::new(),
                        });
                    }

                    if field.get(x - 1, y) == b'|' {
                        let existing_child = tree
                            .iter()
                            .position(|n| n.position.0 == x - 1 && n.position.1 == y)
                            .unwrap();
                        tree[parent_index].children.push(existing_child);
                    } else {
                        let len = tree.len();
                        tree[parent_index].children.push(len);
                        tree.push(Node {
                            position: (x - 1, y),
                            children: Vec::new(),
                        });
                    }

                    field.set(x - 1, y, b'|');
                    field.set(x + 1, y, b'|');
                }
                _ => {}
            }
        }
    }

    // now do a search over the tree
    fn descend_tree(tree: &[Node], index: usize, searched: &mut HashMap<usize, usize>) -> usize {
        let node = &tree[index];

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

    descend_tree(&tree, 0, &mut HashMap::default())
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
