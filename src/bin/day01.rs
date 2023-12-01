use std::fs::read_to_string;

const DIGIT_MAP: &[(&str, u32)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

const LITERAL_MAP: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn calibration_sum(input: &str, part_two: bool) -> u32 {
    let mut sum: u32 = 0;
    for line in input.trim().split('\n') {
        let mut numbers: Vec<u32> = Vec::new();
        for i in 0..line.len() {
            if part_two {
                for (literal, int) in LITERAL_MAP {
                    if line[i..].starts_with(literal) {
                        numbers.push(*int);
                    }
                }
            }
            for (chr, int) in DIGIT_MAP {
                if line[i..].starts_with(chr) {
                    numbers.push(*int);
                }
            }
        }
        sum += numbers.first().unwrap() * 10;
        sum += numbers.last().unwrap();
    }
    sum
}

fn main() {
    let input = read_to_string("./input/day01.txt").expect("Cannot read input file");
    println!("Part one: {}", calibration_sum(&input, false));
    println!("Part two: {}", calibration_sum(&input, true));
}

#[cfg(test)]
mod tests {
    use crate::calibration_sum;

    #[test]
    fn test_part_one() {
        let input = "1abc2\n\
                     pqr3stu8vwx\n\
                     a1b2c3d4e5f\n\
                     treb7uchet";
        let res = calibration_sum(&input, false);
        let exp: u32 = 142;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = "two1nine\n\
                     eightwothree\n\
                     abcone2threexyz\n\
                     xtwone3four\n\
                     4nineeightseven2\n\
                     zoneight234\n\
                     7pqrstsixteen";
        let res = calibration_sum(&input, true);
        let exp: u32 = 281;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
