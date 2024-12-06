type Grid = Vec<Vec<char>>;

fn find_multiple_patterns(grid_chars: &Grid, patterns: &[Vec<Vec<char>>]) -> Vec<usize> {
    let nrows = grid_chars.len();
    let ncols = grid_chars[0].len();

    let mut pattern_count = vec![0; patterns.len()];

    // iterate over each pattern and find it in the grid
    for (pattern_index, pattern) in patterns.iter().enumerate() {
        let prow = pattern.len();
        let pcol = pattern[0].len();

        // Sliding window for each pattern
        for r in 0..=nrows - prow {
            for c in 0..=ncols - pcol {
                if matches_pattern(grid_chars, pattern, r, c) {
                    pattern_count[pattern_index] += 1;
                }
            }
        }
    }

    pattern_count
}

fn matches_pattern(grid: &[Vec<char>], pattern: &[Vec<char>], start_row: usize, start_col: usize) -> bool {
    for r in 0..pattern.len() {
        for c in 0..pattern[r].len() {
            if grid[start_row + r][start_col + c] != pattern[r][c] {
                return false;
            }
        }
    }
    true
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let patterns = vec![
        vec![
            vec!['M', ' ', 'S'],
            vec![' ', 'A', ' '],
            vec!['M', ' ', 'S'],
        ],
        vec![
            vec!['M', ' ', 'M'],
            vec![' ', 'A', ' '],
            vec!['S', ' ', 'S'],
        ],
        vec![
            vec!['S', ' ', 'M'],
            vec![' ', 'A', ' '],
            vec!['S', ' ', 'M'],
        ],
        vec![
            vec!['S', ' ', 'S'],
            vec![' ', 'A', ' '],
            vec!['M', ' ', 'M'],
        ],
    ];

    let counts = find_multiple_patterns(&grid, &patterns);
    let count = counts.iter().sum::<usize>();
    
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
