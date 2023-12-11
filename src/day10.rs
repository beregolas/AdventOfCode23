use std::ops::Index;
use itertools::Itertools;

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

    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

    fn move_from_to(&self, dir: Direction) -> Option<Direction> {
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

    fn get_left_right(&self, out_dir: Direction) -> (Vec<Direction>, Vec<Direction>) {
        let mut right_mode = true;
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut dir = out_dir;
        loop {
            dir = dir.right();
            if self.connects_to(dir) {
                if right_mode {
                    right_mode = false;
                } else {
                    return (left, right)
                }
                right_mode = false;
            } else {
                if right_mode {
                    right.push(dir);
                } else {
                    left.push(dir);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Position(usize, usize);

struct Graph {
    data: Vec<Vec<Field>>,
    start: Option<Position>,
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

    fn find_start(&self) -> Position {
        for (y, row) in self.data.iter().enumerate() {
            for (x, field) in row.iter().enumerate() {
                if let Field::Start = field {
                    return Position(x, y);
                }
            }
        }
        panic!("No start found");
    }

    fn slide(&self, pos: Position, dir: Direction) -> Option<Position> {
        // get direction as vector
        let (dx, dy) = dir.resolve();
        let next_position = (pos.0 as i32 + dx, pos.1 as i32 + dy);
        if next_position.0 < 0 || next_position.1 < 0 || next_position.0 >= self.data[0].len() as i32 || next_position.1 >= self.data.len() as i32 {
            None
        } else {
            Some(Position(next_position.0 as usize, next_position.1 as usize))
        }

    }

    fn step(&self, pos: Position, dir: Direction) -> Option<Position> {
        let field = self[pos];
        let next_position = self.slide(pos, dir);
        if let Some(next_position) = next_position {
            let next_field = self[next_position];
            if field.connects_to(dir) && next_field.connects_to(dir.rev()) {
                Some(next_position)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn find_path(&self, start: Position, start_direction: Direction) -> Option<(Vec<Position>, Vec<Position>)> {
        let mut path = Vec::new();
        let mut right_nodes = Vec::new();
        let mut left_nodes = Vec::new();
        let mut left = 0;
        let mut right = 0;
        let mut pos = start;
        let mut dir = start_direction;
        let mut node = self[pos];
        while let Some(next_pos) = self.step(pos, dir) {
            // find all left and right nodes this node doesn't connect to
            let (l, r) = node.get_left_right(dir);
            for d in l {
                if let Some(neighbour) = self.slide(pos, d) {
                    left_nodes.push(neighbour);
                }
            }
            for d in r {
                if let Some(neighbour) = self.slide(pos, d) {
                    right_nodes.push(neighbour);
                }
            }
            // collect left and right turns
            if node.connects_to(dir.left()) && node != Field::Start {
                left += 1;
            }
            if node.connects_to(dir.right()) && node != Field::Start {
                right += 1;
            }
            // add pos to path
            path.push(pos);
            // move to next pos
            pos = next_pos;
            node = self[pos];
            dir = node.move_from_to(dir.rev()).unwrap_or(start_direction);
            if let Field::Start = self[pos] {
                return Some((path, if left > right { left_nodes } else { right_nodes }));
            }
        }
        None
    }

}

impl Index<Position> for Graph {
    type Output = Field;

    fn index(&self, index: Position) -> &Self::Output {
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
        if let Some(next) = graph.step(pos, direction) {
            pos = next;
        } else {
            continue;
        }
        let mut last_direction = direction;
        let mut steps = 1;
        loop {
            // try to take a step in a new direction
            let new_direction = graph[pos].move_from_to(last_direction.rev());
            if let Some(new_direction) = new_direction {
                if let Some(next) = graph.step(pos, new_direction) {
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
    let mut graph = Graph::new();
    for line in input.lines() {
        graph.add_row(line.chars().map(|c| Field::new(c)).collect());
    }
    let start = graph.find_start();
    let mut result = None;
    let mut directions = vec![Direction::North, Direction::South, Direction::East, Direction::West];
    while result == None && directions.len() > 0 {
        let dir = directions.pop().unwrap();
        result = graph.find_path(start, dir);
    }
    let (path, inside) = result.unwrap();
    let mut q: Vec<Position> = inside
        .into_iter()
        .filter(|pos| !path.contains(pos))
        .unique()
        .collect();
    // make more inner nodes that touch
    let mut inner = Vec::new();
    while let Some(pos) = q.pop() {
        let neighbours: Vec<Position> = vec![Direction::North, Direction::South, Direction::East, Direction::West]
            .iter()
            .map(|dir| graph.slide(pos, *dir))
            .filter(|pos| pos.is_some())
            .map(|pos| pos.unwrap())
            .collect();
        for neighbour in neighbours {
            if !path.contains(&neighbour) && !inner.contains(&neighbour) && !q.contains(&neighbour) {
                q.push(neighbour);
            }
        }
        inner.push(pos);
    }


    inner.iter().count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_left_right1() {
        let f1 = Field::new('J');
        let (l, r) = f1.get_left_right(Direction::North);
        assert_eq!(l, vec![]);
        assert_eq!(r, vec![Direction::East, Direction::South]);
    }

    #[test]
    fn test_find_left_right2() {
        let f1 = Field::new('-');
        let (l, r) = f1.get_left_right(Direction::West);
        assert_eq!(r, vec![Direction::North]);
        assert_eq!(l, vec![Direction::South]);
    }
}