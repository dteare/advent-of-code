use std::fmt;

#[derive (Debug, PartialEq)]
enum Direction {
    Up,
    Left,
}
#[derive (Debug, PartialEq)]
struct Fold {
    position: usize,
    direction: Direction,
}

struct Puzzle {
    paper: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        enum ParseMode {
            Points,
            Folds,
        }

        struct Point {
            x: usize,
            y: usize,
        }

        println!("Parsing <{}>", input);
        let mut paper: Vec<Vec<bool>> = Vec::with_capacity(15);
        paper.resize(15, vec![false; 11]);
        let mut folds: Vec<Fold> = Vec::new();
        let mut mode = ParseMode::Points;

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            println!("<{}>", line_str);
            let trimmed = line_str.trim();
            if trimmed.len() == 0 {
                mode = ParseMode::Folds;
                continue;
            }

            match mode {
                ParseMode::Points => {
                    println!("spliting <{}> on ,", trimmed);
                    let mut parts = trimmed.split(",");
                    let x = parts.next().unwrap().parse::<usize>().unwrap();
                    let y = parts.next().unwrap().parse::<usize>().unwrap();

                    println!("Parsed point {} as ({},{})", trimmed, x, y);
                    paper[y][x] = true;
                }
                ParseMode::Folds => {
                    println!("Parsing fold <{}> on =", trimmed);
                    let mut parts = trimmed.split("=");
                    let mut direction = Direction::Up;
                    let dir_str = parts.next().unwrap();
                    if dir_str.chars().last().unwrap() == 'x' {
                        direction = Direction::Left;
                    }

                    let pos_str = parts.next().unwrap();
                    let position = pos_str.parse::<usize>().unwrap();

                    folds.push(Fold {
                        direction,
                        position,
                    });
                }
            }
        }

        Puzzle { paper, folds }
    }

    // Fold the puzzle `n` times.
    fn fold(&mut self, n: usize) {
        let fold = self.folds.remove(0);
        match fold.direction {
            Direction::Up => {
                println!("FOLDING UP @ {}", fold.position);
                let height = self.paper.len() / 2;
                let width = self.paper[0].len();
                let mut paper: Vec<Vec<bool>> = Vec::with_capacity(height);
                paper.resize(height, vec![false; width]);

                println!("Folding paper into height {}, width {}", height, width);

                // Assumes we always fold in half...
                for y in 0..self.paper.len()/2 {
                    println!("COMBINING @ line {}\n    {:?}\n    {:?}", y, self.paper[y], self.paper[self.paper.len()-1-y]);

                    for x in 0..width {
                        if self.paper[y][x] || self.paper[self.paper.len()-1-y][x] {
                            paper[y][x] = true;
                        }
                    }

                    println!("  👉🏻 {:?}", paper[y]);
                }

                self.paper = paper;
            },
            Direction::Left => {
                println!("FOLDING LEFT @ {}", fold.position);

                let height = self.paper.len();
                let width = self.paper[0].len()/2;
                let mut paper: Vec<Vec<bool>> = Vec::with_capacity(height);
                paper.resize(height, vec![false; width]);

                println!("Folding paper into height {}, width {}", height, width);

                // Assumes we always fold in half...
                for y in 0..self.paper.len() {
                    for x in 0..width {
                        if self.paper[y][x] || self.paper[y][self.paper[0].len()-1 - x] {
                            paper[y][x] = true;
                        }
                    }

                    println!("  👉🏻 {:?}", paper[y]);
                }

                self.paper = paper;
            },
        }
    }

    // Count how many dots are visible
    fn dot_count(&self) -> usize {
        let mut sum = 0;
        for row in self.paper.iter() {
            sum = row
                .iter()
                .fold(sum, |acc, entry| if *entry { acc + 1 } else { acc });
        }

        sum
    }

    // Complete the puzzle by performing all folds
    fn complete(&mut self) {
        while self.folds.len() > 0 {
            self.fold(1);
        }
    }

    fn part_1(&mut self) -> usize {
        self.fold(1);
        self.dot_count()
    }

    fn part_2(&mut self) -> usize {
        0
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();

        for (_i, line) in self.paper.iter().enumerate() {
            for (_j, entry) in line.iter().enumerate() {
                let mut visual = '.';
                if *entry == true {
                    visual = '#';
                }
                display.push_str(visual.to_string().as_str());
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
    let mut puzzle = Puzzle::parse(&read_stdin()?);

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const SAMPLE: &str = r#"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5
    "#;

    #[allow(dead_code)] // wtf?
    fn assert_eq_puzzles(puzzle: &Puzzle, map: &str) {
        println!(
            "@assert_eq_energy_levels with #{} folds remaining",
            puzzle.folds.len()
        );

        let actual = puzzle.to_string().trim().to_string();
        let expected = map.trim().to_string();

        if actual != expected {
            println!("💥 Disruptance in the (origami) force with {} folds remaining. Actual:\n<{}>\nExpected:\n<{}>", puzzle.folds.len(), actual, expected);
            assert!(false,
                        "Origami visual mismatch with {} folds remaining. Actual is <{}>; expected was <{}>.",
                        puzzle.folds.len(), actual, expected
                    );
        }

        // for i in 0..height {
        //     for j in 0..width {
        //         let actual = puzzle.paper[i][j];
        //         let expected = expected_puzzle.paper[i][j];

        //         if actual != expected {
        //             println!("💥 Disruptance in the (origami) force at ({},{}) with {} folds remaining:\n{}", i, j, puzzle.folds.len(), puzzle);
        //             assert!(false,
        //                 "Origami visual mismatch with {} folds remaining at ({},{}). Expected {}; was {}.",
        //                 puzzle.folds.len(), i, j, expected, actual
        //             );
        //         }
        //     }
        // }
    }

    #[test]
    fn baby_steps() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);

        assert_eq!(puzzle.paper.len(), 15);
        assert_eq!(puzzle.paper[0].len(), 11);
        assert_eq!(puzzle.folds.len(), 2);
        assert_eq!(puzzle.folds[0], Fold{direction:Direction::Up, position:7});
        assert_eq!(puzzle.folds[1], Fold{direction:Direction::Left, position:5});

        assert_eq_puzzles(
            &puzzle,
            r#"
...#..#..#.
....#......
...........
#..........
...#....#.#
...........
...........
...........
...........
...........
.#....#.##.
....#......
......#...#
#..........
#.#........"#);

        puzzle.fold(1);
        assert_eq_puzzles(
            &puzzle,r#"
#.##..#..#.
#...#......
......#...#
#...#......
.#.#..#.###
...........
..........."#);

        puzzle.fold(1);
        assert_eq_puzzles(
            &puzzle,
            r#"
#####
#...#
#...#
#...#
#####
.....
....."#);
    }

    #[test]
    fn part_1() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(17, puzzle.part_1());
    }
}
