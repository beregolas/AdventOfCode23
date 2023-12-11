#[derive(Debug)]
struct History {
    data: Vec<Vec<i64>>,
}

impl History {
    fn new(initial: Vec<i64>) -> History {
        let mut data = vec![initial];
        while data.last()
            .unwrap()
            .iter()
            .any(|&x| x != 0) {
            let mut next_line = Vec::new();
            for pair in data.last().unwrap().windows(2) {
                // add the difference between the two numbers to the next line
                next_line.push(pair[1] - pair[0]);
            }
            data.push(next_line);
        }
        History {
            data
        }
    }

    fn next(&self) -> i64 {
        let mut acc = 0;
        for set in self.data.iter().rev() {
            acc += set.last().unwrap();
        }
        acc
    }

    fn prev(&self) -> i64 {
        let mut acc = 0;
        for set in self.data.iter().rev() {
            acc = set.first().unwrap() - acc;
        }
        acc
    }
}

pub(crate) fn c1(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let history = History::new(numbers);
        sum += history.next();
    }
    sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let history = History::new(numbers);
        sum += history.prev();
    }
    sum.to_string()
}