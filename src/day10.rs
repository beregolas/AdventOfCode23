use std::ops::Index;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn resolve(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0)
        }
    }

    fn rev(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Field {
    Pipe(Direction, Direction),
    Start,
    Empty,
}

impl Field {
    fn new(c: char) -> Field {
        match c {
            '|' => Field::Pipe(Direction::North, Direction::South),
            '-' => Field::Pipe(Direction::East, Direction::West),
            'J' => Field::Pipe(Direction::North, Direction::West),
            'L' => Field::Pipe(Direction::North, Direction::East),
            '7' => Field::Pipe(Direction::South, Direction::West),
            'F' => Field::Pipe(Direction::South, Direction::East),
            '.' => Field::Empty,
            'S' => Field::Start,
            _ => panic!("Invalid pipe character")
        }
    }

    fn get_other_direction(&self, dir: Direction) -> Option<Direction> {
        match self {
            Field::Pipe(d1, d2) => {
                if d1 == &dir {
                    Some(*d2)
                } else if d2 == &dir {
                    Some(*d1)
                } else {
                    None
                }
            }
            Field::Start => None,
            Field::Empty => None
        }
    }

    fn connects_to(&self, dir: Direction) -> bool {
        match self {
            Field::Pipe(d1, d2) => {
                d1 == &dir || d2 == &dir
            }
            Field::Start => true,
            Field::Empty => false
        }
    }
}

struct Graph {
    data: Vec<Vec<Field>>,
    start: Option<(usize, usize)>,
    start_directions: Option<(Direction, Direction)>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            data: Vec::new(),
            start: None,
            start_directions: None,
        }
    }

    fn add_row(&mut self, row: Vec<Field>) {
        self.data.push(row);
    }

    fn find_start(&self) -> (usize, usize) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if let Field::Start = field {
                    return (x, y);
                }
            }
        }
        panic!("No start found");
    }

    fn walk(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        // get direction as vector
        let (dx, dy) = dir.resolve();
        // get current field
        let field = &self[(pos.0, pos.1)];
        // get next field
        let next_position = (pos.0 as i32 + dx, pos.1 as i32 + dy);
        if next_position.0 < 0 || next_position.1 < 0 || next_position.0 >= self.data[0].len() as i32 || next_position.1 >= self.data.len() as i32 {
            return None;
        }
        let next = &self[(next_position.0 as usize, next_position.1 as usize)];
        // check if both fields connect to each other
        if field.connects_to(dir) && next.connects_to(dir.rev()) {
            Some((next_position.0 as usize, next_position.1 as usize))
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for Graph {
    type Output = Field;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

pub(crate) fn c1(input: String) -> String {
    let mut graph = Graph::new();
    for line in input.lines() {
        graph.add_row(line.chars().map(|c| Field::new(c)).collect());
    }
    // go from start field and find the loop
    let mut directions = vec![Direction::North, Direction::South, Direction::East, Direction::West];
    for direction in directions {
        let mut pos = graph.find_start();
        // try to take the first step
        if let Some(next) = graph.walk(pos, direction) {
            pos = next;
        } else {
            continue;
        }
        let mut last_direction = direction;
        let mut steps = 1;
        loop {
            // try to take a step in a new direction
            let new_direction = graph[pos].get_other_direction(last_direction.rev());
            if let Some(new_direction) = new_direction {
                if let Some(next) = graph.walk(pos, new_direction) {
                    pos = next;
                    last_direction = new_direction;
                    steps += 1;
                } else {
                    break;
                }
            } else {
                break;

            }
        }
        // if pos is the start pos, we found the loop
        if pos == graph.find_start() {
            return (steps/2).to_string();
        }
    }
    panic!("No loop found");
}


pub(crate) fn c2(input: String) -> String {
    "sum".to_string()
}