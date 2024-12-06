fn all_pairs<T>(vec: &[T]) -> impl Iterator<Item = (&T, &T)> {
    vec.iter()
        .enumerate()
        .flat_map(move |(i, x)| vec.iter().skip(i + 1).map(move |y| (x, y)))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut rules: Vec<(u32, u32)> = vec![]; // pair of u32
                                             // let mut rules: HashMap<u32, u32> = HashMap::new(); // pair of u32
    let mut pages_to_update: Vec<Vec<u32>> = vec![]; // page updates

    let mut lines = input.lines();

    // Step 1: Parse pairs until an empty line is encountered
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        if let Some((first, second)) = line.split_once('|') {
            let a = first.trim().parse::<u32>().unwrap();
            let b = second.trim().parse::<u32>().unwrap();
            // rules.insert(a, b);
            rules.push((a, b));
        }
    }

    // Step 2: Parse remaining lines into groups of Vec<u32>
    for line in lines {
        if !line.trim().is_empty() {
            let group = line
                .split(',')
                .map(|num| num.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            pages_to_update.push(group);
        }
    }

    // instead of copying the valid update, I could probably filter the existing updates and return only the valid ones
    let mut invalid_updates: Vec<Vec<u32>> = pages_to_update
        .iter()
        .filter(|update| all_pairs(update).any(|pair| !rules.contains(&(*pair.0, *pair.1))))
        .cloned()
        .collect::<Vec<Vec<u32>>>(); // let invalid_updates = pages_to_update.iter().filter(|update| {

    for update in &mut invalid_updates {
        for start_index in 0..update.len() - 1 {
            for index in start_index..update.len() - 1 {
                let pair = (update[start_index], update[index + 1]);

                if !rules.contains(&pair) {
                    // check if the reverse pair is in rules
                    let reverse_pair = (update[index + 1], update[start_index]);
                    if rules.contains(&reverse_pair) {
                        update.swap(start_index, index + 1);
                    }
                }
            }
        }
    }

    let sum = invalid_updates
        .iter()
        .map(|update| update.get(update.len() / 2).copied().unwrap_or_default())
        .sum::<u32>();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
