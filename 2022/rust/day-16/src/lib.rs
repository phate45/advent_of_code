mod parser;
use parser::*;

use std::collections::{HashMap, BTreeSet};

use color_eyre::Result;

pub fn part1(source: &str) -> Result<String> {
    let nodes = parse(source).unwrap().1;

    let mut graph = Graph::new();
    for node in nodes.clone() {
        graph.add_node(node);
    }

    for node in nodes {
        node.tunnels.iter().for_each(|t| {
            graph.add_edge(&node.name, t);
        })
    }

    // dbg!(&graph);

    Ok(graph.max_flow("AA").to_string())
}


pub fn part2(_source: &str) -> Result<String> {
    let res = "";

    Ok(res.to_string())
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    name: String,
    flow_rate: u32,
    time: u32,
    visited: bool,
    tunnels: Vec<String>,
}

impl Node {
    pub fn new(name: String, flow_rate: u32, tunnels: Vec<String>) -> Self {
        Node {
            name,
            flow_rate,
            tunnels,
            visited: false,
            time: 0,
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<BTreeSet<usize>>,
    index_map: HashMap<String, usize>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) -> usize {
        let index = self.nodes.len();
        self.index_map.insert(node.name.clone(), index);
        self.nodes.push(node);
        self.edges.push(BTreeSet::new());
        index
    }

    fn add_edge(&mut self, u: &str, v: &str) {
        let u = *self.index_map.get(u).unwrap();
        let v = *self.index_map.get(v).unwrap();
        self.edges[u].insert(v);
        self.edges[v].insert(u);
    }

    fn max_flow(&mut self, start: &str) -> u32 {
        let start = *self.index_map.get(start).unwrap();
        let mut max_flow = 0;

        let mut stack = Vec::new();
        stack.push((start, 0, 0));
        // dbg!(&stack);

        while let Some((node, time, flow)) = stack.pop() {
            if time >= 30 {
                dbg!(&node, &time, stack.len());
                max_flow = std::cmp::max(max_flow, flow);
                continue;
            }

            if self.nodes[node].visited {
                continue;
            }

            self.nodes[node].visited = true;

            for &next in &self.edges[node] {
                stack.push((next, time + 1, flow));
            }

            if self.nodes[node].flow_rate > 0 {
                stack.push((node, time + 1, flow + self.nodes[node].flow_rate));
            }
        }

        max_flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source).unwrap(), "1651");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "1234");
    }
}
