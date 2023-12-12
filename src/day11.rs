use std::cmp::max;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Galaxy (usize, usize);

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
    size: (usize, usize)
}

impl Universe {
    fn new() -> Self {
        Universe {
            galaxies: Vec::new(),
            size: (0, 0)
        }
    }

    fn add_galaxy(&mut self, galaxy: Galaxy) {
        self.size.0 = max(self.size.0, galaxy.0+1);
        self.size.1 = max(self.size.1, galaxy.1+1);
        self.galaxies.push(galaxy);
    }

    fn set_size(&mut self, size: (usize, usize)) {
        self.size = size;
    }

    // factor is not really a factor, it just adds the amount of extra space
    // to use it as a factor, use factor-1
    fn expand(&mut self, factor: usize) {
        let mut empty = (Vec::new(), Vec::new());
        // find all unused y values for empty space
        for x in 0..self.size.0 {
            if self.galaxies
                .iter()
                .filter(|g| g.0 == x)
                .count() == 0 {
                    empty.0.push(x);
                }
        }
        for y in 0..self.size.1 {
            if self.galaxies
                .iter()
                .filter(|g| g.1 == y)
                .count() == 0 {
                empty.1.push(y);
            }
        }
        // move galaxies
        for x in empty.0.iter().rev() {
            self.galaxies
                .iter_mut()
                .filter(|g| g.0 > *x)
                .for_each(|g| g.0 += factor);
        }
        for y in empty.1.iter().rev() {
            self.galaxies
                .iter_mut()
                .filter(|g| g.1 > *y)
                .for_each(|g| g.1 += factor);
        }
        // update size
        self.size.0 += empty.0.len() * factor;
        self.size.1 += empty.1.len() * factor;
    }

}

pub(crate) fn c1(input: String) -> String {
    let mut universe = Universe::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                universe.add_galaxy(Galaxy(x, y));
            }
        }
    }
    universe.expand(1);
    // iterate over all galaxy pairs
    // just count them double and half the result
    let sum = universe.galaxies
        .iter()
        .map(|g1| universe.galaxies
            .iter()
            .fold(0, |acc, g2| acc + (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs())
        )
        .sum::<i32>() / 2;
    sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let mut universe = Universe::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                universe.add_galaxy(Galaxy(x, y));
            }
        }
    }
    universe.expand(1000000-1);
    // iterate over all galaxy pairs
    // just count them double and half the result
    let sum = universe.galaxies
        .iter()
        .map(|g1| universe.galaxies
            .iter()
            .fold(0, |acc, g2| acc + (g1.0 as i128 - g2.0 as i128).abs() + (g1.1 as i128 - g2.1 as i128).abs())
        )
        .sum::<i128>() / 2;
    sum.to_string()
}