use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::rc::Rc;
use std::time::Instant;

type NodeRef = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    children: HashMap<String, NodeRef>,
    size: usize,
    parent: Option<NodeRef>,
}

impl Node {
    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }

    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

fn traverse(node: NodeRef) -> Box<dyn Iterator<Item = NodeRef>> {
    let children = node.borrow().children.values().cloned().collect::<Vec<_>>();
    Box::new(
        std::iter::once(node).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(traverse(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

fn main() {
    let start = Instant::now();
    let contents = include_str!("input.txt");

    let root: NodeRef = NodeRef::default();
    let mut node = root.clone();

    for line in contents.lines() {
        let fragments: Vec<&str> = line.split(' ').collect();
        match fragments[0] {
            "$" => match fragments[1] {
                "cd" => match fragments[2] {
                    "/" => println!("Ignoring root dir"),
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    new_dir => {
                        let child = node
                            .borrow_mut()
                            .children
                            .entry(new_dir.to_string())
                            .or_default()
                            .clone();
                        node = child;
                    }
                },
                "ls" => {}
                _ => panic!("unknown command: {:?}", fragments[1]),
            },
            "dir" => {
                let dir = node
                    .borrow_mut()
                    .children
                    .entry(fragments[1].to_string())
                    .or_default()
                    .clone();
                dir.borrow_mut().parent = Some(node.clone());
            }
            raw_size => {
                let file = node
                    .borrow_mut()
                    .children
                    .entry(fragments[1].to_string())
                    .or_default()
                    .clone();

                let size: usize = raw_size.parse().unwrap();
                file.borrow_mut().size = size;
                file.borrow_mut().parent = Some(node.clone());
            }
        }
    }

    let small_dirs = traverse(root.clone())
        .map(|node| node.borrow().total_size())
        .filter(|&size| size <= 100_000)
        .sum::<u64>();

    dbg!(small_dirs);
    // traverse tree to calculate directory sizes
    // DFS or BFS?


    // part 2

    let unused_space = 70000000 - root.borrow().total_size();

    let smallest_big_dir = traverse(root)
        .map(|node| node.borrow().total_size())
        .filter(|&size| unused_space + size >= 30000000)
        .min();

    dbg!(smallest_big_dir);

    let duration = start.elapsed();
    println!("{:?}", duration)
}
