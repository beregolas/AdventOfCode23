
fn get_data_1(line: &str) -> Vec<u32> {
   line.split(" ")
       .skip(1)
       .filter(|w| w.len() > 0)
       .map(|w| w.parse::<u32>().unwrap())
       .collect::<Vec<u32>>()
}

fn get_data_2(line: &str) -> u128 {
   line.split(" ")
       .skip(1)
       .filter(|w| w.len() > 0)
       .collect::<Vec<&str>>()
       .join("")
       .parse::<u128>()
       .unwrap()
}

pub(crate) fn c1(input: String) -> String {
   let mut lines = input.lines();
   let times = get_data_1(lines.next().unwrap());
   let distances = get_data_1(lines.next().unwrap());
   // iterate over all races
   let mut sum = 1;
   for (&t, &d) in times.iter().zip(distances.iter()) {
      // iterate over all possible times
      let mut winning_times: u32 = 0;
      for i in 1..t {
         let race_distance = (t-i) * i;
         if race_distance > d {
            winning_times += 1;;
         }
      }
      println!("{}...{} -> {}", t, d, winning_times);
      sum *= winning_times;
   }
   sum.to_string()
}

pub(crate) fn c2(input: String) -> String {
   let mut lines = input.lines();
   let time = get_data_2(lines.next().unwrap());
   let distance = get_data_2(lines.next().unwrap());
   // only one race
   let mut winning_times: u128 = 0;
   for i in 1..time {
      let race_distance = (time-i) * i;
      if race_distance > distance {
         winning_times += 1;;
      }
   }
    winning_times.to_string()
}
