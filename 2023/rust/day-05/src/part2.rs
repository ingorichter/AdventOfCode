use crate::custom_error::AocError;
use std::cmp;
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
    source: Range<T>,
    destination: Range<T>,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Range mapping
    // destination, source, range length

    const ARRAY_REPEAT_VALUE: Vec<Ranges<i64>> = Vec::new();
    let mut mappings: [Vec<Ranges<i64>>; NUM_MAPPINGS] = [ARRAY_REPEAT_VALUE; NUM_MAPPINGS];

    let mut seed_ranges: Vec<Range<i64>> = vec![];
    let mut current_mapping: usize = SEED_TO_SOIL;

    input.split("\n\n").for_each(|chunk| {
        chunk.lines().for_each(|line| {
            match line {
                line if line.starts_with("seeds:") => {
                    let (_, seed_str) = line.split_once(": ").unwrap();
                    let numbers: Vec<i64> = seed_str
                        .split_whitespace()
                        .filter_map(|num| num.parse().ok())
                        .collect();
                    // dbg!(&numbers);

                    seed_ranges = numbers
                        .chunks(2)
                        .filter_map(|chunk| {
                            if chunk.len() == 2 {
                                Some(Range {
                                    start: chunk[0],
                                    end: chunk[0] + chunk[1],
                                })
                            } else {
                                None
                            }
                        })
                        .collect();

                    // dbg!(&seeds);
                }
                line if line.starts_with("seed-to-soil map:") => current_mapping = SEED_TO_SOIL,
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
                    let values = line
                        .split_whitespace()
                        .map(|val| val.parse().unwrap())
                        .collect::<Vec<_>>();

                    let ranges = Ranges {
                        destination: Range {
                            start: values[0],
                            end: values[0] + values[2],
                        },
                        source: Range {
                            start: values[1],
                            end: values[1] + values[2],
                        },
                    };

                    mappings[current_mapping].push(ranges);
                }
            }
        })
    });

    // sort all ranges by start index
    mappings.iter_mut().for_each(|ranges| {
        ranges.sort_by(|a, b| a.source.start.cmp(&b.source.start));
    });

    let mut answer = 1 << 60;
    seed_ranges.into_iter().for_each(|current_seed_range| {
        let mut current_ranges = vec![current_seed_range];

        // println!("Len of mappings {:?}", mappings.len());
        for mapping in mappings.iter() {
            let mut temp_ranges = vec![];

            for current_range in &current_ranges {
                let mapped_ranges = map_seed_ranges(current_range, mapping);
                temp_ranges.push(mapped_ranges.clone());
            }

            current_ranges = vec![];
            temp_ranges.into_iter().flatten().for_each(|r| {
                current_ranges.push(r.clone());
            });
        }

        for range in current_ranges.iter() {
            answer = cmp::min(answer, range.start);
        }
    });

    return Ok(answer.to_string());
}

fn map_seed_ranges(seed_range: &Range<i64>, mappings: &[Ranges<i64>]) -> Vec<Range<i64>> {
    let mut answers = vec![];
    let mut results: Vec<Range<i64>> = vec![];

    mappings.iter().for_each(|m: &Ranges<i64>| {
        let end = m.source.end;
        let distance = m.destination.start - m.source.start;

        if !(end < seed_range.start || m.source.start > seed_range.end) {
            answers.push((
                cmp::max(m.source.start, seed_range.start),
                cmp::min(end, seed_range.end),
                distance,
            ))
        }
    });

    if answers.is_empty() {
        return vec![seed_range.clone()];
    }

    answers.iter().enumerate().for_each(|(index, answer)| {
        results.push(Range {
            start: answer.0 + answer.2,
            end: answer.1 + answer.2,
        });

        if index < answers.len() - 1 && answers[index + 1].0 > answer.1 {
            results.push(Range {
                start: answer.1 + 1,
                end: answers[index + 1].0 - 1,
            });
        }
    });

    if answers[0].0 != seed_range.start {
        results.push(Range {
            start: seed_range.start,
            end: answers[0].0 - 1,
        });
    }

    if answers[answers.len() - 1].1 != seed_range.end {
        results.push(Range {
            start: answers[answers.len() - 1].1,
            end: seed_range.end,
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART2: &str = "seeds: 79 14 55 13

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
        assert_eq!("46", process(TEST_DATA_PART2)?);
        Ok(())
    }
}
