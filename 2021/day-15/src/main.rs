use std::fmt;

use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

struct Puzzle {
    map: Vec<Vec<usize>>,
}

impl Puzzle {
    fn parse_part1(input: &str) -> Puzzle {
        // println!("Parsing <{}>", input);
        let mut map: Vec<Vec<usize>> = Vec::new();

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            let trimmed = line_str.trim();
            if trimmed.len() == 0 {
                continue;
            }

            let mut row: Vec<usize> = Vec::new();
            for (_j, c) in trimmed.chars().enumerate() {
                let risk = c.to_digit(10).unwrap();
                row.push(risk.try_into().unwrap());
            }
            map.push(row);
        }

        Puzzle { map }
    }

    fn parse_part2(input: &str) -> Puzzle {
        let puzzle_template = Self::parse_part1(input);

        let template_height = puzzle_template.map.len();
        let template_width = puzzle_template.map[0].len();

        let mut map: Vec<Vec<usize>> = Vec::with_capacity(template_height * 5);
        map.resize(template_height * 5, vec![0; template_width * 5]);

        for section_row in 0..5 {
            for section_col in 0..5 {
                for i in 0..template_height {
                    for j in 0..template_width {
                        let mut value = puzzle_template.map[i][j] + section_row + section_col;
                        if value > 9 {
                            value = value % 9;
                        }
                        map[section_row * template_height + i][section_col * template_width + j] =
                            value;
                    }
                }
            }
        }

        Puzzle { map }
    }

    fn lowest_risk(&mut self) -> usize {
        let dest = Pos(self.map[0].len() - 1, self.map.len() - 1);
        let result = dijkstra(&Pos(0, 0), |p| self.neighbours(p), |p| *p == dest);

        match result {
            Some(path) => path.1,
            None => {
                panic!("No path found");
            }
        }
    }

    /// Given a position on the map, return the neighbouring cells and the required "weight" to get there.
    fn neighbours(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = pos;
        let mut cells: Vec<Pos> = Vec::new();

        if x > 0 {
            cells.push(Pos(x - 1, y));
        }
        if x < self.map[0].len() - 1 {
            cells.push(Pos(x + 1, y));
        }

        if y > 0 {
            cells.push(Pos(x, y - 1));
        }
        if y < self.map.len() - 1 {
            cells.push(Pos(x, y + 1));
        }

        cells.into_iter().map(|p| (p, self.map[p.1][p.0])).collect()
    }

    fn part_1(&mut self) -> usize {
        self.lowest_risk()
    }

    fn part_2(&mut self) -> usize {
        self.lowest_risk()
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();

        for (_i, row) in self.map.iter().enumerate() {
            for (_j, risk) in row.iter().enumerate() {
                display.push_str(&risk.to_string());
            }
            display.push_str("\n");
        }

        write!(f, "{}", display)
    }
}

pub fn read_stdin() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), std::io::Error> {
    let input = &read_stdin()?;
    let mut puzzle1 = Puzzle::parse_part1(input);
    println!("Part 1: {}", puzzle1.part_1());

    let mut puzzle2 = Puzzle::parse_part2(input);
    println!("Part 2: {}", puzzle2.part_2());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const SAMPLE: &str = r#"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "#;

    #[test]
    fn baby_steps() {
        let mut puzzle = super::Puzzle::parse_part1(
            r#"
            11
            91
        "#,
        );
        assert_eq!(2, puzzle.lowest_risk());

        puzzle = super::Puzzle::parse_part1(
            r#"
            19
            11
        "#,
        );
        assert_eq!(2, puzzle.lowest_risk());

        puzzle = super::Puzzle::parse_part1(
            r#"
            129
            129
            839
        "#,
        );
        assert_eq!(15, puzzle.lowest_risk());

        puzzle = super::Puzzle::parse_part1(
            r#"
            111
            275
            839
        "#,
        );
        assert_eq!(16, puzzle.lowest_risk());

        assert_eq!(2, puzzle.neighbours(&Pos(0, 0)).len());
        assert_eq!(2, puzzle.neighbours(&Pos(2, 0)).len());
        assert_eq!(2, puzzle.neighbours(&Pos(0, 2)).len());
        assert_eq!(2, puzzle.neighbours(&Pos(2, 2)).len());
        assert_eq!(4, puzzle.neighbours(&Pos(1, 1)).len());

        assert_eq!(
            3,
            puzzle
                .neighbours(&Pos(0, 0))
                .iter()
                .fold(0, |acc, n| acc + n.1)
        );

        assert_eq!(
            11,
            puzzle
                .neighbours(&Pos(1, 1))
                .iter()
                .fold(0, |acc, n| acc + n.1)
        );

        assert_eq!(
            8,
            puzzle
                .neighbours(&Pos(2, 2))
                .iter()
                .fold(0, |acc, n| acc + n.1)
        );

        puzzle = super::Puzzle::parse_part1(
            r#"
            19999
            19111
            11191"#,
        );
        assert_eq!(8, puzzle.lowest_risk());
    }

    #[test]
    fn part_1() {
        let mut puzzle = super::Puzzle::parse_part1(SAMPLE);

        assert_eq!(puzzle.map.len(), 10);
        assert_eq!(40, puzzle.part_1());
    }

    #[test]
    fn part_2() {
        let mut puzzle = super::Puzzle::parse_part2(SAMPLE);

        assert_eq!(puzzle.map.len(), 10 * 5); // 5 times larger in both dimensions
        assert_eq!(puzzle.map[0].len(), 10 * 5);
        assert_eq!(315, puzzle.part_2());
    }
}
