use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 6! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

// #[derive(Debug)]
// struct Lanternfish {
//     count: usize,  // how many fish w/ this timer are there?
// }

#[derive(Debug)]
struct School {
    day: usize,                 // the day of the sim we are on
    fish: [usize; 9],    // we collect like fish together so we only need 9 (0-8). array position represents the time left till spawn, and value is the count
}

// This only worked when each fish was kept separate. Exponential growth prevented us from doing that
// impl std::fmt::Display for School {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.day == 0 {
//             write!(f, "Initial state: ");
//         } else if self.day == 1 {
//             write!(f, "After  1 day:  ");
//         } else {
//             write!(f, "After {:2} days: ", self.day);
//         }

//         let mut fishies = self.fish.iter().peekable();
//         while let Some(fish) = fishies.next() {
//             write!(f, "{}", fish.timer);
//             if fishies.peek().is_some() {
//                 write!(f, ",");
//             }
//         }
//         write!(f, "")
//     }
// }

// impl Lanternfish {
//     // Returns true if a new Lanternfish was spawned
//     fn age_by_a_day(&mut self) -> bool {
//         if self.timer == 0 {
//             self.timer = 6;
//             return true;
//         }

//         self.timer -= 1;
//         false
//     }
// }

impl School {
    fn parse(input: &str) -> School {
        let mut fish:[usize; 9] = [0,0,0,0,0,0,0,0,0];
        // for i in 0..9 {
        //     fish[i] = 0;
        // }

        println!("Parsing <{}>", input);

        for i in input.split(",") {
           let timer = i.parse::<usize>().unwrap(); // remaining time for this fish
           fish[timer] += 1;
        }

        School { day: 0, fish }
    }

    fn number_of_fish(&self) -> usize {
        let sum: usize = self.fish.iter().sum();
        sum
    }

    fn age_by_n_days(&mut self, n: usize) {
        // println!("Aging school of fishies by {} days\n\n{}", n, self);

        for i in 1..n+1 {
            // println!("Day {}: {}", i, self.number_of_fish());

            // Those fish @ 0 are about to spawn, remember them for later
            let spawners = self.fish[0];

            self.fish[0] = self.fish[1];
            self.fish[1] = self.fish[2];
            self.fish[2] = self.fish[3];
            self.fish[3] = self.fish[4];
            self.fish[4] = self.fish[5];
            self.fish[5] = self.fish[6];
            self.fish[6] = self.fish[7];
            self.fish[7] = self.fish[8];
            
            // Spawned fish start with 8 days remaining
            self.fish[8] = spawners;

            // Spawning fish reset to 6
            self.fish[6] += spawners;

            self.day += 1;
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

//    println!("Starting with {} fish:\n{}", school.fish.len(), school);
    school.age_by_n_days(80);

    println!("⭐️ Analysis:");
    println!("   We end with {} fishies", school.number_of_fish());
 
    Ok(())
}

fn star2() -> std::io::Result<()> {
    let mut school = load_from_file("../lanternfish-school.txt");

    school.age_by_n_days(256);

    println!("⭐️⭐️ Analysis:");
    println!("   We end with {} fishies", school.number_of_fish());
 
    Ok(())
}

#[cfg(test)]
mod test {
    const INPUT_SAMPLE: &str = r#"3,4,3,1,2"#;
    const SAMPLE_RESULT_AFTER_18_DAYS: &str = r#"After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8"#;

    #[test]
    fn baby_steps() {
        let mut school = super::School::parse(INPUT_SAMPLE);

        // Initial state: 3,4,3,1,2
        assert_eq!(0, school.day);
        assert_eq!(5, school.number_of_fish());

        // After  1 day:  2,3,2,0,1
        school.age_by_n_days(1);
        assert_eq!(1, school.day);
        assert_eq!(5, school.number_of_fish());

        // After  2 days: 1,2,1,6,0,8
        school.age_by_n_days(1);
        assert_eq!(6, school.number_of_fish());

        // After  3 days: 0,1,0,5,6,7,8
        school.age_by_n_days(1);
        assert_eq!(7, school.number_of_fish());

        // After  4 days: 6,0,6,4,5,6,7,8,8
        school.age_by_n_days(1);
        assert_eq!(9, school.number_of_fish());

        // After  5 days: 5,6,5,3,4,5,6,7,7,8
        school.age_by_n_days(1);
        assert_eq!(10, school.number_of_fish());

        // After  6 days: 4,5,4,2,3,4,5,6,6,7
        school.age_by_n_days(1);
        assert_eq!(10, school.number_of_fish());

        // After  7 days: 3,4,3,1,2,3,4,5,5,6
        school.age_by_n_days(1);
        assert_eq!(10, school.number_of_fish());

        // After  8 days: 2,3,2,0,1,2,3,4,4,5
        school.age_by_n_days(1);
        assert_eq!(10, school.number_of_fish());

        // After  9 days: 1,2,1,6,0,1,2,3,3,4,8
        school.age_by_n_days(1);
        assert_eq!(11, school.number_of_fish());

        // After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
        school.age_by_n_days(1);
        assert_eq!(12, school.number_of_fish());
    }


    #[test]
    fn part_1_after_18_days() {
        let mut school = super::School::parse(INPUT_SAMPLE);

//        println!("Loaded school of lanternfish:\n\n{}", school);
        assert_eq!(0, school.day);
        assert_eq!(5, school.number_of_fish());

        school.age_by_n_days(18);
        assert_eq!(18, school.day);
        assert_eq!(26, school.number_of_fish());
//        assert_eq!(SAMPLE_RESULT_AFTER_18_DAYS, format!("{}", school));
    }

    #[test]
    fn part_1_after_80_days() {
        let mut school = super::School::parse(INPUT_SAMPLE);
        school.age_by_n_days(80);
        assert_eq!(5934, school.number_of_fish());
    }

    #[test]
    fn part_2() {
        let mut school = super::School::parse(INPUT_SAMPLE);
        school.age_by_n_days(256);
        assert_eq!(26984457539, school.number_of_fish());
    }
}
