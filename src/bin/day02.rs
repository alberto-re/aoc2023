use std::fs::read_to_string;

struct Game {
    id: u8,
    r: Vec<u32>,
    g: Vec<u32>,
    b: Vec<u32>,
}

impl Game {
    fn new(id: u8) -> Self {
        Self {
            id,
            r: Vec::new(),
            g: Vec::new(),
            b: Vec::new(),
        }
    }
}

fn parse(input: &str) -> Vec<Game> {
    let mut res: Vec<Game> = Vec::new();
    for line in input.trim().split('\n') {
        let (id, sets) = line
            .trim_start_matches("Game ")
            .split_once(": ")
            .expect("Malformed input");
        let id: u8 = id.parse().unwrap();
        let mut game = Game::new(id);
        for set in sets.split("; ") {
            for color in set.split(", ") {
                let (n, col) = color.split_once(' ').unwrap();
                let n: u32 = n.parse().unwrap();
                match col {
                    "red" => {
                        game.r.push(n);
                    }
                    "green" => {
                        game.g.push(n);
                    }
                    "blue" => {
                        game.b.push(n);
                    }
                    _ => {
                        panic!("Unexpected color")
                    }
                }
            }
        }
        res.push(game);
    }
    res
}

fn part_one(games: &Vec<Game>) -> u32 {
    let mut sum: u32 = 0;
    for game in games {
        if !game.r.iter().any(|x| x > &12)
            && !game.g.iter().any(|x| x > &13)
            && !game.b.iter().any(|x| x > &14)
        {
            sum += game.id as u32;
        }
    }
    sum
}

fn part_two(games: &Vec<Game>) -> u32 {
    let mut sum: u32 = 0;
    for game in games {
        let r = *game.r.iter().max().unwrap_or(&0);
        let g = *game.g.iter().max().unwrap_or(&0);
        let b = *game.b.iter().max().unwrap_or(&0);
        sum += r * g * b;
    }
    sum
}

fn main() {
    let input = read_to_string("./input/day02.txt").expect("Cannot read input file");
    let games = parse(&input);
    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn test_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse(&input);
        let res = part_one(&games);
        let exp: u32 = 8;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse(&input);
        let res = part_two(&games);
        let exp: u32 = 2286;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
