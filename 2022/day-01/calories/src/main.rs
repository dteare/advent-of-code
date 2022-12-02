use std::fmt;

struct Puzzle {
    elves: Vec<Elf>,
}

struct Elf {
    inventory: Vec<usize>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        println!("Parsing <{}>", input);
        let mut elves: Vec<Elf> = Vec::new();
        let mut inventory: Vec<usize> = Vec::new();

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            let trimmed = line_str.trim();
            if trimmed.len() == 0 {
                // New line signifies end of the current Elf's item list
                elves.push(Elf { inventory });
                inventory = Vec::new();
                continue;
            }

            let calories = line_str.parse::<usize>().unwrap();
            inventory.push(calories);
        }

        // Not all files will end with a new line. Add the last Elf if appropriate
        elves.push(Elf { inventory });

        Puzzle { elves }
    }

    fn part_1(&mut self) -> usize {
        let mut max_calories: usize = 0;

        for (i, elf) in self.elves.iter().enumerate() {
            let calories = elf.inventory.iter().sum::<usize>();
            println!(
                "Elf #{} has {} items with a total of {} calories",
                i,
                elf.inventory.len(),
                calories
            );

            if calories > max_calories {
                max_calories = calories;
            }
        }

        max_calories
    }

    fn part_2(&mut self) -> usize {
        2
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();

        for (_i, elf) in self.elves.iter().enumerate() {
            display.push_str(" * ");
            display.push_str(&elf.to_string());
        }

        write!(f, "{}", display)
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();

        for (_i, item) in self.inventory.iter().enumerate() {
            display.push_str(&item.to_string());
            display.push_str(",");
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
    //    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const SAMPLE: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn baby_steps() {
        let puzzle = super::Puzzle::parse(SAMPLE);

        assert_eq!(puzzle.elves.len(), 5);
        assert_eq!(puzzle.elves[0].inventory.len(), 3);
        assert_eq!(puzzle.elves[1].inventory.len(), 1);
        assert_eq!(puzzle.elves[1].inventory[0], 4000);
    }

    #[test]
    fn part_1() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(24000, puzzle.part_1());
    }

    // #[test]
    // fn part_2() {
    //     let mut puzzle = super::Puzzle::parse(SAMPLE);
    //     assert_eq!(12345, puzzle.part_2());
    // }
}
