use std::collections::HashMap;

pub fn extract_digits(line: &str) -> Vec<char> {
    return line.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub fn extract_digits2(line: &str) -> Vec<char> {
    let map = HashMap::from([
        ("one".to_string(), "1"),
        ("two".to_string(), "2"),
        ("three".to_string(), "3"),
        ("four".to_string(), "4"),
        ("five".to_string(), "5"),
        ("six".to_string(), "6"),
        ("seven".to_string(), "7"),
        ("eight".to_string(), "8"),
        ("nine".to_string(), "9"),
        ]);

    let mut digits = Vec::new();
    println!("{}", line);

    let line_length = line.len();
    // println!("{}", line_length);
    for (index, c) in line.char_indices() {
        // println!("{}-{}", index, c);
        // do something with character `c` and index `i`
        if c.is_ascii_digit() {
            // println!("Found digit {}", c);
            digits.push(c);
        } else {
            let mut hit = false;
            let remaining_chars = line_length - index;
            // println!("Remaining chars {}", remaining_chars);
            if remaining_chars >= 3 {
                let s = &line[index..index+3];
                // println!("Found string {}", s);
                let digit = map.get(s);
                if digit.is_some() {
                    digits.push(digit.unwrap().chars().next().unwrap());
                    hit = true;
                }
            }

            if !hit && remaining_chars >= 4 {
                let s = &line[index..index+4];
                // println!("Found string {}", s);
                let digit = map.get(s);
                if digit.is_some() {
                    digits.push(digit.unwrap().chars().next().unwrap());
                    hit = true;
                }
            }

            if !hit && remaining_chars >= 5 {
                let s = &line[index..index+5];
                // println!("Found string {}", s);
                let digit = map.get(s);
                if digit.is_some() {
                    digits.push(digit.unwrap().chars().next().unwrap());
                }
            }
        }
    }

    // println!("Digits {:?}", digits);
    return digits;
}

pub fn process_part1(input: &str) -> String {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let digits = extract_digits(line);
        let first = digits.first();
        let last = digits.last();
        let number = format!("{}{}", first.unwrap(), last.unwrap());
        sum += number.parse::<u32>().unwrap();
    });
    return sum.to_string();
}

pub fn process_part2(input: &str) -> String {
    let mut sum = 0;

    input.lines().for_each(|line| {
        let digits = extract_digits2(line);
        let first = digits.first();
        let last = digits.last();
        let number = format!("{}{}", first.unwrap(), last.unwrap());
        sum += number.parse::<u32>().unwrap();
    });
    return sum.to_string();
}

// one
// two
// three
// four
// five
// six
// seven
// eight
// nine

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_PART1: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    
    const TEST_DATA_PART2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part1() {
        let result = process_part1(TEST_DATA_PART1);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2() {
        let result = process_part2(TEST_DATA_PART2);
        assert_eq!(result, "281");
    }
}
