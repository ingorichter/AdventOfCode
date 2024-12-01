#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let line_count: u32 = input.lines().count().try_into().unwrap();
    let mut list1 = Vec::with_capacity(line_count as usize);
    let mut list2 = Vec::with_capacity(line_count as usize);

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        list1.push(parts.first().unwrap().parse::<i32>().unwrap());
        list2.push(parts.last().unwrap().parse::<i32>().unwrap());
    });

    let similarity = list1.iter().zip(list2.iter()).fold(0, |acc, (a, _b)| {
        let factor = list2.iter().filter(|&x| x == a).count();

        acc + factor as i32 * a
    });

    return Ok(similarity.to_string());}

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
        assert_eq!("31", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
