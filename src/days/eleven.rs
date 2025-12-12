use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;
type Explored<'a> = HashMap<&'a str, usize>;

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let key = &line[..3];
        let connections = line[5..].split(' ').collect();
        graph.insert(key, connections);
    }

    graph
}

pub fn part_one(input: &str) -> impl Display {
    let graph = parse_graph(input);

    let mut explored_paths: Explored = HashMap::new();

    fn explore_paths<'a>(graph: &Graph<'a>, node: &'a str, explored: &mut Explored<'a>) -> usize {
        if node == "out" {
            return 1;
        }

        let children = graph.get(node);
        let mut counts = 0;

        for child in children.iter().flat_map(|c| c.iter()) {
            let child_paths = match explored.get(child) {
                Some(child_paths) => child_paths.clone(),
                None => {
                    let child_paths = explore_paths(graph, child, explored);
                    explored.insert(child, child_paths.clone());
                    child_paths
                }
            };

            counts += child_paths;
        }

        counts
    }

    let paths = explore_paths(&graph, "you", &mut explored_paths);

    paths
}

type ExploredSet<'a> = HashMap<&'a str, Vec<HashSet<&'a str>>>;

pub fn part_two_slow(input: &str) -> impl Display {
    let graph = parse_graph(input);

    let mut explored_paths: ExploredSet = HashMap::new();

    fn explore_paths<'a>(
        graph: &Graph<'a>,
        node: &'a str,
        explored: &mut ExploredSet<'a>,
    ) -> Vec<HashSet<&'a str>> {
        if node == "out" {
            return vec![HashSet::from_iter(["out"])];
        }

        let children = graph.get(node);
        let mut paths = Vec::new();

        for child in children.iter().flat_map(|c| c.iter()) {
            let mut child_paths = match explored.get(child) {
                Some(child_paths) => child_paths.clone(),
                None => {
                    let child_paths = explore_paths(graph, child, explored);
                    explored.insert(child, child_paths.clone());
                    child_paths
                }
            };

            for path in &mut child_paths {
                path.insert(node);
            }
            paths.extend(child_paths);
        }

        paths
    }

    let paths = explore_paths(&graph, "svr", &mut explored_paths);

    paths
        .iter()
        .filter(|p| p.contains("fft") && p.contains("dac"))
        .count()
}

pub fn part_two(input: &str) -> impl Display {
    #[derive(Clone)]
    enum SearchState {
        Out(usize),
        Fft(usize),
        Dac(usize),
        Both(usize),
    }

    type ExploredPair<'a> = HashMap<&'a str, SearchState>;

    impl SearchState {
        fn add(&self, other: &Self) -> Self {
            match (self, other) {
                (Self::Out(a), Self::Out(b)) => Self::Out(*a + *b),
                (Self::Fft(a), Self::Fft(b)) => Self::Fft(*a + *b),
                (Self::Dac(a), Self::Dac(b)) => Self::Dac(*a + *b),
                (Self::Both(a), Self::Both(b)) => Self::Both(*a + *b),

                (Self::Out(_), Self::Fft(b)) => Self::Fft(*b),
                (Self::Fft(a), Self::Out(_)) => Self::Fft(*a),
                (Self::Out(_), Self::Dac(b)) => Self::Dac(*b),
                (Self::Dac(a), Self::Out(_)) => Self::Dac(*a),

                // doesn't matter, death path
                (Self::Dac(a), Self::Fft(_)) => Self::Dac(*a),
                (Self::Fft(a), Self::Dac(_)) => Self::Fft(*a),

                (Self::Both(a), _) => Self::Both(*a),
                (_, Self::Both(b)) => Self::Both(*b),
            }
        }

        fn both(&self) -> usize {
            match self {
                Self::Both(b) => *b,
                _ => panic!("expected Both"),
            }
        }
    }

    let graph = parse_graph(input);
    let mut explored_paths: ExploredPair = HashMap::new();

    fn explore_paths<'a>(
        graph: &Graph<'a>,
        node: &'a str,
        target: &str,
        explored: &mut ExploredPair<'a>,
    ) -> SearchState {
        if node == target {
            return SearchState::Out(1);
        }

        let children = graph.get(node);

        let mut state = SearchState::Out(0);

        for child in children.iter().flat_map(|c| c.iter()) {
            let child_paths = match explored.get(child) {
                Some(child_paths) => child_paths.clone(),
                None => {
                    let child_paths = explore_paths(graph, child, target, explored);

                    let child_paths = match *child {
                        "dac" => match child_paths {
                            SearchState::Fft(count) => SearchState::Both(count),
                            SearchState::Out(count) => SearchState::Dac(count),
                            other => other,
                        },
                        "fft" => match child_paths {
                            SearchState::Dac(count) => SearchState::Both(count),
                            SearchState::Out(count) => SearchState::Fft(count),
                            other => other,
                        },
                        _ => child_paths,
                    };

                    explored.insert(child, child_paths.clone());
                    child_paths
                }
            };

            state = state.add(&child_paths);
        }

        state
    }

    let paths = explore_paths(&graph, "svr", "out", &mut explored_paths);
    paths.both()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eleven() {
        let input = include_str!("../../inputs/eleven_test.txt");
        assert_eq!("5", part_one(input).to_string().as_str());
    }

    #[test]
    fn eleven2() {
        let input = include_str!("../../inputs/eleven_test2.txt");
        assert_eq!("2", part_two(input).to_string().as_str());

        let input = include_str!("../../inputs/eleven_test3.txt");
        assert_eq!("6", part_two_slow(input).to_string().as_str());
        assert_eq!("6", part_two(input).to_string().as_str());
    }
}
