use crate::common::{create_disc_layout, BlockType};

const FREE_SPACE_ID: usize = 12121;

#[derive(Debug, Clone)]
struct File {
    start: usize,
    length: usize,
    file_id: usize,
}

impl File {
    fn checksum(&self) -> usize {
        (self.start..self.start + self.length)
            .map(|index| self.file_id * index)
            .sum()
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let disc_layout: Vec<BlockType> = create_disc_layout(input);

    // dbg!(&disc_layout);

    let files = build_file_list(&disc_layout);

    // dbg!(&files);

    // reverse iterate through disc_layout and move every file to the next free space at the beginning of disc_layout based on the zeroth element of empty_space_indices
    let mut free_space: Vec<File> = files
        .iter()
        .filter(|f| f.file_id == FREE_SPACE_ID)
        .cloned()
        .collect();
    
    let files_to_move: Vec<File> = files
        .iter()
        .filter(|f| f.file_id != FREE_SPACE_ID)
        .cloned()
        .collect();

    // dbg!(&free_space);
    // dbg!(&files_to_move);

    // Move all blocks from the files_to_move list to the first free space in free_space
    // if there is no free space for this file, then calculate the checksum for that file based on
    // the current position, otherwise "move" the file the new location, calculate the checksum and
    // update the free space entry to reflect that this space is now smaller or entirely gone

    let checksum: usize = files_to_move
        .iter()
        .rev()
        .map(|file_to_move| {
            #[cfg(debug_assertions)]
            println!("Try to move file {:?}", file_to_move);

            let moved_file = move_file_to_free_space(&mut free_space, file_to_move);

            // #[cfg(debug_assertions)]
            // if moved_file.start != file_to_move.start {
            //     println!("Successfully moved {:?} to {:?}", file_to_move, moved_file);
            // } else {
            //     println!("Failed to move {:?}", file_to_move);
            // }

            moved_file.checksum()
        })
        .sum();

    Ok(checksum.to_string())
}

fn move_file_to_free_space(free_space: &mut [File], file_to_move: &File) -> File {
    let possible_free_space_indices: Vec<u32> = free_space
        .iter()
        .enumerate()
        .filter(|(_, free_space_block)| {
            free_space_block.length >= file_to_move.length
                && free_space_block.start < file_to_move.start
        })
        .map(|(i, _)| i as u32)
        .collect();

    // println!("{:?}", &possible_free_space_indices);
    // no space to move file
    match possible_free_space_indices.is_empty() {
        true => file_to_move.clone(),
        false => {
            // update the remaining free space
            let index = possible_free_space_indices[0] as usize;
            let free_space = &mut free_space[index];
            // #[cfg(debug_assertions)]
            // println!("Found free space {:?} for {:?}", free_space, file_to_move);
            let moved_file = File {
                start: free_space.start,
                length: file_to_move.length,
                file_id: file_to_move.file_id,
            };

            // update the empty space, it might be used for another file
            free_space.start += file_to_move.length;
            free_space.length -= file_to_move.length;

            // #[cfg(debug_assertions)]
            // println!("Updated free space {:?}", free_space);
            // dbg!(&free_space);

            moved_file
        }
    }
}

fn build_file_list(disc_layout: &[BlockType]) -> Vec<File> {
    let mut block_start: i32 = -1;
    let mut previous_block: BlockType = BlockType::Undefined;
    let mut blocks: Vec<File> = vec![];
    disc_layout.iter().enumerate().for_each(|(index, block)| {
        // #[cfg(debug_assertions)]
        // println!("{} {:?}", index, block);

        // dbg!(&block_start, &previous_value, &value);

        // no previous value
        if previous_block == BlockType::Undefined {
            // new Block
            // dbg!("new block");
            block_start = index as i32;
            previous_block = block.clone();
        } else if previous_block != *block {
            // Block ends here
            // dbg!("block ends here");

            let sector_id = match previous_block {
                BlockType::FileBlock(file_id) => file_id,
                BlockType::FreespaceBlock => FREE_SPACE_ID,
                _ => unreachable!(),
            };

            let new_block = File {
                start: block_start as usize,
                length: index - block_start as usize,
                file_id: sector_id,
            };
            // dbg!(&block);

            blocks.push(new_block);

            block_start = index as i32;
            previous_block = block.clone();
        }
    });

    // ensure that we don't forget the last block
    if block_start != -1 {
        let sector_id = match previous_block {
            BlockType::FileBlock(file_id) => file_id,
            BlockType::FreespaceBlock => FREE_SPACE_ID,
            _ => unreachable!(),
        };
        let block = File {
            start: block_start as usize,
            length: disc_layout.len() - block_start as usize,
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
    // #[ignore]
    fn test_process_simple() -> miette::Result<()> {
        let input = "12345";
        // |-|00|111|0000|33333|
        // 3*1+4*1+5*1+10*2+11*2+12*2+13*2+14*2
        assert_eq!("132", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_2() -> miette::Result<()> {
        let input = "7042";
        assert_eq!("34", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_3() -> miette::Result<()> {
        let input = "75345";

        assert_eq!("129", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_4() -> miette::Result<()> {
        let input = "14145515";

        assert_eq!("125", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_5() -> miette::Result<()> {
        let input = "9953877292941";
        assert_eq!("5768", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_6() -> miette::Result<()> {
        let input = "1313165";
        assert_eq!("169", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_7() -> miette::Result<()> {
        let input = "1313135333";
        assert_eq!("248", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_process_simple_8() -> miette::Result<()> {
        let input = "131213533321";
        assert_eq!("328", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    // https://www.reddit.com/r/adventofcode/comments/1ha2mhp/2024_day_9_i_am_so_confused_about_the_id_rule_for/
    fn test_process_simple_9() -> miette::Result<()> {
        let input = "233313312141413140211";
        assert_eq!("2910", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    // https://www.reddit.com/r/adventofcode/comments/1hqc3sk/2024_day_9_part_2_works_for_examples_but_not_for/
    fn test_process_simple_10() -> miette::Result<()> {
        let input = "15121111211111612";
        assert_eq!("1106", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    // https://www.reddit.com/r/adventofcode/comments/1hqc3sk/2024_day_9_part_2_works_for_examples_but_not_for/
    fn test_process_simple_11() -> miette::Result<()> {
        let input = "233313312141413140256";
        assert_eq!("5828", process(input)?);
        Ok(())
    }

    #[test]
    // #[ignore]
    // https://www.reddit.com/r/adventofcode/comments/1ha2mhp/2024_day_9_i_am_so_confused_about_the_id_rule_for/
    fn test_process_simple_12() -> miette::Result<()> {
        let input = "10211";
        assert_eq!("9", process(input)?);
        Ok(())
    }

    // #[test_case(6304576012713)]
    // fn real_input(expected: usize) {
    //     assert_eq!(part_2(_INPUT), expected.into());
    // }

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

        let file3 = File {
            start: 7,
            length: 5,
            file_id: 2,
        };

        assert_eq!(file3.checksum(), 90);

        let file4 = File {
            start: 12,
            length: 3,
            file_id: 1,
        };

        assert_eq!(file4.checksum(), 39);

        let file5 = File {
            start: 0,
            length: 7,
            file_id: 0,
        };

        assert_eq!(file5.checksum(), 0);

        // 7042
        let file6 = File {
            start: 7,
            length: 4,
            file_id: 1,
        };

        assert_eq!(file6.checksum(), 34);
    }
}
