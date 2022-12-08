struct Node {
    children: Vec<Node>,
    name: String,
    filesize: Option<usize>,
}

impl Node {
    pub fn new(name: String, filesize: Option<usize>) -> Node {
        Node {
            children: vec!(), name, filesize,
        }
    }

    pub fn add_files(&mut self, entries: Vec<(&str, usize)>) {
        for (name, size) in entries {
            self.children.push(Node::new(String::from(name), Some(size)));
        }
    }

    pub fn add_dir(&mut self, dir: Node) {
        self.children.push(dir)
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }
}

fn create_node(name: &str, entries: Vec<(&str, Option<usize>)>) -> Node {
    let mut node = Node::new(String::from(name), None);
    let mut vec: Vec<_> = vec![];
    for (filename, Some(size)) in entries {
        vec.push((filename, size))
    }
    return node
}

fn convert_entries(entries: Vec<&str>) -> Vec<(&str, Option<usize>)> {
    let res: Vec<(&str, usize)> = vec![];
    for entry in entries {
        if entry.starts_with("dir") {
            res.push((entry.strip_prefix("dir ").unwrap(), None))
        } else {
            let tmp = entry.split(" ");
            let size = tmp.next().unwrap().parse::<usize>().unwrap();
            let name = tmp.next().unwrap();
            res.push((name, Some(size)));
        }
    }
    return res
}

fn recurse_input(node: Node, chunks: Vec<(Vec<&str>, Vec<(&str, Option<usize>)>)>) -> (Vec<&str>, Vec<(&str, Option<usize>)>) {
    let (commands, entries) = chunks[0];
    assert_eq!("ls", commands[commands.len()-1]);

    if let Some(dir) = commands[0].strip_prefix("cd ") {
        if dir == ".." {
            return (commands[1..].to_vec(), entries)
        }
        let dir_node = create_node(dir, entries);
        node.add_dir(dir_node);
        recurse_input(node, chunks[1..].to_vec())
    } else { panic!() }
}

fn parse_input(input: &str) -> Node {
    let (commands, entries): (Vec<&str>, Vec<&str>) = input.lines()
        .partition(|x| x.starts_with("$"));
    let entries = convert_entries(entries);
    let node = Node::new("ROOT");
    recurse_input(node, commands, entries);
    return node
}


fn main() {
    input = include_str!("../../../inputs/2022_day07.txt");
    parse_input(input);
}
