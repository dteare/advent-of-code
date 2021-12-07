use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 6! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

#[derive(Debug)]
struct Lanternfish {
    timer: usize,
}

#[derive(Debug)]
struct School {
    day: usize,
    fish: Vec<Lanternfish>,
}

impl std::fmt::Display for School {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.day == 0 {
            write!(f, "Initial state: ");
        } else if self.day == 1 {
            write!(f, "After  1 day:  ");
        } else {
            write!(f, "After {:2} days: ", self.day);
        }

        let mut fishies = self.fish.iter().peekable();
        while let Some(fish) = fishies.next() {
            write!(f, "{}", fish.timer);
            if fishies.peek().is_some() {
                write!(f, ",");
            }
        }
        write!(f, "")
    }
}

impl Lanternfish {
    // Returns true if a new Lanternfish was spawned
    fn age_by_a_day(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            return true;
        }

        self.timer -= 1;
        false
    }
}

impl School {
    fn parse(input: &str) -> School {
        println!("Parsing <{}>", input);
        let fish: Vec<Lanternfish> = input
            .split(",")
            .map(|s| Lanternfish {
                timer: s.parse::<usize>().unwrap(),
            })
            .collect();

        School { day: 0, fish }
    }

    fn age_by_n_days(&mut self, n: usize) {
        println!("Aging school of fishies by {} days\n\n{}", n, self);

        for _ in 1..n+1 {
            let mut spawned_newbies = 0;
            for fishie in self.fish.iter_mut() {
                if fishie.age_by_a_day() {
                    spawned_newbies += 1;
                }
            }

            for _ in 0..spawned_newbies {
                self.fish.push(Lanternfish { timer: 8 });
            }
            self.day += 1;

            // println!("{}", self);
        }
    }
}

fn load_from_file(file_name: &str) -> School {
    //  let input = File::open(file_name).expect("can't open {}", file_name);
    //  io::BufReader::new(input).lines().enumerate()
    let mut input = File::open(file_name).unwrap();
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).unwrap();
    School::parse(input_buffer.as_str())
}

fn star1() -> std::io::Result<()> {
    let mut school = load_from_file("../lanternfish-school.txt");

    println!("Starting with {} fish:\n{}", school.fish.len(), school);
    school.age_by_n_days(80);

    println!("⭐️ Analysis:");
    println!("   We end with {} fishies", school.fish.len());
 
    Ok(())
}

fn star2() -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    const INPUT_SAMPLE: &str = r#"3,4,3,1,2"#;
    const SAMPLE_RESULT_AFTER_18_DAYS: &str = r#"After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8"#;

    #[test]
    fn part_1_after_18_days() {
        let mut school = super::School::parse(INPUT_SAMPLE);

        println!("Loaded school of lanternfish:\n\n{}", school);
        assert_eq!(0, school.day);
        assert_eq!(5, school.fish.len());

        school.age_by_n_days(18);
        assert_eq!(18, school.day);
        assert_eq!(26, school.fish.len());
        assert_eq!(SAMPLE_RESULT_AFTER_18_DAYS, format!("{}", school));
    }

    #[test]
    fn part_1_after_80_days() {
        let mut school = super::School::parse(INPUT_SAMPLE);
        school.age_by_n_days(80);
        assert_eq!(5934, school.fish.len());
    }
}
