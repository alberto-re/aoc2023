use regex::Regex;
use std::fs::read_to_string;

struct Element {
    x: u64,
    y: u64,
    value: String,
}

fn parse(input: &str) -> (Vec<Element>, Vec<Element>) {
    let mut symbols: Vec<Element> = Vec::new();
    let mut numbers: Vec<Element> = Vec::new();
    let re_symbol = Regex::new(r"[^\d\.]").unwrap();
    let re_number = Regex::new(r"\d+").unwrap();
    for (y, line) in input.trim().split('\n').enumerate() {
        for caps in re_symbol.find_iter(line) {
            symbols.push(Element {
                x: caps.range().next().unwrap() as u64,
                y: y as u64,
                value: caps.as_str().to_string(),
            });
        }
        for caps in re_number.find_iter(line) {
            numbers.push(Element {
                x: caps.range().next().unwrap() as u64,
                y: y as u64,
                value: caps.as_str().to_string(),
            });
        }
    }
    (symbols, numbers)
}

fn is_adjacent(first: &Element, second: &Element) -> bool {
    first.y <= second.y + 1
        && first.y >= second.y.saturating_sub(1)
        && first.x >= second.x.saturating_sub(1)
        && first.x <= second.x + second.value.len() as u64
}

fn part_one(symbols: &Vec<Element>, numbers: &Vec<Element>) -> u64 {
    let mut sum: u64 = 0;
    for number in numbers {
        let mut adjacent = false;
        for symbol in symbols {
            if is_adjacent(symbol, number) {
                adjacent = true;
                break;
            }
        }
        if adjacent {
            sum += number.value.parse::<u64>().unwrap();
        }
    }
    sum
}

fn part_two(symbols: &Vec<Element>, numbers: &Vec<Element>) -> u64 {
    let mut sum: u64 = 0;
    for symbol in symbols {
        if symbol.value != "*" {
            continue;
        }
        let mut adjacent: Vec<u64> = Vec::new();
        for number in numbers {
            if is_adjacent(symbol, number) {
                adjacent.push(number.value.parse::<u64>().unwrap());
            }
        }
        if adjacent.len() == 2 {
            sum += adjacent[0] * adjacent[1];
        }
    }
    sum
}

fn main() {
    let input = read_to_string("./input/day03.txt").expect("Cannot read input file");
    let (symbols, numbers) = parse(&input);
    println!("Part one: {}", part_one(&symbols, &numbers));
    println!("Part two: {}", part_two(&symbols, &numbers));
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn test_part_one() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";
        let (symbols, numbers) = parse(&input);
        let res = part_one(&symbols, &numbers);
        let exp: u64 = 4361;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";
        let (symbols, numbers) = parse(&input);
        let res = part_two(&symbols, &numbers);
        let exp: u64 = 467835;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
