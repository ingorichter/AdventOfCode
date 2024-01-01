use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<&str> = input.split('\n').collect();
    
    let times: Vec<i32> = lines[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distance: Vec<i32> = lines[1].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

    let mut results: Vec<usize> = vec![0; times.len()];
    for (index, time) in times.iter().enumerate() {
        for t in 0..*time {
            let remaining_time = time - t;
            if t * remaining_time > distance[index] {
                results[index] += 1;
            }
        }
    };

    let product = results.iter().product::<usize>();

    return Ok(product.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("288", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
