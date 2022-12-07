mod parser;

use camino::Utf8PathBuf;
use color_eyre::Result;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{combinator::all_consuming, Finish};
use parser::*;

pub fn part1(source: &str) -> String {
    let tree = create_tree(source).unwrap();

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>()
        .to_string();

    sum
}

pub fn part2(source: &str) -> Result<String> {
    let tree = create_tree(source).unwrap();

    let total_space = 70000000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let min = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min()
        .unwrap()
        .to_string();
    Ok(min)
}

#[derive(Debug)]
struct FsEntry {
    #[allow(dead_code)]
    path: Utf8PathBuf,
    size: u64,
}

fn create_tree(source: &str) -> Result<Tree<FsEntry>> {
    let lines = source
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");
    Ok(tree)
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part1() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part1(&source), "95437");
    }

    #[test]
    fn test_part2() {
        let source = fs::read_to_string("./test_input.txt").unwrap();
        assert_eq!(part2(&source).unwrap(), "24933642");
    }
}
