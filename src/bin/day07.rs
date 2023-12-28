use anyhow::{anyhow, Context, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

type Hand = Vec<usize>;

fn card_value(label: char, jolly: bool) -> Result<usize> {
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
        'J' => {
            if jolly {
                Ok(1)
            } else {
                Ok(11)
            }
        }
        'Q' => Ok(12),
        'K' => Ok(13),
        'A' => Ok(14),
        _ => Err(anyhow!("Invalid label {}", label)),
    }
}

fn parse(input: &str, jolly: bool) -> Result<Vec<(Hand, usize)>> {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let mut res = Vec::new();
    for line in lines {
        let (cards, bid) = line
            .split_once(' ')
            .ok_or(anyhow!("line should have two elements"))?;
        let values = cards
            .chars()
            .map(|l| card_value(l, jolly))
            .collect::<Result<Hand, _>>()?;
        res.push((values, bid.parse::<usize>()?));
    }
    Ok(res)
}

fn hand_type(hand: &Hand) -> usize {
    assert!(hand.len() == 5);
    let mut grouped: HashMap<usize, usize> = HashMap::new();
    for value in hand {
        grouped.entry(*value).and_modify(|c| *c += 1).or_insert(1);
    }
    if grouped.len() == 1 {
        // Five of a kind
        6
    } else if grouped.len() == 2 {
        if grouped.into_values().max().unwrap() == 4 {
            // Four of a kind
            5
        } else {
            // Full house
            4
        }
    } else if grouped.len() == 3 {
        if grouped.into_values().max().unwrap() == 3 {
            // Three of a kind
            3
        } else {
            // Two pair
            2
        }
    } else if grouped.len() == 4 {
        // One pair
        1
    } else {
        // High card
        0
    }
}

fn best_hand_type(hand: &Hand) -> usize {
    if !hand.contains(&1) {
        return hand_type(hand);
    }
    let mut max_hand = 0;
    for i in 2..=14 {
        if i == 11 {
            continue;
        }
        let mut new_hand: Hand = Vec::new();
        for card in hand {
            if card == &1 {
                new_hand.push(i);
            } else {
                new_hand.push(*card);
            }
        }
        let new_hand_type = hand_type(&new_hand);
        if new_hand_type > max_hand {
            max_hand = new_hand_type;
        }
    }
    max_hand
}

fn compare_hands(left: &Hand, right: &Hand) -> isize {
    assert!(left.len() == right.len());
    for i in 0..left.len() {
        match left[i].cmp(&right[i]) {
            Ordering::Greater => return -1,
            Ordering::Less => return 1,
            Ordering::Equal => (),
        }
    }
    0
}

fn custom_cmp(left: &(Hand, usize), right: &(Hand, usize), jolly: bool) -> Ordering {
    let tleft = if jolly {
        best_hand_type(&left.0)
    } else {
        hand_type(&left.0)
    };
    let tright = if jolly {
        best_hand_type(&right.0)
    } else {
        hand_type(&right.0)
    };
    match tleft.cmp(&tright) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            let cmp = compare_hands(&left.0, &right.0);
            if cmp == -1 {
                Ordering::Greater
            } else if cmp == 1 {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }
}

fn part_one(hands: &[(Hand, usize)]) -> Result<usize> {
    let mut hands = hands.to_owned();
    hands.sort_by(|a, b| custom_cmp(a, b, false));
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.1 * (i + 1);
    }
    Ok(total)
}

fn part_two(hands: &[(Hand, usize)]) -> Result<usize> {
    let mut hands = hands.to_owned();
    hands.sort_by(|a, b| custom_cmp(a, b, true));
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.1 * (i + 1);
    }
    Ok(total)
}

fn main() -> Result<()> {
    let input = read_to_string("./input/day07.txt").with_context(|| "could not read input file")?;
    let hands = parse(&input, false)?;
    println!("Part one: {}", part_one(&hands)?);
    let hands = parse(&input, true)?;
    println!("Part two: {}", part_two(&hands)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::best_hand_type;
    use crate::compare_hands;
    use crate::hand_type;
    use crate::parse;
    use crate::part_one;
    use crate::part_two;
    use std::fs::read_to_string;

    #[test]
    fn test_hand_type_five_of_a_kind() {
        let res = hand_type(&vec![14, 14, 14, 14, 14]);
        let exp = 6;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_four_of_a_kind() {
        let res = hand_type(&vec![14, 14, 8, 14, 14]);
        let exp = 5;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_full_house() {
        let res = hand_type(&vec![2, 3, 3, 3, 2]);
        let exp = 4;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_three_of_a_kind() {
        let res = hand_type(&vec![10, 10, 10, 9, 8]);
        let exp = 3;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_two_pair() {
        let res = hand_type(&vec![2, 3, 4, 3, 2]);
        let exp = 2;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_one_pair() {
        let res = hand_type(&vec![14, 2, 3, 14, 4]);
        let exp = 1;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_hand_type_high_card() {
        let res = hand_type(&vec![2, 3, 4, 5, 6]);
        let exp = 0;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_best_hand_type_four_of_a_kind() {
        let res = best_hand_type(&vec![10, 5, 5, 1, 5]);
        let exp = 5;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
        let res = best_hand_type(&vec![13, 10, 1, 1, 10]);
        let exp = 5;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
        let res = best_hand_type(&vec![12, 12, 12, 1, 14]);
        let exp = 5;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_best_hand_type_two_pair() {
        let res = best_hand_type(&vec![13, 13, 6, 7, 7]);
        let exp = 2;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_best_hand_type_one_pair() {
        let res = best_hand_type(&vec![3, 2, 10, 3, 13]);
        let exp = 1;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_compare_hands() {
        let res = compare_hands(&vec![3, 3, 3, 3, 2], &vec![2, 14, 14, 14, 14]);
        let exp = -1;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
        let res = compare_hands(&vec![7, 7, 8, 8, 8], &vec![7, 7, 7, 8, 8]);
        let exp = -1;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_one() {
        let input = read_to_string("./input/day07_test.txt").unwrap();
        let hands = parse(&input, false).unwrap();
        let res = part_one(&hands).unwrap();
        let exp = 6440;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day07_test.txt").unwrap();
        let hands = parse(&input, true).unwrap();
        let res = part_two(&hands).unwrap();
        let exp = 5905;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
