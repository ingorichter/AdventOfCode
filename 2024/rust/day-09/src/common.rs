use std::iter::repeat;

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    FreespaceBlock,
    FileBlock(usize),
    Undefined,
}

pub fn create_disc_layout(input: &str) -> Vec<BlockType> {
    input
        .trim()
        .chars()
        .collect::<Vec<_>>() // Convert to Vec<char> to use chunks
        .chunks(2)
        .enumerate()
        .flat_map(|(file_id, chunk)| {
            let file_and_space = match chunk {
                [a, b] => (a.to_digit(10).unwrap(), b.to_digit(10).unwrap()),
                [a] => (a.to_digit(10).unwrap(), 0),
                _ => unreachable!(),
            };
            
            // dbg!(file_and_space);

            let total_size = file_and_space.0 as usize + file_and_space.1 as usize;
            let mut files_and_spaces = Vec::with_capacity(total_size);
            files_and_spaces.extend(repeat(BlockType::FileBlock(file_id)).take(file_and_space.0 as usize));
            // there was no empty space at the end of the input
            files_and_spaces.extend(repeat(BlockType::FreespaceBlock).take(file_and_space.1 as usize));

            files_and_spaces
        })
        .collect()
}
