use std::cmp::min;

#[derive(Debug, Clone)]
struct Almanac {
    seed_to_soil: Vec<(usize, usize, isize)>,
    soil_to_fertilizer: Vec<(usize, usize, isize)>,
    fertilizer_to_water: Vec<(usize, usize, isize)>,
    water_to_light: Vec<(usize, usize, isize)>,
    light_to_temperature: Vec<(usize, usize, isize)>,
    temperature_to_humidity: Vec<(usize, usize, isize)>,
    humidity_to_location: Vec<(usize, usize, isize)>,
}

impl Almanac {

    fn new() -> Almanac {
        Almanac {
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }

    fn add_translation(&mut self, output_start: usize, input_start: usize, length: usize, name: &str) {
        let translation = (input_start, input_start+length, output_start as isize - input_start as isize);

        match name {
            "seed-to-soil" => self.seed_to_soil.push(translation),
            "soil-to-fertilizer" => self.soil_to_fertilizer.push(translation),
            "fertilizer-to-water" => self.fertilizer_to_water.push(translation),
            "water-to-light" => self.water_to_light.push(translation),
            "light-to-temperature" => self.light_to_temperature.push(translation),
            "temperature-to-humidity" => self.temperature_to_humidity.push(translation),
            "humidity-to-location" => self.humidity_to_location.push(translation),
            x => panic!("This is not a valid translation! {:?}", x)
       }
    }

    fn translate(&self, input: usize, translation: &Vec<(usize, usize, isize)>) -> usize {
        let translation = translation
            .iter()
            .find(|x| x.0 <= input && input < x.1);
        match translation {
            // add offset to input
            Some(x) => (input as isize + x.2) as usize,
            // else return input unchanged
            None => input
        }
    }

    fn get_location_from_seed(&self, seed: usize) -> usize {
        let soil = self.translate(seed, &self.seed_to_soil);
        let fertilizer = self.translate(soil, &self.soil_to_fertilizer);
        let water = self.translate(fertilizer, &self.fertilizer_to_water);
        let light = self.translate(water, &self.water_to_light);
        let temperature = self.translate(light, &self.light_to_temperature);
        let humidity = self.translate(temperature, &self.temperature_to_humidity);
        let location = self.translate(humidity, &self.humidity_to_location);
        location
    }
}



pub(crate) fn c1(input: String) -> String {
    let mut almanac = Almanac::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut category = "";
    for line in input.lines() {
        // check if line starts with number, letter or is empty
        // line is empty -> skip it
        if line.len() == 0 {
            continue;
        }
        // line start with a letter -> change category
        if line.chars().next().unwrap().is_alphabetic() {
            // if line starts with "seeds:", add initial seeds
            if line.starts_with("seeds:") {
                let mut seed_list = line.split(" ").skip(1);
                for seed in seed_list {
                    seeds.push(seed.parse::<usize>().unwrap());
                }
            }
            category = line.split(" ").next().unwrap();
            continue;
        }
        // line starts with a number -> add translation
        let mut translation = line.split(" ");
        almanac.add_translation(
            translation.next().unwrap().parse::<usize>().unwrap(),
            translation.next().unwrap().parse::<usize>().unwrap(),
            translation.next().unwrap().parse::<usize>().unwrap(),
            category
        );

    }
    // println!("{:?}", almanac);
    let mut lowest_location = usize::MAX;
    for seed in seeds {
        let location = almanac.get_location_from_seed(seed);
        println!("{} -> {}", seed, location);
        lowest_location = min(lowest_location, location);
    }
    lowest_location.to_string()
}

pub(crate) fn c2(input: String) -> String {
    let mut almanac = Almanac::new();
    let mut seeds: Vec<usize> = Vec::new();
    let mut category = "";
    for line in input.lines() {
        // check if line starts with number, letter or is empty
        // line is empty -> skip it
        if line.len() == 0 {
            continue;
        }
        // line start with a letter -> change category
        if line.chars().next().unwrap().is_alphabetic() {
            // if line starts with "seeds:", add initial seeds
            if line.starts_with("seeds:") {
                let mut seed_list = line.split(" ").skip(1);
                for seed in seed_list {
                    seeds.push(seed.parse::<usize>().unwrap());
                }
            }
            category = line.split(" ").next().unwrap();
            continue;
        }
        // line starts with a number -> add translation
        let mut translation = line.split(" ");
        almanac.add_translation(
            translation.next().unwrap().parse::<usize>().unwrap(),
            translation.next().unwrap().parse::<usize>().unwrap(),
            translation.next().unwrap().parse::<usize>().unwrap(),
            category
        );

    }
    // println!("{:?}", almanac);
    // start 10 threads
    let mut threads = Vec::new();
    for slice in seeds.chunks(2) {
        let almanac = almanac.clone();
        let slice = slice.to_vec();
        println!("started thread with {:?} seeds", slice[1]);
        threads.push(std::thread::spawn(move || {
            let mut lowest_location = usize::MAX;
            for seed in slice[0]..slice[0]+slice[1] {
                let location = almanac.get_location_from_seed(seed);
                lowest_location = min(lowest_location, location);
            }
            lowest_location
        }));
    }
    // wait for all threads to finish
    let mut lowest_location = usize::MAX;
    for thread in threads {
        let location = thread.join().unwrap();
        println!("thread finished with location {}", location);
        lowest_location = min(lowest_location, location);
    }
    lowest_location.to_string()
}
