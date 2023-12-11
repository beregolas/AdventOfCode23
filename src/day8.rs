use std::collections::HashMap;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct Node([char; 3]);

impl Node {
    fn new(name: [char; 3]) -> Node {
        Node(name)
    }

    fn ends_in_z(&self) -> bool {
        self.0[2] == 'Z'
    }
}

struct Graph {
    nodes: HashMap<Node, (Node, Node)>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node, left: Node, right: Node) {
        self.nodes.insert(node, (left, right));
    }

    fn follow(&self, node: Node, instruction: char) -> Node {
        let (left, right) = self.nodes.get(&node).unwrap();
        if instruction == 'L' {
            *left
        } else {
            *right
        }
    }
}

struct Path<'a> {
    nodes: Vec<(Node, usize, bool)>,
    graph: &'a Graph,
    instructions: Vec<char>,
    circle_start: Option<usize>,
    target_positions: Vec<usize>
}

impl Path<'_> {
    fn new(start: Node, graph: &Graph, instructions: Vec<char>) -> Path {
        Path {
            nodes: vec![(start, 0, false)],
            graph,
            instructions,
            circle_start: None,
            target_positions: Vec::new(),
        }
    }

    fn is_target(&self, idx: i128) -> bool {
        for &target in &self.target_positions {
            if (idx - target as i128) % (self.nodes.len() - self.circle_start.unwrap()) as i128 == 0 {
                return true;
            }
        }
        false
    }

    fn follow_until_circle(&mut self) {
        loop {
            let mut next_instruction_idx = (self.nodes.len()-1) % self.instructions.len();
            let mut next_node = self.graph.follow(
                self.nodes.last().unwrap().0,
                self.instructions[next_instruction_idx]
            );
            let mut next = (next_node, next_instruction_idx, next_node.ends_in_z());
            if self.nodes.contains(&next) {
                // circle found, save circle start
                self.circle_start = Some(self.nodes.iter().position(|x| x == &next).unwrap());
                // find all target positions
                self.target_positions = self.nodes.iter().enumerate().fold(Vec::new(), |mut acc, (i, n)| {
                    if n.2 {
                        acc.push(i)
                    }
                    acc
                });
                break;
            } else {
                self.nodes.push(next);
            }
        }
    }

}

fn get_common_target(paths: Vec<Path>) -> u128 {
    // get first path
    let first = paths.first().unwrap();
    // iterate until you find an integer that is a valid target for all paths
    let mut n = first.target_positions.first().unwrap().clone() as u128;
    loop {
        if paths.iter().all(|path| path.is_target(n as i128)) {
            return n;
        }
        if !first.is_target(n as i128) {
            println!("ERROR!!!!")
        }
        n += first.nodes.len() as u128 - first.circle_start.unwrap() as u128;
        if (n % 1000000) < 1000 {
            println!("n: {}", n);
        }
    }
}

pub(crate) fn c1(input: String) -> String {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    let mut graph = Graph::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let line = line.replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace("= ", "");
        let mut values = line.split(" ");
        let from = Node(values.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap());
        let left = Node(values.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap());
        let right = Node(values.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap());
        graph.add_node(from, left, right);
    }
    // follow the yellow brick road
    let mut current = Node("AAA".chars().collect::<Vec<char>>().try_into().unwrap());
    let goal = Node("ZZZ".chars().collect::<Vec<char>>().try_into().unwrap());
    let mut count = 0;
    while current != goal {
        current = graph.follow(current, instructions[count % instructions.len()]);
        count += 1;
    }
    count.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();
    let mut graph = Graph::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let line = line.replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace("= ", "");
        let mut values = line.split(" ");
        let from = Node(values.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap());
        let left = Node(values.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap());
        let right = Node(values.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap());
        graph.add_node(from, left, right);
    }
    // follow the yellow brick road
    // until all current nodes end in Z
    let mut start_nodes: Vec<Node> = graph.nodes.keys().filter_map(|node| {
        if node.0[2] == 'A' {
            Some(*node)
        } else {
            None
        }
    }).collect();
    // find a loop for every path
    let mut paths: Vec<Path> = Vec::new();
    for (i, n) in start_nodes.iter().enumerate() {
        let start = *n;
        // follow the yellow brick road until we reach a circle
        let mut path = Path::new(start, &graph, instructions.clone());
        path.follow_until_circle();
        paths.push(path);
    }
    let count = get_common_target(paths);
    count.to_string()
}