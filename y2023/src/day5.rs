use std::{ops::Range, str::Lines};

#[allow(dead_code)]
fn part_1(input: &str) -> u64 {
    let parsed_map = parse_input(input);
    parsed_map
        .0
        .iter()
        .map(|seed| parsed_map.1.seed_to_location(*seed))
        .min()
        .expect("Invalid state")
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    let parsed_map = parse_input(input);
    let seed_ranges: Vec<Range<u64>> = parsed_map
        .0
        .windows(2)
        .step_by(2)
        .map(|range| range[0]..range[0] + range[1])
        .collect();
    (0..u64::MAX)
        .map(|location| (location, parsed_map.1.location_to_seed(location)))
        .find(|(_, seed)| seed_ranges.iter().any(|range| range.contains(seed)))
        .map(|(location, _)| location)
        .expect("No seed found")
}

#[derive(Debug)]
struct ParsedMappings {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    size: u64,
}

impl ParsedMappings {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = ParsedMappings::find_destination(&self.seed_to_soil, seed);
        let fertilizer = ParsedMappings::find_destination(&self.soil_to_fertilizer, soil);
        let water = ParsedMappings::find_destination(&self.fertilizer_to_water, fertilizer);
        let light = ParsedMappings::find_destination(&self.water_to_light, water);
        let temperature = ParsedMappings::find_destination(&self.light_to_temperature, light);
        let humidity = ParsedMappings::find_destination(&self.temperature_to_humidity, temperature);
        ParsedMappings::find_destination(&self.humidity_to_location, humidity)
    }

    fn find_destination(mappings: &[Mapping], source: u64) -> u64 {
        mappings
            .iter()
            .find(|mapping| source >= mapping.source && source < mapping.source + mapping.size)
            .map(|mapping| mapping.destination + source - mapping.source)
            .unwrap_or(source)
    }

    fn location_to_seed(&self, location: u64) -> u64 {
        let humidity = ParsedMappings::find_source(&self.humidity_to_location, location);
        let temperature = ParsedMappings::find_source(&self.temperature_to_humidity, humidity);
        let light = ParsedMappings::find_source(&self.light_to_temperature, temperature);
        let water = ParsedMappings::find_source(&self.water_to_light, light);
        let fertilizer = ParsedMappings::find_source(&self.fertilizer_to_water, water);
        let soil = ParsedMappings::find_source(&self.soil_to_fertilizer, fertilizer);
        ParsedMappings::find_source(&self.seed_to_soil, soil)
    }

    fn find_source(mappings: &[Mapping], destination: u64) -> u64 {
        mappings
            .iter()
            .find(|mapping| {
                destination >= mapping.destination
                    && destination < mapping.destination + mapping.size
            })
            .map(|mapping| mapping.source + destination - mapping.destination)
            .unwrap_or(destination)
    }
}

fn parse_input(input: &str) -> (Vec<u64>, ParsedMappings) {
    let mut lines = input.lines();
    let seeds = parse_seeds(&mut lines);
    let seed_to_soil = parse_mapping("seed-to-soil map:", &mut lines);
    let soil_to_fertilizer = parse_mapping("soil-to-fertilizer map:", &mut lines);
    let fertilizer_to_water = parse_mapping("fertilizer-to-water map:", &mut lines);
    let water_to_light = parse_mapping("water-to-light map:", &mut lines);
    let light_to_temperature = parse_mapping("light-to-temperature map:", &mut lines);
    let temperature_to_humidity = parse_mapping("temperature-to-humidity map:", &mut lines);
    let humidity_to_location = parse_mapping("humidity-to-location map:", &mut lines);
    (
        seeds,
        ParsedMappings {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

fn parse_seeds(lines: &mut Lines<'_>) -> Vec<u64> {
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .expect("Invalid seeds input")
        .1
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    lines.next();
    seeds
}

fn parse_mapping(header: &str, lines: &mut Lines) -> Vec<Mapping> {
    lines
        .next()
        .filter(|line| line.starts_with(header))
        .expect("Invalid mapping header");
    let mut mappings = Vec::new();
    loop {
        match lines.next() {
            Some("") => break,
            None => break,
            Some(line) => {
                let mut line = line.split(' ');
                let destination = line.next().unwrap().parse::<u64>().unwrap();
                let source = line.next().unwrap().parse::<u64>().unwrap();
                let size = line.next().unwrap().parse::<u64>().unwrap();
                mappings.push(Mapping {
                    destination,
                    source,
                    size,
                });
            }
        }
    }
    mappings
}

#[cfg(test)]
mod tests {
    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part_1(input), 35);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = include_str!("../input/day5.txt");
        assert_eq!(part_1(input), 173706076);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part_2(input), 46);
    }

    #[test]
    fn solve_part_2_challenge() {
        //NOTE: running this with release mode takes 1.5 sec
        let input = include_str!("../input/day5.txt");
        assert_eq!(part_2(input), 11611182);
    }
}
