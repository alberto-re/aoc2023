use std::{collections::HashMap, fs::read_to_string};

struct Card {
    id: u32,
    win: Vec<u32>,
    have: Vec<u32>,
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    for line in input.trim().split('\n') {
        let (card, numbers) = line.split_once(':').unwrap();
        let (_, id) = card.split_once(' ').unwrap();
        let id: u32 = id.trim().parse().unwrap();
        let (before, after) = numbers.split_once('|').unwrap();
        let win: Vec<u32> = before
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let have: Vec<u32> = after
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let card = Card { id, win, have };
        cards.push(card);
    }
    cards
}

fn part_one(cards: &Vec<Card>) -> u32 {
    let mut points = 0;
    for card in cards {
        let mut cpoint = 0;
        for n in &card.win {
            if card.have.contains(n) {
                if cpoint == 0 {
                    cpoint = 1
                } else {
                    cpoint *= 2;
                }
            }
        }
        points += cpoint;
    }
    points
}

fn part_two(cards: &Vec<Card>) -> u32 {
    let mut n = 0;
    let mut id_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut scards: Vec<u32> = Vec::new();
    for card in cards {
        let mut wins: u32 = 0;
        for w in &card.win {
            if card.have.contains(w) {
                wins += 1;
            }
        }
        let mut won: Vec<u32> = Vec::new();
        for i in card.id + 1..=card.id + wins {
            won.push(i);
        }
        id_map.insert(card.id, won.clone());
        scards.push(card.id);
    }
    while let Some(cn) = scards.pop() {
        n += 1;
        let won = id_map.get(&cn).unwrap();
        for i in won.iter() {
            scards.push(*i);
        }
    }
    n
}

fn main() {
    let input = read_to_string("./input/day04.txt").expect("Cannot read input file");
    let cards = parse(&input);
    println!("Part one: {}", part_one(&cards));
    println!("Part two: {}", part_two(&cards));
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn test_part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = parse(&input);
        let res = part_one(&cards);
        let exp = 13;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let cards = parse(&input);
        let res = part_two(&cards);
        let exp = 30;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
