use anyhow::{anyhow, Context, Result};
use std::env::args;
use std::fs::read_to_string;

type Race = (usize, usize);

fn parse(input: &str) -> Result<Vec<Race>> {
    let lines = input
        .split_once('\n')
        .ok_or(anyhow!("input should have two lines"))?;
    let times = lines
        .0
        .trim_start_matches("Time: ")
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    let distances = lines
        .1
        .trim_start_matches("Distance: ")
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    assert!(times.len() == distances.len());
    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push((times[i], distances[i]));
    }
    Ok(races)
}

fn race_options(time: usize) -> Vec<(usize, usize)> {
    let mut opts = Vec::new();
    for i in 0..=time {
        opts.push((i, i * (time - i)));
    }
    opts
}

fn part_one(races: &Vec<Race>) -> Result<usize> {
    let mut n = 1;
    for race in races {
        let opts = race_options(race.0);
        let wins = opts.iter().filter(|&r| r.1 > race.1).collect::<Vec<_>>();
        if !wins.is_empty() {
            n *= wins.len();
        }
    }
    Ok(n)
}

fn part_two(races: &Vec<Race>) -> Result<usize> {
    let mut time = 0;
    let mut distance = 0;
    for race in races {
        time = time * 10usize.pow(race.0.to_string().len() as u32) + race.0;
        distance = distance * 10usize.pow(race.1.to_string().len() as u32) + race.1;
    }
    let opts = race_options(time);
    let wins = opts.iter().filter(|&r| r.1 > distance).collect::<Vec<_>>();
    Ok(wins.len())
}

fn main() -> Result<()> {
    let input_file = args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input file argument"))?;
    let input = read_to_string(&input_file)
        .with_context(|| format!("could not read file `{}`", input_file))?;
    let races = parse(&input)?;
    println!("Part one: {}", part_one(&races)?);
    println!("Part two: {}", part_two(&races)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;
    use crate::race_options;
    use std::fs::read_to_string;

    #[test]
    fn test_race_options() {
        let res = race_options(7);
        let exp = vec![
            (0, 0),
            (1, 6),
            (2, 10),
            (3, 12),
            (4, 12),
            (5, 10),
            (6, 6),
            (7, 0),
        ];
        assert!(res == exp, "result = {:?}, expected = {:?}", res, exp);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("./input/day06_test.txt").unwrap();
        let races = parse(&input).unwrap();
        let res = part_one(&races).unwrap();
        let exp = 288;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day06_test.txt").unwrap();
        let races = parse(&input).unwrap();
        let res = part_two(&races).unwrap();
        let exp = 71503;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
