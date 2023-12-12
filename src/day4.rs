
fn get_line_numbers(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    // split line into card number and list of numbers
    let mut contents = line.split(":");
    let index = contents.next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
    let mut numbers = contents.next().unwrap().split("|");
    let winners = numbers.next().expect("missing delimiter").split(" ").filter_map(
        |x| if x.len() > 0 {Some(x.parse::<u32>().unwrap())} else {None}
    ).collect();
    let chosen = numbers.next().expect("missing delimiter").split(" ").filter_map(
        |x| if x.len() > 0 {Some(x.parse::<u32>().unwrap())} else {None}
    ).collect();
    (index, winners, chosen)
}

fn get_winner_amount(winners: &Vec<u32>, chosen: &Vec<u32>) -> u32 {
    let mut winner_sum = 0;
    for c in chosen {
        if winners.contains(&c) {
            winner_sum += 1;
        }
    }
    winner_sum
}

pub(crate) fn c1(input: String) -> String {
    let mut total_sum = 0;
    for line in input.lines() {
        let (_idx, winners, chosen) = get_line_numbers(line);
        let winner_sum = get_winner_amount(&winners, &chosen);
        if winner_sum > 0 {
            total_sum += 2_u32.pow(winner_sum-1);
        }
    }
   total_sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
    // make a vector of all the lines as a reference to get the original lines
    let lines: Vec<(u32, u32)> = input
        .lines()
        .map(|l| get_line_numbers(l))
        .map(|(i, w, c)| (i, get_winner_amount(&w, &c)))
        .collect();
    // make a vector of all lines as working copy
    let mut working_lines = lines.clone();
    // iterate over all lines
    let mut sum = 0;
    while let Some(line) = working_lines.pop() {
        // increase sum by 1
        sum += 1;
        // get line index
        let index = line.0;
        // append new lines by winner amount
        working_lines.append(lines[index as usize..(index+line.1) as usize].to_vec().as_mut());
    }
    sum.to_string()

}
