use regex::Regex;


fn word_to_digit(word: &str) -> u32 {
match word {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("This is not a digit!"),
    }
}


pub(crate) fn c1(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last_digit = line.chars().rfind(|c| c.is_digit(10)).unwrap();
        sum += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }
    sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let re_f = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_b = Regex::new(&(r"d\|one|two|three|four|five|six|seven|eight|nine").chars().rev().collect::<String>()).unwrap();
    let mut sum:u32 = 0;
    for line in input.lines() {
        let first_digit = word_to_digit(re_f.find(line).unwrap().as_str());
        let rev_line: String = line.chars().rev().collect();
        let last_digit = word_to_digit(re_b.find(&rev_line).unwrap().as_str().chars().rev().collect::<String>().as_str());
        sum += first_digit * 10 + last_digit;
        println!("{}{} - {}", first_digit, last_digit, line);
    }
    sum.to_string()
}
