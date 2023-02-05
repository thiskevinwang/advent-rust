use id_tree::{InsertBehavior, Node, Tree};
use std::path::PathBuf;

#[derive(Debug)]
struct FsEntry {
    path: PathBuf,
    size: i128,
}

type Result<T> = std::result::Result<T, &'static str>;
fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> Result<i128> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child).unwrap())?;
    }
    Ok(total)
}

pub fn solution() {
    println!("https://adventofcode.com/2022/day/7");
    let lines = include_str!("input.txt").lines();

    // let mut cwd = PathBuf::new();
    let mut tree = Tree::<FsEntry>::new();

    let root = tree
        .insert(
            Node::new(FsEntry {
                path: "/".into(),
                size: 0,
            }),
            InsertBehavior::AsRoot,
        )
        .unwrap();

    // curr will be a moving pointer to the current node
    let mut curr = root;

    for line in lines {
        // println!("{line:?}");
        let is_cmd = line.split(" ").collect::<Vec<&str>>()[0] == "$";

        match line {
            // cmd
            // std::str::Lines (&str) -> std::str::Split -> Vec<&str> -> &str
            // https://stackoverflow.com/a/26643821/9823455
            i if is_cmd => {
                if let [_, cmd, arg] = i.split(" ").collect::<Vec<&str>>()[..] {
                    match cmd {
                        "cd" => {
                            match arg {
                                "/" => {
                                    // noop — we set this as our root node.
                                }
                                // go up 1 level
                                ".." => {
                                    curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                                }
                                // push
                                _ => {
                                    // cwd.push(arg);
                                    let node = Node::new(FsEntry {
                                        path: PathBuf::from(arg),
                                        size: 0,
                                    });
                                    curr = tree
                                        .insert(node, InsertBehavior::UnderNode(&curr))
                                        .unwrap();
                                }
                            }
                        }
                        "ls" => {
                            // when listing, the lines until the next command are
                            // dir or file stats
                            // maybe noop?
                        }
                        _ => {
                            // this case should never be reached
                            panic!(
                                "Line parsing reached an unexpected case. This should never occur."
                            );
                        }
                    }
                }
            }
            // ls output; either file or dir stats
            _ => {
                // file
                if let Ok(file_size) = line.split(" ").collect::<Vec<&str>>()[0].parse::<i128>() {
                    let file_name = line.split(" ").collect::<Vec<&str>>()[1];

                    let node = Node::new(FsEntry {
                        size: file_size,
                        path: PathBuf::from(file_name),
                    });
                    tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                }
                // dir
                else {
                }
            }
        }
    }

    // This will get used by part 1 and part 2
    let folders = tree
        .traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        // filter out all leaf nodes (aka files) since we only want the sum of directory sizes
        .filter(|n| n.children().len() > 0);

    let root_size = total_size(&tree, tree.get(tree.root_node_id().unwrap()).unwrap()).unwrap();

    // the sum of folders less than or equal to 100_000
    let part1 = folders
        .clone()
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        // .inspect(|s| {
        //     dbg!(s);
        // })
        .sum::<i128>();

    // Part 1
    dbg!(part1); // 1297159

    // Part 2
    let remaining_space = 70_000_000 - root_size;

    // The smallest dir that can be removed to free up enough space for the
    // 30_000_000 update.
    let smol_dir = folders
        .clone()
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s >= (30_000_000 - remaining_space))
        .min()
        .unwrap();

    dbg!(smol_dir); // 38666390
}
