mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    println!("Welcome to the Advent Solver!");
    println!("Please enter the day you would like to solve (format: [daynumber]-[challengenumber]):");
    // Get the day-challenge input from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    let mut input = input.split("-");
    let day = input.next().unwrap().parse::<u8>().unwrap();
    let challenge = input.next().unwrap().parse::<u8>().unwrap();
    // read the corresponding file
    let filename = format!("input/{}-{}.txt", day, challenge);
    let contents = std::fs::read_to_string(filename).expect("This file does not exist!");
    // solve the challenge
    match (day, challenge) {
        (1, 1) => {
            println!("The solution to day 1, challenge 1 is: {}", day1::c1(contents));
        }
        (1, 2) => {
            println!("The solution to day 1, challenge 2 is: {}", day1::c2(contents));
        }
        (2, 1) => {
            println!("The solution to day 2, challenge 1 is: {}", day2::c1(contents));
        }
        (2, 2) => {
            println!("The solution to day 2, challenge 2 is: {}", day2::c2(contents));
        }
        (3, 1) => {
            println!("The solution to day 3, challenge 1 is: {}", day3::c1(contents));
        }
        (3, 2) => {
            println!("The solution to day 3, challenge 2 is: {}", day3::c2(contents));
        }
        (4, 1) => {
            println!("The solution to day 4, challenge 1 is: {}", day4::c1(contents));
        }
        (4, 2) => {
            println!("The solution to day 4, challenge 2 is: {}", day4::c2(contents));
        }
        (5, 1) => {
            println!("The solution to day 5, challenge 1 is: {}", day5::c1(contents));
        }
        (5, 2) => {
            println!("The solution to day 5, challenge 2 is: {}", day5::c2(contents));
        }
        (6, 1) => {
            println!("The solution to day 6, challenge 1 is: {}", day6::c1(contents));
        }
        (6, 2) => {
            println!("The solution to day 6, challenge 2 is: {}", day6::c2(contents));
        }
        (7, 1) => {
            println!("The solution to day 7, challenge 1 is: {}", day7::c1(contents));
        }
        (7, 2) => {
            println!("The solution to day 7, challenge 2 is: {}", day7::c2(contents));
        }
        (8, 1) => {
            println!("The solution to day 8, challenge 1 is: {}", day8::c1(contents));
        }
        (8, 2) => {
            println!("The solution to day 8, challenge 2 is: {}", day8::c2(contents));
        }
        (9, 1) => {
            println!("The solution to day 9, challenge 1 is: {}", day9::c1(contents));
        }
        (9, 2) => {
            println!("The solution to day 9, challenge 2 is: {}", day9::c2(contents));
        }
        (10, 1) => {
            println!("The solution to day 10, challenge 1 is: {}", day10::c1(contents));
        }
        (10, 2) => {
            println!("The solution to day 10, challenge 2 is: {}", day10::c2(contents));
        }
        (_, _) => {
            println!("This challenge has not been solved yet!");
        }
    }
}


