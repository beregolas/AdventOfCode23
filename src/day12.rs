


pub(crate) fn c1(input: String) -> String {
    for line in input.lines() {
        let chars = *line.split(" ").collect::<Vec<&str>>().first().unwrap();
        println!("{}", line);
    }
    "sum".to_string()
}

pub(crate) fn c2(input: String) -> String {
    "sum".to_string()
}