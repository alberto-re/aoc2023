use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fs::read_to_string;

type Hand = Vec<usize>;

fn card_value(label: char) -> Result<usize> {
    match label {
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'T' => Ok(10),
        'J' => Ok(11),
        'Q' => Ok(12),
        'K' => Ok(13),
        'A' => Ok(14),
        _ => Err(anyhow!("Invalid label {}", label)),
    }
}

fn parse(input: &str) -> Result<Vec<(Hand, usize)>> {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let mut res = Vec::new();
    for line in lines {
        let (cards, bid) = line
            .split_once(' ')
            .ok_or(anyhow!("line should have two elements"))?;
        let values = cards
            .chars()
            .map(|l| card_value(l))
            .collect::<Result<Hand, _>>()?;
        res.push((values, bid.parse::<usize>()?));
    }
    Ok(res)
}

fn hand_type(hand: Hand) -> Result<usize> {
    assert!(hand.len() == 5);
    let mut grouped: HashMap<usize, usize> = HashMap::new();
    for value in hand {
        grouped.entry(value).and_modify(|c| *c += 1).or_insert(1);
    }
    if grouped.len() == 1 {
        // Five of a kind
        Ok(5)
    } else if grouped.len() == 2 {
        if grouped.into_values().max().unwrap() == 4 {
            // Four of a kind
            Ok(4)
        } else {
            // Three of a kind
            Ok(3)
        }
    } else if grouped.len() == 3 {
        // Two pair
        Ok(2)
    } else if grouped.len() == 4 {
        // One pair
        Ok(1)
    } else {
        // High card
        Ok(0)
    }
}

fn part_one(hands: &Vec<(Hand, usize)>) -> Result<usize> {
    Ok(0)
}

// fn part_two(hands: &Vec<(Hand, usize)>) -> Result<usize> {
//     Ok(0)
// }

fn main() -> Result<()> {
    let input = read_to_string("./input/day07.txt")
        .with_context(|| format!("could not read input file"))?;
    let hands = parse(&input)?;
    println!("Part one: {}", part_one(&hands)?);
    // println!("Part two: {}", part_two(&hands)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::hand_type;
    use crate::parse;
    use crate::part_one;
    // use crate::part_two;
    use std::fs::read_to_string;

    #[test]
    fn test_hand_type_five_of_a_kind() {
        let res = hand_type(vec![8, 8, 8, 8, 8]).unwrap();
        let exp = 5;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_four_of_a_kind() {
        let res = hand_type(vec![7, 8, 8, 8, 8]).unwrap();
        let exp = 4;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_three_of_a_kind() {
        let res = hand_type(vec![7, 7, 8, 8, 8]).unwrap();
        let exp = 3;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("./input/day07_test.txt").unwrap();
        let hands = parse(&input).unwrap();
        let res = part_one(&hands).unwrap();
        let exp = 6440;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    // #[test]
    // fn test_part_two() {
    //     let input = read_to_string("./input/day07_test.txt").unwrap();
    //     let hands = parse(&input).unwrap();
    //     let res = part_two(&hands).unwrap();
    //     let exp = 71503;
    //     assert!(res == exp, "result = {}, expected = {}", res, exp);
    // }
}
