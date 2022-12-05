use std::collections::HashMap;
use std::collections::VecDeque;
struct Puzzle {
    rucksacks: Vec<Rucksack>,
}

#[derive(Clone)]
struct Rucksack {
    comparment_1: HashMap<char, usize>,
    comparment_2: HashMap<char, usize>,
}

// todo someday maybe
// type Priority = usize;
// impl From<char> for Priority {
//     fn from(c: char) -> Self {
//         ...
//     }
// }

fn parse_items(input: &str) -> (Vec<char>, Vec<char>) {
    let parts = input.chars();
    let mut all = VecDeque::from_iter(parts);

    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();

    while all.len() > 0 {
        left.push(all.pop_front().unwrap());
        right.push(all.pop_back().unwrap());
    }
    (left, right)
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        //println!("Parsing <{}>", input);
        let mut rucksacks: Vec<Rucksack> = Vec::new();

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            let _trimmed = line_str.trim();

            let (items1, items2) = parse_items(line_str);

            let coalesced_items1 = items1.into_iter().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

            let coalesced_items2 = items2.into_iter().fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

            rucksacks.push(Rucksack {
                comparment_1: coalesced_items1,
                comparment_2: coalesced_items2,
            });
        }

        Puzzle { rucksacks }
    }

    fn priorty_sum(&self) -> usize {
        self.rucksacks
            .iter()
            .fold(0, |acc, r| acc + r.duplicate().1)
    }

    fn part_1(&self) -> usize {
        self.priorty_sum()
    }

    fn part_2(&self) -> usize {
        0
    }
}

impl Rucksack {
    fn duplicate(&self) -> (char, usize) {
        // The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
        let mut dupe: Option<char> = None;
        for k in self.comparment_1.keys() {
            if self.comparment_2.contains_key(k) {
                dupe = Some(*k);
            }
        }

        let d = dupe.unwrap();
        (d, self.priority(d))
    }

    fn priority(&self, c: char) -> usize {
        // Lowercase item types a through z have priorities 1 through 26.
        // Uppercase item types A through Z have priorities 27 through 52.
        let ascii = c as usize;

        if ascii >= 97 {
            ascii - 96 // a = 1
        } else {
            ascii - 38 // A == 27
        }
    }
}

pub fn read_stdin() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), std::io::Error> {
    let input = &read_stdin()?;
    let puzzle = Puzzle::parse(input);
    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const SAMPLE: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn baby_steps() {
        let puzzle = super::Puzzle::parse(SAMPLE);

        assert_eq!(puzzle.rucksacks.len(), 6);

        // First rucksack has items vJrwpWtwJgWrhcsFMMfFFhFp stored in:
        //   compartment 1 vJrwpWtwJgWr -> v,J2,r2,w2,p,W2,t,g
        //   compartment 2 hcsFMMfFFhFp -> h2,c,s,F4,M2,f,p
        assert_eq!(puzzle.rucksacks[0].comparment_1.len(), 8);
        assert_eq!(puzzle.rucksacks[0].comparment_2.len(), 7);

        assert_eq!(puzzle.rucksacks[0].comparment_1[&'v'], 1);
        assert_eq!(puzzle.rucksacks[0].comparment_1[&'p'], 1);
        assert_eq!(puzzle.rucksacks[0].comparment_2[&'F'], 4);
        assert_eq!(puzzle.rucksacks[0].comparment_2[&'p'], 1);

        assert_eq!(puzzle.rucksacks[0].duplicate(), ('p', 16));
        assert_eq!(puzzle.rucksacks[1].duplicate(), ('L', 38));
        assert_eq!(puzzle.rucksacks[2].duplicate(), ('P', 42));
        assert_eq!(puzzle.rucksacks[3].duplicate(), ('v', 22));
        assert_eq!(puzzle.rucksacks[4].duplicate(), ('t', 20));
        assert_eq!(puzzle.rucksacks[5].duplicate(), ('s', 19));

        assert_eq!(puzzle.priorty_sum(), 157);
    }

    #[test]
    fn part_1() {}

    #[test]
    fn part_2() {}
}
