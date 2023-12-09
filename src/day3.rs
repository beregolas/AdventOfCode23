use std::cmp::{max, min};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hit {
    number: u32,
    line: usize,
    columns: (usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Hit {
    fn is_part_number(&self, array: &Vec<Vec<char>>) -> bool {
        let mut symbols = "".to_string();
        // get all symbols in the area around the number
        for i in if self.line > 0 {self.line-1} else {self.line}
            ..=if self.line+1 < array.len() {self.line+1} else {self.line} {
            symbols += &*array
                [max(0, i)]
                [(if self.columns.0 > 0 {self.columns.0-1} else {self.columns.0})..min(array[self.line].len()-1, self.columns.1 + 1)].iter().collect::<String>();
        }
        // remove all digits from symbols
        symbols = symbols.replace(|c: char| c.is_digit(10), "");
        // remove all dots from symbols
        symbols = symbols.replace(".", "");
        // check if any symbols remain
        symbols.len() > 0
    }

    fn find_gear(&self, array: &Vec<Vec<char>>) -> Vec<Position> {
        // get all gear symbols in the area around the number
        let mut gears = Vec::new();
        // compute column indices:
        let columns = (
            if self.columns.0 > 0 {self.columns.0-1} else {self.columns.0},
            if self.columns.1+1 < array[self.line].len() {self.columns.1 + 1} else {self.columns.1},
        );
        // check the line above if it exists
        let lines = (
            if self.line > 0 {self.line-1} else {self.line},
            if self.line+1 < array.len() {self.line+1} else {self.line},
        );
        for line in lines.0..=lines.1 {
            for column in columns.0..columns.1 {
                if array[line][column] == '*' {
                    gears.push(Position(line, column));
                }
            }
        }
        gears
    }
}

pub(crate) fn c1(input: String) -> String {
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for (index, line) in input.lines().enumerate() {
        // check for numbers with regex
        let re = Regex::new(r"\d+").unwrap();
        let numbers: Vec<Hit> = re.find_iter(line)
            .map(|x| Hit{
                number: x.as_str().parse().expect("This is not a number!"),
                line: index,
                columns: (x.start(), x.end()),
            }).collect();
        // check if any of the numbers is a part number
        for number in numbers {
            if number.is_part_number(&array) {
                sum += number.number;
            }
        }

    }
    sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut gears: HashMap<Position, Vec<Hit>> = HashMap::new();
    for (index, line) in input.lines().enumerate() {
        // check for numbers with regex
        let re = Regex::new(r"\d+").unwrap();
        let numbers: Vec<Hit> = re.find_iter(line)
            .map(|x| Hit{
                number: x.as_str().parse().expect("This is not a number!"),
                line: index,
                columns: (x.start(), x.end()),
            }).collect();
        // get all gear numbers
        for number in numbers {
            let gear_hits = number.find_gear(&array);
            for gear in gear_hits {
                if !gears.contains_key(&gear) {
                    gears.insert(gear, Vec::new());
                }
                gears.get_mut(&gear).unwrap().push(number);
            }
        }
    }
    println!("{:?}", gears);
    for (gear, hits) in gears {
        if hits.len() == 2 {
            sum += hits[0].number * hits[1].number;
        }
    }
    sum.to_string()
}
