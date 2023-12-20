use anyhow::{anyhow, Context, Result};
use std::env::args;
use std::fs::read_to_string;

type Seed = u64;
type SeedRange = (u64, u64);
type Map = Vec<(u64, u64, i64)>;

fn parse(input: &str) -> Result<(Vec<Seed>, Vec<Map>)> {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let seeds_txt = sections
        .first()
        .ok_or(anyhow!("should have seeds section"))?;
    let seeds_txt = seeds_txt.trim_start_matches("seeds: ").trim();
    let seeds = seeds_txt
        .split_ascii_whitespace()
        .map(|n| n.parse())
        .collect::<Result<Vec<Seed>, _>>()?;
    let mut maps = Vec::new();
    for section in &sections[1..] {
        let section_txt = section.split_once(':').ok_or(anyhow!(""))?.1;
        let mut map = Vec::new();
        for map_txt in section_txt.trim().split('\n') {
            let numbers = map_txt
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>())
                .collect::<Result<Vec<u64>, _>>()?;
            let (dst, src, len) = (numbers[0], numbers[1], numbers[2]);
            map.push((src, src + len - 1, dst as i64 - src as i64));
        }
        maps.push(map.clone());
    }
    Ok((seeds, maps))
}

fn part_one(seeds: &Vec<Seed>, maps: &Vec<Map>) -> Result<u64> {
    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut n: u64 = *seed;
        for map in maps {
            for (start, end, delta) in map {
                if (*start..=*end).contains(&n) {
                    n = (n as i64 + *delta) as u64;
                    break;
                }
            }
        }
        locations.push(n);
    }
    if let Some(n) = locations.iter().min() {
        Ok(*n)
    } else {
        Err(anyhow!("Should have at least one element"))
    }
}

fn part_two(seeds: &Vec<Seed>, maps: &Vec<Map>) -> Result<u64> {
    let mut ranges: Vec<SeedRange> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        ranges.push((seeds[i], seeds[i] + seeds[i + 1] - 1));
    }
    for map in maps {
        let mut new_ranges: Vec<(u64, u64)> = Vec::new();
        while let Some(range) = ranges.pop() {
            let mut found = false;
            for submap in map {
                // range is fully included in a map
                if range.0 >= submap.0 && range.1 <= submap.1 {
                    new_ranges.push((
                        (range.0 as i64 + submap.2) as u64,
                        (range.1 as i64 + submap.2) as u64,
                    ));
                    found = true;
                // range is partially included in a map
                // r (20, 30) m (24, 27)
                } else if submap.0 > range.0 && submap.1 < range.1 {
                    ranges.push((range.0, submap.0 - 1));
                    new_ranges.push((
                        (submap.0 as i64 + submap.2) as u64,
                        (submap.1 as i64 + submap.2) as u64,
                    ));
                    ranges.push((submap.1 + 1, range.1));
                    found = true;
                // range overlaps with a map on the right
                // r (20, 30) m (25, 40)
                } else if range.0 < submap.0 && range.1 >= submap.0 {
                    ranges.push((range.0, (submap.0 as i64 - 1) as u64));
                    new_ranges.push((
                        (submap.0 as i64 + submap.2) as u64,
                        (range.1 as i64 + submap.2) as u64,
                    ));
                    found = true;
                // range overlaps with a map on the left
                // r (20, 30) m (10, 25)
                } else if range.1 > submap.1 && range.0 <= submap.1 {
                    ranges.push((submap.1 + 1, range.1));
                    new_ranges.push((
                        (range.0 as i64 + submap.2) as u64,
                        (submap.1 as i64 + submap.2) as u64,
                    ));
                    found = true;
                }
            }
            if !found {
                new_ranges.push((range.0, range.1));
            }
        }
        ranges = new_ranges;
    }
    let start_indexes: Vec<u64> = ranges.iter().map(|r| r.0).collect();
    if let Some(n) = start_indexes.iter().min() {
        Ok(*n)
    } else {
        Err(anyhow!("Should have at least one element"))
    }
}

fn main() -> Result<()> {
    let input_file = args()
        .nth(1)
        .ok_or_else(|| anyhow!("missing input file argument"))?;
    let input = read_to_string(&input_file)
        .with_context(|| format!("could not read file `{}`", input_file))?;
    let (seeds, maps) = parse(&input)?;
    println!("Part one: {}", part_one(&seeds, &maps)?);
    println!("Part two: {}", part_two(&seeds, &maps)?);
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
        let input = read_to_string("./input/day05_test.txt").unwrap();
        let (seeds, maps) = parse(&input).unwrap();
        let res = part_one(&seeds, &maps).unwrap();
        let exp = 35;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day05_test.txt").unwrap();
        let (seeds, maps) = parse(&input).unwrap();
        let res = part_two(&seeds, &maps).unwrap();
        let exp = 46;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
