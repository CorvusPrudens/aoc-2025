use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn increment_joltage(buttons: &[usize], joltage: &mut [usize]) {
    for button in buttons {
        joltage[*button] += 1;
    }
}

impl Machine {
    fn exhaustive_light_solution(&self) -> usize {
        fn toggle_lights(buttons: &[usize], lights: &mut [bool]) {
            for button in buttons {
                let value = lights[*button];
                lights[*button] = !value;
            }
        }

        // BFS
        let mut states_in = vec![vec![false; self.target_lights.len()]; 1];
        let mut states_out = Vec::new();

        let mut step = 1;
        loop {
            for state in states_in.drain(..) {
                for button in self.buttons.iter() {
                    let mut new_state = state.clone();
                    toggle_lights(button, &mut new_state);

                    if new_state == self.target_lights {
                        return step;
                    }

                    states_out.push(new_state);
                }
            }

            core::mem::swap(&mut states_out, &mut states_in);
            step += 1;
        }
    }

    fn a_star_joltage(&self) -> usize {
        #[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Debug)]
        struct Position(Vec<usize>);

        #[derive(Clone, PartialEq, Eq)]
        struct OpenNode {
            score: i64,
            position: Position,
        }

        impl Ord for OpenNode {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.score.cmp(&self.score)
            }
        }

        impl PartialOrd for OpenNode {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(other.score.cmp(&self.score))
            }
        }

        let mut came_from = HashMap::<Position, Position>::new();
        fn reconstruct_path<'a>(
            came_from: &'a HashMap<Position, Position>,
            mut current: &'a Position,
        ) -> Vec<&'a Position> {
            let mut path = vec![current];

            while let Some(prev) = came_from.get(current) {
                path.insert(0, prev);
                current = prev;
            }

            path
        }

        let start = Position(vec![0; self.joltage.len()]);

        let mut g_score = HashMap::<Position, i64>::new();
        g_score.insert(start.clone(), 0);

        // let mut f_score = HashMap::<Position, i64>::new();
        // let start_score = heuristic(&start, &self.joltage);
        // f_score.insert(start.clone(), start_score);

        let mut open_set = BinaryHeap::new();
        open_set.push(OpenNode {
            score: heuristic(&start, &self.joltage),
            position: start,
        });

        // just the distance
        fn heuristic(node: &Position, goal: &[usize]) -> i64 {
            node.0
                .iter()
                .zip(goal)
                .map(|(a, b)| *b as i64 - *a as i64)
                .map(|d| d * d)
                .sum()
        }

        while let Some(OpenNode {
            score,
            position: current_node,
        }) = open_set.pop()
        {
            if current_node.0 == self.joltage {
                let path = reconstruct_path(&came_from, &current_node);
                // println!("{path:#?}");
                return path.len() - 1;
            }

            // println!("current_node: {:?}", current_node.0);

            // for each neighbor (including backwards neighbors?)
            for button in self.buttons.iter() {
                let mut neighbor = current_node.clone();
                increment_joltage(button, &mut neighbor.0);

                // prune invalid nodes
                if neighbor.0.iter().zip(&self.joltage).any(|(a, b)| a > b) {
                    continue;
                }

                let tentative_g_score = g_score[&current_node] + button.len() as i64;

                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(i64::MAX) {
                    came_from.insert(neighbor.clone(), current_node.clone());
                    g_score.insert(neighbor.clone(), tentative_g_score);

                    open_set.push(OpenNode {
                        score: tentative_g_score + heuristic(&neighbor, &self.joltage),
                        position: neighbor,
                    });
                }
            }
        }

        panic!("Failed to find path");
    }

    // naive DFS and BFS are too slow
    fn exhaustive_joltage_solution(&mut self) -> usize {
        // DFS
        self.buttons.sort_by_key(|b| Reverse(b.len()));

        fn dfs(machine: &Machine, depth: usize, state: &[usize]) -> Option<usize> {
            for button in &machine.buttons {
                let mut new_state = state.to_vec();
                increment_joltage(button, &mut new_state);

                if new_state == machine.joltage {
                    return Some(depth);
                } else if new_state.iter().zip(&machine.joltage).any(|(a, b)| a > b) {
                    return None;
                } else if let Some(value) = dfs(machine, depth + 1, &new_state) {
                    return Some(value);
                }
            }

            None
        }

        let initial_state = vec![0; self.joltage.len()];
        dfs(&self, 1, &initial_state).unwrap()

        // let mut step = 1;
        // loop {
        //     for state in states_in.drain(..) {
        //         for button in self.buttons.iter() {
        //             let mut new_state = state.clone();
        //             increment_joltage(button, &mut new_state);

        //             if new_state == self.joltage {
        //                 return step;
        //             }

        //             // prune invalid branches
        //             if new_state.iter().zip(&self.joltage).all(|(a, b)| a <= b) {
        //                 states_out.push(new_state);
        //             }
        //         }
        //     }

        //     core::mem::swap(&mut states_out, &mut states_in);
        //     step += 1;
        // }
    }

    fn parse_line(line: &str) -> Self {
        let mut parts = line.split(' ');

        let lights = parts.next().unwrap();
        let mut target_lights = Vec::new();
        for char in lights[1..lights.len() - 1].chars() {
            match char {
                '.' => target_lights.push(false),
                '#' => target_lights.push(true),
                _ => unreachable!(),
            }
        }

        let mut joltage = Vec::new();
        let mut buttons = Vec::new();

        for part in parts {
            match part.chars().next().unwrap() {
                '(' => {
                    let numbers = part[1..part.len() - 1]
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect();
                    buttons.push(numbers);
                }
                '{' => {
                    let numbers = part[1..part.len() - 1]
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap());
                    joltage.extend(numbers);
                }
                _ => unreachable!(),
            }
        }

        Machine {
            target_lights,
            buttons,
            joltage,
        }
    }
}

pub fn part_one(input: &str) -> impl Display {
    input
        .lines()
        .map(Machine::parse_line)
        .map(|machine| machine.exhaustive_light_solution())
        .inspect(|value| println!("value: {value}"))
        .sum::<usize>()
}

pub fn part_two(input: &str) -> impl Display {
    input
        .lines()
        .map(Machine::parse_line)
        // .map(|mut machine| machine.exhaustive_joltage_solution())
        .map(|machine| machine.a_star_joltage())
        .inspect(|value| println!("value: {value}"))
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ten() {
        let input = include_str!("../../inputs/ten_test.txt");
        assert_eq!("7", part_one(input).to_string().as_str());
    }

    #[test]
    fn ten2() {
        let input = include_str!("../../inputs/ten_test.txt");
        assert_eq!("33", part_two(input).to_string().as_str());
    }
}
