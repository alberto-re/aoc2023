use anyhow::{anyhow, Context, Result};
use gcd::Gcd;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse(input: &str) -> Result<(Vec<char>, NodeMap)> {
    let (instr, nodes) = input.split_once("\n\n").ok_or(anyhow!(""))?;
    let instr = instr.chars().collect::<Vec<char>>();
    let re = Regex::new(r"(?m)^(?<src>\w\w\w) = \((?<left>\w\w\w), (?<right>\w\w\w)\)$").unwrap();
    let mut nmap: HashMap<&str, (&str, &str)> = HashMap::new();
    for (_, [src, left, right]) in re.captures_iter(nodes).map(|c| c.extract()) {
        nmap.insert(src, (left, right));
    }
    Ok((instr, nmap))
}

fn part_one(instr: &Vec<char>, nodes: &HashMap<&str, (&str, &str)>) -> usize {
    let mut i = 0;
    let mut n = 0;
    let chr_len = instr.len();
    let mut cur_node = "AAA";
    loop {
        if cur_node == "ZZZ" {
            break;
        }
        cur_node = if instr[i] == 'L' {
            nodes.get(cur_node).unwrap().0
        } else {
            nodes.get(cur_node).unwrap().1
        };
        n += 1;
        i += 1;
        i %= chr_len;
    }
    n
}

fn part_two(instr: &Vec<char>, nodes: &HashMap<&str, (&str, &str)>) -> usize {
    let mut i = 0;
    let mut n = 0;
    let chr_len = instr.len();
    let mut cur_nodes: Vec<&str> = nodes.keys().filter(|n| n.ends_with('A')).copied().collect();
    let mut node_period: HashMap<usize, usize> = HashMap::new();
    loop {
        for (idx, node) in cur_nodes.to_owned().iter().enumerate() {
            if node.ends_with('Z') && !node_period.contains_key(&idx) {
                node_period.insert(idx, n);
            }
            if instr[i] == 'L' {
                cur_nodes[idx] = nodes.get(node).unwrap().0;
            } else {
                cur_nodes[idx] = nodes.get(node).unwrap().1;
            };
        }
        if node_period.len() == cur_nodes.len() {
            break;
        }
        n += 1;
        i += 1;
        i %= chr_len;
    }
    let mut res = 1;
    for i in 0..node_period.len() {
        res = res * node_period[&i] / res.gcd(node_period[&i])
    }
    res
}

fn main() -> Result<()> {
    let input = read_to_string("./input/day08.txt").with_context(|| "could not read input file")?;
    let (instr, nodes) = parse(&input)?;
    println!("Part one: {}", part_one(&instr, &nodes));
    println!("Part two: {}", part_two(&instr, &nodes));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;
    use std::fs::read_to_string;

    #[test]
    fn test_part_one_a() {
        let input = read_to_string("./input/day08_test_a.txt").unwrap();
        let (instr, nodes) = parse(&input).unwrap();
        let res = part_one(&instr, &nodes);
        let exp = 2;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_one_b() {
        let input = read_to_string("./input/day08_test_b.txt").unwrap();
        let (instr, nodes) = parse(&input).unwrap();
        let res = part_one(&instr, &nodes);
        let exp = 6;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day08_test_c.txt").unwrap();
        let (instr, nodes) = parse(&input).unwrap();
        let res = part_two(&instr, &nodes);
        let exp = 6;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
