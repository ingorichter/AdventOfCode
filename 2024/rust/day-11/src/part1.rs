type EngravedNumber = u64;

#[tracing::instrument]
pub fn process_stones(list_of_stones: &Vec<EngravedNumber>) -> Vec<EngravedNumber> {
    let mut new_list_of_stones: Vec<EngravedNumber> = Vec::new();

    list_of_stones.iter().for_each(|stone| {
        let number_as_string = stone.to_string();
        let number_digits = number_as_string.chars().count();

        if *stone == 0  {
            new_list_of_stones.push(1);
        } else if number_digits % 2 == 0 {
            let left_number =  number_as_string[0..number_digits/2].parse::<EngravedNumber>().unwrap();
            let right_number = number_as_string[number_digits/2..number_digits].parse::<EngravedNumber>().unwrap();
            new_list_of_stones.push(left_number);
            new_list_of_stones.push(right_number);
        } else {
            new_list_of_stones.push(*stone * 2024);
        }
    });

    new_list_of_stones
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut stones: Vec<EngravedNumber> = input
        .split_whitespace()
        .map(|x| x.parse::<EngravedNumber>().unwrap())
        .collect();

    for _ in 0..25 {
        stones = process_stones(&stones);
    }

    // dbg!(&stones);

    Ok(stones.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "not implemented"]
    fn test_process() -> miette::Result<()> {
        let input = "0 1 10 99 999";
        // 1 2024 1 0 9 9 2021976
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
