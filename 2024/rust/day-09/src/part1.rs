use std::iter::repeat;

#[derive(Debug, Clone)]
enum Type {
    Space,
    File(usize),
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let disc_layout: Vec<Type> = input
        .trim()
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
            result.extend(repeat(Type::File(i)).take(pair.0 as usize));
            result.extend(repeat(Type::Space).take(pair.1 as usize));
            result
        })
        .collect();

    // dbg!(&disc_layout);
    // find all indices of empty spaces in reverse order to easier remove them from the list
    let mut empty_space_indices: Vec<usize> = disc_layout
        .iter()
        .enumerate()
        .filter_map(|(i, t)| if let Type::Space = t { Some(i) } else { None })
        .rev()
        .collect();

    // dbg!(&empty_space_indices);

    // reverse iterate through disc_layout and move every file to the next free space at the beginning of disc_layout based on the zeroth element of empty_space_indices
    let checksum: usize = disc_layout
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(index, t)| match t {
            Type::File(file_id) => {
                // #[cfg(debug_assertions)]
                // println!("Found file at index {} with id {}", index, file_id);

                if let Some(target_index) = empty_space_indices.pop() {
                    // #[cfg(debug_assertions)]
                    // println!("Moving to target index {}", target_index);

                    Some(file_id * target_index)
                } else {
                    Some(file_id * index)
                }
            },
            Type::Space => {
                // #[cfg(debug_assertions)]
                // println!("Found space at index {}", index);

                if !empty_space_indices.is_empty() {
                    empty_space_indices.remove(0);    
                }
                None
            }
        })
        .sum();

    // dbg!(checksum);
    // dbg!(&disc_layout);

    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_simple() -> miette::Result<()> {
        let input = "12345";
        assert_eq!("60", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_5() -> miette::Result<()> {
        let input = "233313312141413140256";

        assert_eq!("3383", process(input)?);
        Ok(())
    }
}
