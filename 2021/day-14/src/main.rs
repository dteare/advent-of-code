use std::collections::HashMap;
use std::fmt;

struct Puzzle {
    polymer: Vec<char>,
    pairs: HashMap<String, char>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        enum ParseMode {
            Template,
            Pairs,
        }

        println!("Parsing <{}>", input);
        let mut polymer: Vec<char> = Vec::new();
        let mut pairs: HashMap<String, char> = HashMap::new();
        let mut mode = ParseMode::Template;

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            println!("<{}>", line_str);
            let trimmed = line_str.trim();
            if trimmed.len() == 0 {
                mode = ParseMode::Pairs;
                continue;
            }

            match mode {
                ParseMode::Template => {
                    println!("Parsing template <{}>", trimmed);
                    for c in trimmed.chars() {
                        polymer.push(c);
                    }
                }
                ParseMode::Pairs => {
                    println!("Parsing insertion pair <{}>", trimmed);
                    let mut parts = trimmed.split(" -> ");
                    let pair = parts.next().unwrap();
                    let insert = parts.next().unwrap();
                    pairs.insert(pair.to_string(), insert.chars().last().unwrap());
                }
            }
        }

        Puzzle { polymer, pairs }
    }

    fn step(&mut self, n:usize) {
        for _ in 0..n {
            for i in (1..self.polymer.len()).rev() {
                println!("Looking at {} and {}: {}-{}", i-1, i, self.polymer[i-1], self.polymer[i]);
    
                let mut template_pair = self.polymer[i-1].to_string();
                template_pair.push(self.polymer[i]);
    
                let mut insertion:Option<char> = None;
                for (pair,insert) in self.pairs.iter() {
                    println!("matching {} against {}", pair, template_pair);
    
                    if template_pair == *pair {
                        insertion = Some(*insert);
                    }
                }
    
                if let Some(c) = insertion {
                    self.polymer.insert(i, c);
                }
            }
        }
    }

    fn count(&self) -> HashMap<char, usize> {
        let mut r:HashMap<char,usize> = HashMap::new();

        for p in self.polymer.iter() {
            let existing_count = r.get(p);

            match existing_count {
                Some(count) => {
                    r.insert(*p, count+1);
                }, 
                None => {
                    r.insert(*p, 1);
                },
            }
        }

        r
    }

    fn part_1(&mut self) -> usize {
        self.step(10);
        let count = self.count();

        let mut most_common: (char, usize) = ('_', 0);
        let mut least_common: (char, usize) = ('_', usize::MAX);

        for (p, c) in count.iter() {
            if *c > most_common.1 {
                most_common = (*p, *c);
            }
            if *c < least_common.1 {
                least_common = (*p, *c);
            }
        }

        most_common.1 - least_common.1
    }

    fn part_2(&mut self) -> usize {
        0
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();

        for (_i, p) in self.polymer.iter().enumerate() {
            display.push_str(&p.to_string());
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
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "#;

    #[test]
    fn baby_steps() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);

        assert_eq!(puzzle.polymer.len(), 4);
        assert_eq!(puzzle.pairs.len(), 16);
        assert_eq!(puzzle.to_string(), "NNCB");

        puzzle.step(1);
        assert_eq!(puzzle.to_string(), "NCNBCHB");

        puzzle.step(1);
        assert_eq!(puzzle.to_string(), "NBCCNBBBCBHCB");

        puzzle.step(1);
        assert_eq!(puzzle.to_string(), "NBBBCNCCNBBNBNBBCHBHHBCHB");

        puzzle.step(1);
        assert_eq!(puzzle.to_string(), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");

        puzzle.step(1);
        assert_eq!(puzzle.polymer.len(), 97);

        puzzle.step(5);
        assert_eq!(puzzle.polymer.len(), 3073);

        let count = puzzle.count();
        assert_eq!(count[&'B'], 1749);
        assert_eq!(count[&'C'], 298);
        assert_eq!(count[&'H'], 161);
        assert_eq!(count[&'N'], 865);
    }

    #[test]
    fn part_1() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(1588, puzzle.part_1());
    }
}
