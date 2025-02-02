use std::collections::HashMap;

type EngravedNumber = u64;
type StonesCache = HashMap<(EngravedNumber, u32), u64>;

/*
    1. 0 -> 1
    2. even -> split in two and add to list
    3. odd -> multiply by 2024 and add to list
*/
#[tracing::instrument]
fn blink(stone: EngravedNumber, blinks: u32, cache: &mut StonesCache) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let key: (EngravedNumber, u32) = (stone, blinks);

    if let Some(possible_stones) = cache.get(&key) {
        return *possible_stones;
    }

    let number_as_string = stone.to_string();
    let number_digits = number_as_string.chars().count();

    let result = match stone {
        0 => blink(1, blinks - 1, cache),
        _ if number_digits % 2 == 0 => {
            let left_number = number_as_string[0..number_digits/2].parse::<EngravedNumber>().unwrap();
            let right_number = number_as_string[number_digits/2..number_digits].parse::<EngravedNumber>().unwrap();
            blink(left_number, blinks - 1, cache) + blink(right_number, blinks - 1, cache)
        },
        _ => blink(stone * 2024, blinks - 1, cache)
    };

    cache.insert(key, result);
    result
}

#[tracing::instrument]
fn read_input(input: &str) -> Vec<EngravedNumber> {
    input
        .split_whitespace()
        .map(|x| x.parse::<EngravedNumber>().unwrap())
        .collect()
}

#[tracing::instrument]
fn sum_of_blinks(stones: &Vec<EngravedNumber>, blinks: u32) -> u64 {
    let mut cache: StonesCache = HashMap::new();

    stones.iter().map(|stone| {
        blink(*stone, blinks,  &mut cache)
    }).sum()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let stones: Vec<EngravedNumber> = read_input(input);

    let total_stones: u64 = sum_of_blinks(&stones, 75);

    Ok(total_stones.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "not implemented"]
    fn test_process() -> miette::Result<()> {
        let input = "0 1 10 99 999";
        // let stones = read_input(input);
        let stones = sum_of_blinks(&read_input(input), 1);
        // 1 2024 1 0 9 9 2021976
        assert_eq!(7, stones);
        Ok(())
    }
}
