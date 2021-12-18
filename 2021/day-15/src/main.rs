use std::collections::HashMap;
use std::fmt;

struct Puzzle {
    map: Vec<Vec<usize>>,
    cache: HashMap<(usize,usize),usize>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        println!("Parsing <{}>", input);
        let mut map: Vec<Vec<usize>> = Vec::new();

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            let trimmed = line_str.trim();
            if trimmed.len() == 0 {
                continue;
            }

            let mut row:Vec<usize> = Vec::new();
            for (_j, c) in trimmed.chars().enumerate() {
                let risk = c.to_digit(10).unwrap();
                row.push(risk.try_into().unwrap());
            }
            map.push(row);
        }

        let cache: HashMap<(usize,usize),usize> = HashMap::new();
        Puzzle { map, cache }
    }

    fn lowest_risk(&mut self) -> usize {
        self.path_walker(0, 0)  - self.map[0][0]
    }

    /// (row,col) is the starting position. Think of it as the remaining square after navigating (and starting from) this position. Assume no switchbacks so the shortest path is either the one to the right or the one going down. 
    fn path_walker(&mut self, row:usize, col:usize) -> usize {
        //println!("@path_walker ({},{})", row, col);
        if row > self.map.len()-1 && col > self.map[0].len()-1 {
            panic!("Coding error. Should have avoided recursing this deep. ({}, {})", row, col);
        }

        if row == self.map.len() - 1 && col == self.map[0].len()-1 {
            return self.map[row][col];
        }

        if row >= self.map.len() - 1 {
            return self.map[row][col] + self.path_walker(row, col+1);
        }
        if col >= self.map[0].len() - 1 {
            return self.map[row][col] + self.path_walker(row+1, col);
        }


        let right_risk:usize;
        {
            let cached_right = self.cache.get_mut(&(row, col+1));
            match cached_right {
                Some(val) => {
                    right_risk = *val;
                },
                None => {
                    right_risk = self.path_walker(row, col+1);
                }
            }
        }

        let cached_down = self.cache.get_mut(&(row+1, col));
        let down_risk:usize;
        match cached_down {
            Some(val) => {
                down_risk = *val;
            },
            None => {
                down_risk = self.path_walker(row+1, col);
            }
        }

        let result:usize;
        if right_risk < down_risk {
            result = self.map[row][col] + right_risk
        }
        else if down_risk < right_risk {
            result = self.map[row][col] + down_risk
        }
        else {
            println!("⚠️ Point ({},{})={} Tricky map has equal risk right and down. Choosing right.", row, col, self.map[row][col]);
            result = self.map[row][col] + right_risk
        }

        self.cache.insert((row,col), result);
        result
    }

    fn part_1(&mut self) -> usize {
        self.lowest_risk()
    }

    fn part_2(&mut self) -> usize {
        0
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
    let mut puzzle = Puzzle::parse(&read_stdin()?);

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2:\n{}", puzzle.part_2());

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
        let mut puzzle = super::Puzzle::parse(r#"
            11
            91
        "#);
        assert_eq!(2, puzzle.lowest_risk());


        puzzle = super::Puzzle::parse(r#"
            19
            11
        "#);
        assert_eq!(2, puzzle.lowest_risk());


        puzzle = super::Puzzle::parse(r#"
            129
            129
            839
        "#);
        assert_eq!(15, puzzle.lowest_risk());


        puzzle = super::Puzzle::parse(r#"
            111
            275
            839
        "#);
        assert_eq!(16, puzzle.lowest_risk());
    }

    #[test]
    fn part_1() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);

        println!("Parsed puzzle:\n{}", puzzle);

        assert_eq!(puzzle.map.len(), 10);
        assert_eq!(40, puzzle.part_1());
    }

    // #[test]
    // fn part_2() {
    //     let mut puzzle = super::Puzzle::parse(SAMPLE);
    //     assert_eq!(2188189693529, puzzle.part_2());
    // }
}
