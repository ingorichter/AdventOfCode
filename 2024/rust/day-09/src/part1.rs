use std::iter::repeat;

#[derive(Debug, Clone)]
enum Type {
    Space,
    File(u32),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let disc_layout: Vec<Type> = input
        .chars()
        .collect::<Vec<_>>() // Convert to Vec<char> to use chunks
        .chunks(2)
        .enumerate()
        .flat_map(|(i, chunk)| {
            let pair = match chunk {
                [a, b] => (a.to_digit(10).unwrap(), b.to_digit(10).unwrap()),
                [a] => (a.to_digit(10).unwrap(), 0),
                _ => unreachable!(),
            };

            let total_size = pair.0 as usize + pair.1 as usize;
            let mut result = Vec::with_capacity(total_size);
            result.extend(repeat(Type::File(i.try_into().unwrap())).take(pair.0 as usize));
            result.extend(repeat(Type::Space).take(pair.1 as usize));
            result
        })
        .collect();

    dbg!(&disc_layout);

    Ok("1929".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple() -> miette::Result<()> {
        let input = "12345";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
