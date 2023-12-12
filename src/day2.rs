use std::cmp::max;



pub(crate) fn c1(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let id: u32 = parts[0].split(" ").collect::<Vec<&str>>()[1].parse().expect("This is not a number!");
        let games = parts[1].split(";").collect::<Vec<&str>>();
        // check all games for possibility (12 red, 13 green, 14 blue)
        let mut possible = true;
        for game in games {
            let game = game.trim();
            let draws = game.split(",").collect::<Vec<&str>>();
            for draw in draws {
                let draw = draw.trim();
                let color = draw.split(" ").collect::<Vec<&str>>()[1];
                let number = draw.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().expect("This is not a number!");
                if (color == "red" && number > 12) || (color == "green" && number > 13) || (color == "blue" && number > 14) {
                    possible = false;
                }
            }
        }
        if possible {
            sum += id;
        }
    }
    sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let _id: u32 = parts[0].split(" ").collect::<Vec<&str>>()[1].parse().expect("This is not a number!");
        let games = parts[1].split(";").collect::<Vec<&str>>();
        // check all games for lowest possible score
        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        for game in games {
            let game = game.trim();
            let draws = game.split(",").collect::<Vec<&str>>();
            for draw in draws {
                let draw = draw.trim();
                let color = draw.split(" ").collect::<Vec<&str>>()[1];
                let number = draw.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().expect("This is not a number!");
                match color {
                    "red" => {
                        red = max(red, number);
                    },
                    "green" => {
                        green = max(green, number);
                    },
                    "blue" => {
                        blue = max(blue, number);
                    },
                    _ => panic!("This is not a color!"),
                }
            }
        }
        sum += red * green * blue;
    }
    sum.to_string()
}
