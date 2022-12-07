use slab_tree::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Plik {
    name: String,
    size: usize,
}

impl Plik {
    fn new(size: usize, name: &str) -> Self {
        Plik {
            size: size,
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
struct Katalog {
    name: String,
    files: Vec<Plik>,
}

impl Katalog {
    fn new(name: &str) -> Self {
        Katalog {
            name: name.to_string(),
            files: Vec::new(),
        }
    }

    fn add_file(&mut self, file: Plik) {
        self.files.push(file);
    }
}

fn count_total_size(node: NodeRef<Katalog>) -> usize {
    node.data().files.iter().map(|f| f.size).sum::<usize>()
        + node.children().map(|d| count_total_size(d)).sum::<usize>()
}

fn main() -> std::io::Result<()> {
    let f = File::open("input-07.txt")?;
    let reader = BufReader::new(f);

    let mut tree = TreeBuilder::new().with_root(Katalog::new("/")).build();
    let mut node_id = tree.root_id().expect("root doesn't exist?");

    for line in reader.lines() {
        let line = line?;
        if let Some(name) = line.strip_prefix("$ cd ") {
            if name == "/" {
                continue;
            } else if name == ".." {
                node_id = tree.get(node_id).unwrap().parent().unwrap().node_id();
            } else {
                node_id = tree
                    .get(node_id)
                    .unwrap()
                    .children()
                    .filter(|d| d.data().name == name)
                    .next()
                    .unwrap()
                    .node_id();
            }
        } else if line.contains("$ ls") {
            continue;
        } else {
            if let Some(name) = line.strip_prefix("dir ") {
                tree.get_mut(node_id).unwrap().append(Katalog::new(name));
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                tree.get_mut(node_id)
                    .unwrap()
                    .data()
                    .add_file(Plik::new(size.parse::<usize>().unwrap(), name));
            }
        }
    }

    let dirs_with_size = tree
        .root()
        .unwrap()
        .traverse_pre_order()
        .map(|node_ref| (node_ref.data().name.to_string(), count_total_size(node_ref)))
        .filter(|(_, size)| *size < 100_000)
        .collect::<Vec<_>>();

    println!(
        "Part1 - {:?}",
        dirs_with_size.iter().map(|s| s.1).sum::<usize>()
    );
    let root = tree.root().unwrap();
    let total_size = 70_000_000;
    let needed_space = 30_000_000;
    let root_size = count_total_size(root);
    let current_free_space = total_size - root_size;

    let mut dirs = tree
        .root()
        .unwrap()
        .traverse_pre_order()
        .map(|node_ref| (node_ref.data().name.to_string(), count_total_size(node_ref)))
        .collect::<Vec<_>>();
    dirs.sort_by_key(|x| x.1);
    let dir_size = dirs
        .iter()
        .filter(|s| current_free_space + s.1 >= needed_space)
        .collect::<Vec<_>>()[0];
    println!("Part2 - {:?}", dir_size);
    Ok(())
}
