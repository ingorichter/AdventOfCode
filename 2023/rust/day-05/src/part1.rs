use crate::custom_error::AocError;
use std::ops::Range;

const SEED_TO_SOIL: usize = 0;
const SOIL_TO_FERTILIZER: usize = 1;
const FERTILIZER_TO_WATER: usize = 2;
const WATER_TO_LIGHT: usize = 3;
const LIGHT_TO_TEMPERATURE: usize = 4; 
const TEMPERATURE_TO_HUMIDITY: usize = 5;
const HUMIDITY_TO_LOCATION: usize = 6;

const NUM_MAPPINGS: usize = 7;

#[derive(Debug)]
struct Ranges<T> {
    destination: Range<T>,
    source: Range<T>
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    // Range mapping
    // destination, source, range length

    const ARRAY_REPEAT_VALUE: Vec<Ranges<u64>> = Vec::new();
    let mut mappings: [Vec<Ranges<u64>>; NUM_MAPPINGS] = [ARRAY_REPEAT_VALUE; NUM_MAPPINGS];

    let mut seeds: Vec<u64> = vec![];
    let mut current_mapping: usize = SEED_TO_SOIL;

    input.split("\n\n").for_each(|chunk| {
        chunk.lines().for_each(|line| {
            match line {
                line if line.starts_with("seeds:") => {
                    let (_, seed_str) = line.split_once(": ").unwrap();
                    seeds = seed_str.split_whitespace().map(|val| val.parse().unwrap()).collect::<Vec<_>>();
                }
                line if line.starts_with("seed-to-soil map:") => {
                    current_mapping = SEED_TO_SOIL
                }
                line if line.starts_with("soil-to-fertilizer map:") => {
                    current_mapping = SOIL_TO_FERTILIZER
                }
                line if line.starts_with("fertilizer-to-water map:") => {
                    current_mapping = FERTILIZER_TO_WATER
                }
                line if line.starts_with("water-to-light map:") => {
                    current_mapping = WATER_TO_LIGHT;
                }
                line if line.starts_with("light-to-temperature map:") => {
                    current_mapping = LIGHT_TO_TEMPERATURE;
                }
                line if line.starts_with("temperature-to-humidity map:") => {
                    current_mapping = TEMPERATURE_TO_HUMIDITY;
                }
                line if line.starts_with("humidity-to-location map:") => {
                    current_mapping = HUMIDITY_TO_LOCATION;
                }
                &_ => {
                    let values = line.split_whitespace().map(|val| val.parse().unwrap()).collect::<Vec<_>>();

                    let ranges = Ranges {
                        destination: Range { start: values[0], end: values[0] + values[2] },
                        source: Range { start: values[1], end: values[1] + values[2] },
                    };

                    mappings[current_mapping].push(ranges);
                }
            }
        })
    });

    let mut locations: Vec<_> = vec![];
    seeds.iter().for_each(|seed| {
        // dbg!(seed);
        let mut current_value = *seed;

        for item in mappings.iter().enumerate() {
            let (index, ranges) = item;
            for range in ranges {
                if range.source.contains(&current_value) {
                    // dbg!(range);
                    let delta = current_value - range.source.start;
                    let value = range.destination.start + delta;
                    // println!("Destination for {} is {} in Map {}", &current_value, value, index);
                    current_value = value;
                    break;
                } else {
                    //println!("No match for {} in Map {}", &current_value, index);
                }
            }
        };

        locations.push(current_value);
    });

    // dbg!(locations);
    let result = locations.iter().min().unwrap().to_owned();
    return Ok(result.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "seeds: 79 14 55 13

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
    
    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("35", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
