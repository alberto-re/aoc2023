use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs::read_to_string;

type Galaxy = Vec<(usize, usize)>;
type Distances = HashMap<((usize, usize), (usize, usize)), usize>;

fn parse(input: &str) -> Result<(Galaxy, Vec<usize>, Vec<usize>)> {
    let mut galaxies = vec![];
    let mut rows = vec![];
    let mut cols = vec![];
    for (y, line) in input.lines().enumerate() {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                galaxies.push((x, y))
            }
            if y == 0
                && input
                    .lines()
                    .all(|l| l.chars().nth(x) == line.chars().next())
            {
                cols.push(x);
            }
        });
        if line.chars().all(|c| c == line.chars().next().unwrap()) {
            rows.push(y);
        }
    }
    Ok((galaxies, rows, cols))
}

fn manthattan(start: (usize, usize), end: (usize, usize)) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

fn manthattan_exp(
    start: (usize, usize),
    end: (usize, usize),
    rows: &Vec<usize>,
    cols: &Vec<usize>,
    coef: usize,
) -> usize {
    let mut dist = manthattan(start, end);
    for row in rows {
        if start.1 < end.1 && row > &start.1 && row < &end.1 {
            dist += coef - 1;
        }
        if start.1 > end.1 && row > &end.1 && row < &start.1 {
            dist += coef - 1;
        }
    }
    for col in cols {
        if start.0 < end.0 && col > &start.0 && col < &end.0 {
            dist += coef - 1;
        }
        if start.0 > end.0 && col > &end.0 && col < &start.0 {
            dist += coef - 1;
        }
    }
    dist
}

fn solve(
    galaxies: &[(usize, usize)],
    rows: &Vec<usize>,
    cols: &Vec<usize>,
    coef: usize,
) -> Result<usize> {
    let mut dmap: Distances = HashMap::new();
    galaxies.iter().for_each(|g1| {
        galaxies.iter().for_each(|g2| {
            if g1 != g2 && !dmap.contains_key(&(*g2, *g1)) {
                dmap.insert((*g1, *g2), manthattan_exp(*g1, *g2, rows, cols, coef));
            }
        })
    });
    Ok(dmap.values().sum::<usize>())
}

fn main() -> Result<()> {
    let input = read_to_string("./input/day11.txt").with_context(|| "could not read input file")?;
    let (galaxies, rows, cols) = parse(&input)?;
    println!("Part one: {}", solve(&galaxies, &rows, &cols, 2)?);
    println!("Part two: {}", solve(&galaxies, &rows, &cols, 1000000)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::solve;
    use std::fs::read_to_string;

    #[test]
    fn test_part_one() {
        let input = read_to_string("./input/day11_test.txt").unwrap();
        let (galaxies, rows, cols) = parse(&input).unwrap();
        let res = solve(&galaxies, &rows, &cols, 2).unwrap();
        let exp = 374;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day11_test.txt").unwrap();
        let (galaxies, rows, cols) = parse(&input).unwrap();
        let res = solve(&galaxies, &rows, &cols, 10).unwrap();
        let exp = 1030;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
        let res = solve(&galaxies, &rows, &cols, 100).unwrap();
        let exp = 8410;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
