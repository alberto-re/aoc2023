use anyhow::{anyhow, Context, Result};
use geo::algorithm::Contains;
use geo::geometry::LineString;
use geo::geometry::Polygon;
use geo::point;
use std::fs::read_to_string;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }
}

type Diagram = Vec<Vec<char>>;

fn parse(input: &str) -> Result<(Diagram, Pos)> {
    let mut diagram = Vec::new();
    let mut start = Pos::new(0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            row.push(chr);
            if chr == 'S' {
                start = Pos::new(x, y);
            }
        }
        diagram.push(row);
    }
    Ok((diagram, start))
}

fn start_shape(diagram: &Diagram, start: Pos) -> Result<char> {
    // N, E, S, W
    let mut connections = [0u8, 0u8, 0u8, 0u8];
    if start.y > 0 && ['|', '7', 'F'].contains(&diagram[start.y - 1][start.x]) {
        connections[0] = 1;
    };
    if ['-', '7', 'J'].contains(&diagram[start.y][start.x + 1]) {
        connections[1] = 1;
    };
    if ['|', 'L', 'J'].contains(&diagram[start.y + 1][start.x]) {
        connections[2] = 1;
    };
    if start.x > 0 && ['-', 'F', 'L'].contains(&diagram[start.y][start.x - 1]) {
        connections[3] = 1;
    };
    match connections {
        [1, 1, 0, 0] => Ok('L'),
        [1, 0, 1, 0] => Ok('|'),
        [1, 0, 0, 1] => Ok('J'),
        [0, 1, 1, 0] => Ok('F'),
        [0, 1, 0, 1] => Ok('-'),
        [0, 0, 1, 1] => Ok('7'),
        _ => Err(anyhow!("any pipe should connect exactly two directions")),
    }
}

fn path(diagram: &Diagram, start: Pos) -> Result<Vec<(char, Pos)>> {
    let mut curpos: (char, Pos) = (start_shape(diagram, start)?, start);
    let mut lastpos = curpos;
    let mut path: Vec<(char, Pos)> = Vec::new();
    path.push(curpos);
    loop {
        let directions: ((isize, isize), (isize, isize)) = match curpos {
            ('F', _) => ((0, 1), (1, 0)),
            ('-', _) => ((-1, 0), (1, 0)),
            ('7', _) => ((-1, 0), (0, 1)),
            ('|', _) => ((0, -1), (0, 1)),
            ('J', _) => ((0, -1), (-1, 0)),
            ('L', _) => ((0, -1), (1, 0)),
            _ => unreachable!(),
        };
        let gonext = if path.len() == 1 {
            directions.0
        } else {
            let (x, y) = directions.0;
            if (curpos.1.x as isize + x) as usize == lastpos.1.x
                && (curpos.1.y as isize + y) as usize == lastpos.1.y
            {
                directions.1
            } else {
                directions.0
            }
        };
        lastpos = curpos;
        let coords = Pos::new(
            (curpos.1.x as isize + gonext.0) as usize,
            (curpos.1.y as isize + gonext.1) as usize,
        );
        curpos = (diagram[coords.y][coords.x], coords);
        if curpos.0 == 'S' {
            break;
        }
        path.push(curpos);
    }
    Ok(path)
}

fn part_one(diagram: &Diagram, start: Pos) -> Result<usize> {
    let path = path(diagram, start)?;
    Ok(path.len() / 2)
}

fn is_tile_enclosed(x: usize, y: usize, path: &[(char, Pos)]) -> bool {
    let p = point!(x: x as f32, y: y as f32);
    let poly: Polygon<_> = Polygon::new(
        LineString::from(
            path.iter()
                .map(|p| (p.1.x as f32, p.1.y as f32))
                .collect::<Vec<(_, _)>>(),
        ),
        vec![],
    );
    poly.contains(&p)
}

fn part_two(diagram: &Diagram, start: Pos) -> Result<usize> {
    let path = path(diagram, start)?;
    let mut count = 0;
    for (x, row) in diagram.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if is_tile_enclosed(x, y, &path) {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn main() -> Result<()> {
    let input = read_to_string("./input/day10.txt").with_context(|| "could not read input file")?;
    let (diagram, start) = parse(&input)?;
    println!("Part one: {}", part_one(&diagram, start)?);
    println!("Part two: {}", part_two(&diagram, start)?);
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
        let input = read_to_string("./input/day10_test_a.txt").unwrap();
        let (diagram, start) = parse(&input).unwrap();
        let res = part_one(&diagram, start).unwrap();
        let exp = 4;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_one_b() {
        let input = read_to_string("./input/day10_test_b.txt").unwrap();
        let (diagram, start) = parse(&input).unwrap();
        let res = part_one(&diagram, start).unwrap();
        let exp = 8;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("./input/day10_test_c.txt").unwrap();
        let (diagram, start) = parse(&input).unwrap();
        let res = part_two(&diagram, start).unwrap();
        let exp = 4;
        assert!(res == exp, "result = {}, expected = {}", res, exp);
    }
}
