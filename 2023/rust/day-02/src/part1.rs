use crate::custom_error::AocError;
use std::cmp;

#[derive(Debug)]
struct Cube {
    color: String,
    amount: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Vec<Cube>>,
}

fn parse_gameinformation(line: &str) -> Game {
    let mut cubes: Vec<Vec<Cube>> = Vec::new();

    let game_info: Vec<&str> = line.split(':').collect();
    let number = game_info[0].split(' ').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

    game_info[1].split(';').for_each(|cube| {
        let mut cube_splitted: Vec<&str> = cube.split(',').collect();
        let mut cubes_in_game: Vec<Cube> = Vec::new();

        for c in cube_splitted.iter_mut() {
            *c = c.trim();
            let cube_and_amount:Vec<&str> = c.split_ascii_whitespace().collect();
            cubes_in_game.push(Cube {
                color: cube_and_amount[1].to_string(),
                amount: cube_and_amount[0].parse::<u32>().unwrap(),
            });
        }

        cubes.push(cubes_in_game);
    });

    Game { id: number, cubes }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut games: Vec<Game> = Vec::new();

    input.lines().for_each(|line| {
        games.push(parse_gameinformation(line));
    });

    let mut sum = 0;
    for game in games.iter() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        game.cubes.iter().for_each(|cubes| {
            cubes.iter().for_each(|cube| {
                match cube.color.as_str() {
                    "red" => red = cmp::max(cube.amount, red),
                    "green" => green = cmp::max(cube.amount, green),
                    "blue" => blue = cmp::max(cube.amount, blue),
                    _ => (),
                }
            });
        });

        // dbg!(red, green, blue);
        if !(red > 12 || green > 13 || blue > 14) {
            sum += game.id;
        }// else {
        //     println!("Game {} isn't possible", game.id);
        // }
    }

    return Ok(sum.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_PART1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_process() -> miette::Result<()> {
        assert_eq!("8", process(TEST_DATA_PART1)?);
        Ok(())
    }
}
