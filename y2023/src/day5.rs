use std::{collections::HashMap, str::Lines};

#[allow(dead_code)]
fn part_1(input: &str) -> u32 {
    let parsed_map = parse_input(input);
    parsed_map
        .0
        .iter()
        .map(|seed| parsed_map.1.seed_to_location(*seed))
        .min()
        .expect("Invalid state")
}

#[allow(dead_code)]
fn part_2(input: &str) -> u32 {
    input.len() as u32
}

#[derive(Debug)]
struct ParsedMappings {
    seed_to_soil: HashMap<u32, u32>,
    soil_to_fertilizer: HashMap<u32, u32>,
    fertilizer_to_water: HashMap<u32, u32>,
    water_to_light: HashMap<u32, u32>,
    light_to_temperature: HashMap<u32, u32>,
    temperature_to_humidity: HashMap<u32, u32>,
    humidity_to_location: HashMap<u32, u32>,
}

impl ParsedMappings {
    fn seed_to_location(&self, seed: u32) -> u32 {
        *self
            .seed_to_soil
            .get(&seed)
            .and_then(|soil| self.soil_to_fertilizer.get(soil).or(Some(soil)))
            .and_then(|fertilizer| {
                self.fertilizer_to_water
                    .get(fertilizer)
                    .or(Some(fertilizer))
            })
            .and_then(|water| self.water_to_light.get(water).or(Some(water)))
            .and_then(|light| self.light_to_temperature.get(light).or(Some(light)))
            .and_then(|temperature| {
                self.temperature_to_humidity
                    .get(temperature)
                    .or(Some(temperature))
            })
            .and_then(|humidity| self.humidity_to_location.get(humidity).or(Some(humidity)))
            .expect("Invalid seed")
    }
}

fn parse_input(input: &str) -> (Vec<u32>, ParsedMappings) {
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

fn parse_seeds(lines: &mut Lines<'_>) -> Vec<u32> {
    let seeds: Vec<u32> = lines
        .next()
        .unwrap()
        .split_once(": ")
        .expect("Invalid seeds input")
        .1
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    lines.next();
    seeds
}

fn parse_mapping(header: &str, lines: &mut Lines) -> HashMap<u32, u32> {
    lines
        .next()
        .filter(|line| line.starts_with(header))
        .expect("Invalid mapping header");
    let mut map = HashMap::new();
    loop {
        match lines.next() {
            Some(line) if line.is_empty() => break,
            None => break,
            Some(line) => {
                let mut line = line.split(' ');
                let from = line.next().unwrap().parse::<u32>().unwrap();
                let to = line.next().unwrap().parse::<u32>().unwrap();
                let size = line.next().unwrap().parse::<u32>().unwrap();
                (0..size).for_each(|index| {
                    map.insert(from + index, to + index);
                });
            }
        }
    }
    map
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
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"";
        assert_eq!(part_2(input), 0);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = include_str!("../input/day5.txt");
        assert_eq!(part_2(input), 0);
    }
}
