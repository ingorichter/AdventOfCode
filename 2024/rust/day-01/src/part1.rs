#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut dist: i32 = 0;

    let line_count: u32 = input.lines().count().try_into().unwrap();
    let mut list1 = Vec::with_capacity(line_count as usize);
    let mut list2 = Vec::with_capacity(line_count as usize);

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        list1.push(parts.first().unwrap().parse::<i32>().unwrap());
        list2.push(parts.last().unwrap().parse::<i32>().unwrap());
    });

    list1.sort();
    list2.sort();

    let pairs: Vec<_> = list1.iter().zip(list2.iter()).collect();    

    for (a, b) in pairs {
        dist += (a - b).abs();
    }

    return Ok(dist.to_string());}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("11", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
