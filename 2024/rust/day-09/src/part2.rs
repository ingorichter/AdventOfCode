use std::iter::repeat;

#[derive(Debug, Clone, PartialEq)]
enum Type {
    FreespaceBlock,
    FileBlock(usize),
    Undefined,
}

const FREE_SPACE_ID: usize = usize::MAX;

#[derive(Debug, Clone)]
struct File {
    start: usize,
    length: usize,
    file_id: usize,
}

impl File {
    fn checksum(&self) -> usize {
        (self.start..self.start + self.length).map(|i| self.file_id * i).sum()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let disc_layout: Vec<Type> = create_disc_layout(input);

    dbg!(&disc_layout);

    let files = make_file_list(&disc_layout);

    dbg!(&files.len());

    // reverse iterate through disc_layout and move every file to the next free space at the beginning of disc_layout based on the zeroth element of empty_space_indices
    let mut free_space: Vec<File> = files
        .iter()
        .filter_map(|f| {
            if f.file_id == FREE_SPACE_ID {
                Some(f.clone())
            } else {
                None
            }
        })
        .collect();

    let files_to_move: Vec<File> = files
        .iter()
        .filter_map(|f| {
            if f.file_id != FREE_SPACE_ID {
                Some(f.clone())
            } else {
                None
            }
        })
        .collect();

    // dbg!(&free_space);
    // dbg!(&files_to_move);

    // Move all blocks from the files_to_move list to the first free space in free_space
    // if there is no free space for this file, then calculate the checksum for that file based on
    // the current position, otherwise "move" the file the new location, calculate the checksum and
    // update the free space entry to reflect that this space is now smaller or entirely gone

    println!("===> Moving files to free space");
    let checksums: Vec<_> = files_to_move
        .iter()
        .rev()
        .map(|file_to_move| {
            println!("Try to move file {:?}", file_to_move);

            let moved_file = move_file_to_free_space(&mut free_space, file_to_move);

            if moved_file.start != file_to_move.start {
                println!("Successfully moved {:?} to {:?}", file_to_move, moved_file);
                dbg!(&moved_file);
            } else {
                println!("Failed to move {:?} to {:?}", file_to_move, moved_file);
            }
            
            moved_file.checksum()
        })
        .collect();

    // dbg!(&checksums);
    let checksum: usize = checksums.iter().sum();
    
    Ok(checksum.to_string())
}

fn create_disc_layout(input: &str) -> Vec<Type> {
    input
        .trim()
        .chars()
        .collect::<Vec<_>>() // Convert to Vec<char> to use chunks
        .chunks(2)
        .enumerate()
        .flat_map(|(i, chunk)| {
            let mut pair = match chunk {
                [a, b] => (a.to_digit(10).unwrap(), b.to_digit(10).unwrap()),
                [a] => (a.to_digit(10).unwrap(), u32::MAX),
                _ => unreachable!(),
            };

            if pair.1 == 0 {
                pair.1 = 1;
            }

            let total_size = pair.0 as usize + pair.1 as usize;
            let mut result = Vec::with_capacity(total_size);
            result.extend(repeat(Type::FileBlock(i)).take(pair.0 as usize));
            // there was no empty space at the end of the input
            if pair.1 != u32::MAX {
                result.extend(repeat(Type::FreespaceBlock).take(pair.1 as usize));
            }

            result
        })
        .collect()
}

fn move_file_to_free_space(free_space: &mut [File], file_to_move: &File) -> File {
    for free_space in free_space.iter_mut() {
        if free_space.length >= file_to_move.length {
            #[cfg(debug_assertions)]
            println!("Found free space {:?} for {:?}", free_space, file_to_move);
            let moved_file = File {
                start: free_space.start,
                length: file_to_move.length,
                file_id: file_to_move.file_id,
            };

            // update the empty space, perhaps it will be used for another file
            free_space.start += file_to_move.length;
            free_space.length -= file_to_move.length;

            #[cfg(debug_assertions)]
            println!("Updated free space {:?}", free_space);

            return moved_file;
        }
    }

    // this could be optimized
    File {
        start: file_to_move.start,
        length: file_to_move.length,
        file_id: file_to_move.file_id,
    }
}

fn make_file_list(disc_layout: &[Type]) -> Vec<File> {
    let mut block_start: i32 = -1;
    let mut previous_value: Type = Type::Undefined;
    let mut blocks: Vec<File> = vec![];
    disc_layout.iter().enumerate().for_each(|(index, value)| {
        #[cfg(debug_assertions)]
        println!("{} {:?}", index, value);

        // dbg!(&block_start, &previous_value, &value);

        // no previous value
        if previous_value == Type::Undefined {
            // new Block
            // dbg!("new block");
            block_start = index as i32;
            previous_value = value.clone();
        } else if previous_value != *value {
            // Block ends here
            // dbg!("block ends here");

            let sector_id = match previous_value {
                Type::FileBlock(file_id) => file_id,
                Type::FreespaceBlock => FREE_SPACE_ID,
                _ => unreachable!(),
            };
            let block = File {
                start: block_start as usize,
                length: (index - block_start as usize),
                file_id: sector_id,
            };
            // dbg!(&block);

            blocks.push(block);

            block_start = index as i32;
            previous_value = value.clone();
        }
    });

    // ensure that we don't forget the last block
    if block_start != -1 {
        let sector_id = match previous_value {
            Type::FileBlock(file_id) => file_id,
            Type::FreespaceBlock => usize::MAX,
            _ => unreachable!(),
        };
        let block = File {
            start: block_start as usize,
            length: (disc_layout.len() - block_start as usize),
            file_id: sector_id,
        };
        blocks.push(block);
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_process_simple() -> miette::Result<()> {
        let input = "12345";
        let disc_layout: Vec<Type> = create_disc_layout(input);
        dbg!(&disc_layout);
        assert_eq!("141", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_2() -> miette::Result<()> {
        let input = "7042";

        assert_eq!("38", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_3() -> miette::Result<()> {
        let input = "75345";

        assert_eq!("138", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_4() -> miette::Result<()> {
        let input = "14145515";

        assert_eq!("138", process(input)?);
        Ok(())
    }
    
    // Test Block checksum
    #[test]
    fn test_block_checksum() {
        let file = File {
            start: 0,
            length: 3,
            file_id: 5,
        };
        // 0*5 + 1*5 + 2*5
        assert_eq!(file.checksum(), 15);

        let file2 = File {
            start: 2,
            length: 2,
            file_id: 9,
        };

        // 2*9 + 3*9
        assert_eq!(file2.checksum(), 45);

        let file2 = File {
            start: 10,
            length: 2,
            file_id: 10,
        };
        // 10*10 + 11*10
        assert_eq!(file2.checksum(), 210);
    }
}
