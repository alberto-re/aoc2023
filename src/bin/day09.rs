use anyhow::{Context, Result};
use std::fs::read_to_string;

type Hist = Vec<isize>;

fn parse(input: &str) -> Result<Vec<Hist>> {
    let mut res: Vec<Hist> = Vec::new();
    for line in input.lines() {
        let hist: Hist = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<isize>())
            .collect::<Result<Vec<isize>, _>>()?;
        res.push(hist);
    }
    Ok(res)
}

fn solve(histories: &[Hist], part_one: bool) -> isize {
    let mut n = 0;
    for hist in histories.iter() {
        let mut seqs: Vec<Box<Hist>> = Vec::new();
        seqs.push(Box::new(hist.to_owned()));
        while !seqs.last().unwrap().iter().all(|n| *n == 0) {
            let mut nseq: Hist = Vec::new();
            for i in 0..seqs.last().unwrap().len() - 1 {
                nseq.push(seqs.last().unwrap()[i + 1] - seqs.last().unwrap()[i]);
            }
            seqs.push(Box::new(nseq));
        }
        let mut lastval: Vec<isize> = Vec::new();
        for (i, seq) in seqs.iter().rev().enumerate() {
            if i == 0 {
                lastval.push(0);
            } else if part_one {
                lastval.push(seq.last().unwrap() + lastval.last().unwrap());
            } else {
                lastval.push(seq.first().unwrap() - lastval.last().unwrap());
            }
        }
        n += lastval.last().unwrap();
    }
    n
}

fn main() -> Result<()> {
    let input = read_to_string("./input/day09.txt").with_context(|| "could not read input file")?;
    let histories = parse(&input)?;
    println!("Part one: {}", solve(&histories, true));
    println!("Part two: {}", solve(&histories, false));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::solve;
    use std::fs::read_to_string;

    #[test]
    fn test_part_one() {
        let input = read_to_string("./input/day09_test.txt").unwrap();
        let histories = parse(&input).unwrap();
        let res = solve(&histories, true);
        let exp = 114;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day09_test.txt").unwrap();
        let histories = parse(&input).unwrap();
        let res = solve(&histories, false);
        let exp = 2;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
