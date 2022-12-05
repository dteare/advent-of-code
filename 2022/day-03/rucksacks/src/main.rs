#![feature(array_chunks)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
struct Puzzle {
    rucksacks: Vec<Rucksack>,
}

#[derive(Clone, Debug)]
struct Rucksack {
    comparment_1: HashMap<char, usize>,
    comparment_2: HashMap<char, usize>,
    all_items: HashSet<char>,
}

// todo someday maybe
// type Priority = usize;
// impl From<char> for Priority {
//     fn from(c: char) -> Self {
//         ...
//     }
// }

fn parse_items(input: &str) -> (Vec<char>, Vec<char>, Vec<char>) {
    let parts = input.chars();
    let mut items = VecDeque::from_iter(parts);

    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();

    while items.len() > 0 {
        left.push(items.pop_front().unwrap());
        right.push(items.pop_back().unwrap());
    }
    (
        left.clone(),
        right.clone(),
        left.into_iter().chain(right).collect(),
    )
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        //println!("Parsing <{}>", input);
        let mut rucksacks: Vec<Rucksack> = Vec::new();

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            let _trimmed = line_str.trim();

            let (items1, items2, all) = parse_items(line_str);

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
                all_items: HashSet::from(all.into_iter().collect::<HashSet<char>>()),
            });
        }

        Puzzle { rucksacks }
    }

    fn priorty_sum(&self) -> usize {
        self.rucksacks
            .iter()
            .fold(0, |acc, r| acc + r.duplicate().1)
    }

    fn item_in_common(&self, rucksacks: &[Rucksack]) -> (char, usize) {
        let overlap = rucksacks[0].all_items.intersection(&rucksacks[1].all_items);

        for c in overlap.into_iter() {
            if rucksacks[2].all_items.contains(c) {
                return (*c, rucksacks[2].priority(*c));
            }
        }

        panic!("There was no item in common for this group of rucksacks");
    }

    fn group_badge_priority_sum(&self) -> usize {
        let mut sum = 0;

        for group in self.rucksacks.array_chunks::<3>() {
            let (_, priority) = self.item_in_common(group);
            sum += priority;
        }

        sum
    }

    fn part_1(&self) -> usize {
        self.priorty_sum()
    }

    fn part_2(&self) -> usize {
        self.group_badge_priority_sum()
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
    }

    #[test]
    fn part_1() {
        let puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(puzzle.priorty_sum(), 157);
    }

    #[test]
    fn part_2() {
        let puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(puzzle.group_badge_priority_sum(), 70);
    }
}
