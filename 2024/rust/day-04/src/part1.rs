type Grid = Vec<Vec<char>>;

fn count_word_occurrences(grid_chars: &Grid, word: &str) -> usize {
    let nrows = grid_chars.len();
    let ncols = grid_chars[0].len();
    let word_len = word.len();
    let reversed_word = word.chars().rev().collect::<String>();

    let mut count = 0;

    // Horizontal search
    for row in grid_chars {
        let row_str: String = row.iter().collect();
        count += count_substring(&row_str, word);
        count += count_substring(&row_str, &reversed_word);
    }

    // Vertical search
    for col in 0..ncols {
        // transform each column into a string
        let column: String = (0..nrows).map(|row| grid_chars[row][col]).collect();
        count += count_substring(&column, word);
        count += count_substring(&column, &reversed_word);
    }

    // Diagonal search (top-left to bottom-right)
    for r in 0..nrows {
        for c in 0..ncols {
            if let Some(diag) = diagonal_tl_br(&grid_chars, r, c, word_len) {
                if diag == word {
                    count += 1;
                }
                if diag == reversed_word {
                    count += 1;
                }
            }
        }
    }

    // Diagonal search (top-right to bottom-left)
    for r in 0..nrows {
        for c in 0..ncols {
            if let Some(diag) = diagonal_tr_bl(&grid_chars, r, c, word_len) {
                if diag == word {
                    count += 1;
                }
                if diag == reversed_word {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_substring(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

fn diagonal_tl_br(grid: &[Vec<char>], start_row: usize, start_col: usize, word_len: usize) -> Option<String> {
    let mut diag = String::new();
    for i in 0..word_len {
        let r = start_row + i;
        let c = start_col + i;
        if r < grid.len() && c < grid[0].len() {
            diag.push(grid[r][c]);
        } else {
            return None;
        }
    }
    Some(diag)
}

fn diagonal_tr_bl(grid: &[Vec<char>], start_row: usize, start_col: usize, word_len: usize) -> Option<String> {
    let mut diag = String::new();
    for i in 0..word_len {
        let r = start_row + i;
        if start_col >= i {
            let c = start_col - i;
            if r < grid.len() {
                diag.push(grid[r][c]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    Some(diag)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word = "XMAS";
    let count = count_word_occurrences(&grid, word);
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
