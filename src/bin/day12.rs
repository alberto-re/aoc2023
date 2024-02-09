use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Record {
    condition: Vec<char>,
    groups: Vec<usize>,
}

type Cache = HashMap<Record, usize>;

impl Record {
    fn new(condition: Vec<char>, groups: Vec<usize>) -> Self {
        Self { condition, groups }
    }

    fn valid_arrangements(&self, cache: &mut Cache) -> usize {
        if cache.contains_key(self) {
            return *cache.get(self).unwrap();
        }

        if self.groups.is_empty() {
            if !self.condition.contains(&'#') {
                return 1;
            } else {
                return 0;
            }
        }

        if self.condition.is_empty() {
            return 0;
        }

        let next_char = self.condition[0];

        let retval = match next_char {
            '#' => self.pound(cache),
            '.' => self.dot(cache),
            '?' => self.pound(cache) + self.dot(cache),
            _ => unreachable!(),
        };

        cache.insert(self.clone(), retval);
        retval
    }

    fn pound(&self, cache: &mut Cache) -> usize {
        let next_group = self.groups[0];
        let this_group: Vec<char> =
            self.condition[..usize::min(next_group, self.condition.len())].to_vec();

        if this_group.len() != next_group || !this_group.iter().all(|&c| c != '.') {
            return 0;
        }

        if self.condition.len() == next_group {
            if self.groups.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        }

        if self.condition[next_group] != '#' {
            return Record::new(
                self.condition[next_group + 1..].to_vec(),
                self.groups[1..].to_vec(),
            )
            .valid_arrangements(cache);
        }

        0
    }

    fn dot(&self, cache: &mut Cache) -> usize {
        Record::new(self.condition[1..].to_vec(), self.groups.clone()).valid_arrangements(cache)
    }
}

fn parse(input: &str) -> Result<Vec<Record>> {
    let mut result = vec![];
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let condition = parts
            .first()
            .ok_or(anyhow!("Invalid input"))?
            .chars()
            .collect::<Vec<char>>();
        let groups = parts
            .last()
            .ok_or(anyhow!("Invalid input"))?
            .split(',')
            .map(|c| c.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        result.push(Record::new(condition, groups));
    }
    Ok(result)
}

fn part_one(records: &[Record]) -> Result<usize> {
    let mut cache: Cache = HashMap::new();
    Ok(records
        .iter()
        .map(|r| r.valid_arrangements(&mut cache))
        .sum())
}

fn part_two(records: &[Record]) -> Result<usize> {
    let records = records
        .iter()
        .map(|r| {
            let mut conditions: Vec<char> = vec![];
            let mut groups: Vec<usize> = vec![];
            (0..5).for_each(|i| {
                r.condition.iter().for_each(|c| conditions.push(*c));
                if i != 4 {
                    conditions.push('?');
                }
                r.groups.iter().for_each(|g| groups.push(*g));
            });
            Record::new(conditions, groups)
        })
        .collect::<Vec<Record>>();
    let mut cache: Cache = HashMap::new();
    Ok(records
        .iter()
        .map(|r| r.valid_arrangements(&mut cache))
        .sum())
}

fn main() -> Result<()> {
    let input = read_to_string("./input/day12.txt").with_context(|| "could not read input file")?;
    let records: Vec<Record> = parse(&input)?;
    println!("Part one: {}", part_one(&records)?);
    println!("Part two: {}", part_two(&records)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;
    use std::fs::read_to_string;

    #[test]
    fn test_part_one() {
        let input = read_to_string("./input/day12_test.txt").unwrap();
        let records = parse(&input).unwrap();
        let res = part_one(&records).unwrap();
        let exp = 21;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day12_test.txt").unwrap();
        let records = parse(&input).unwrap();
        let res = part_two(&records).unwrap();
        let exp = 525152;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
