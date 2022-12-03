use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
enum Ending {
    Win,
    Lose,
    Draw,
}

// How to interpret the X, Y, Z encoding within the input
enum Encoding {
    WhatToPlay, // star 1
    HowItEnds,  // star 2
}

struct Tournament {
    rounds: Vec<Round>,
}

#[derive(Clone)]
struct Round {
    theirs: Choice,
    mine: Choice,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl FromStr for Ending {
    type Err = ();

    fn from_str(input: &str) -> Result<Ending, Self::Err> {
        match input {
            "X" => Ok(Ending::Lose),
            "Y" => Ok(Ending::Draw),
            "Z" => Ok(Ending::Win),
            _ => Err(()),
        }
    }
}

fn winning_play(theirs: &Choice) -> Choice {
    match theirs {
        Choice::Rock => Choice::Paper,
        Choice::Paper => Choice::Scissors,
        Choice::Scissors => Choice::Rock,
    }
}

fn losing_play(theirs: &Choice) -> Choice {
    match theirs {
        Choice::Rock => Choice::Scissors,
        Choice::Paper => Choice::Rock,
        Choice::Scissors => Choice::Paper,
    }
}

fn drawing_play(theirs: &Choice) -> Choice {
    match theirs {
        Choice::Rock => Choice::Rock,
        Choice::Paper => Choice::Paper,
        Choice::Scissors => Choice::Scissors,
    }
}

fn parse_choices(input: &str, mode: &Encoding) -> (Choice, Choice) {
    let mut parts = input.split(" ");
    let theirs = Choice::from_str(parts.next().unwrap()).unwrap();

    let my_str = parts.next().unwrap();

    match mode {
        Encoding::WhatToPlay => {
            let mine = Choice::from_str(my_str).unwrap();
            (theirs, mine)
        }
        Encoding::HowItEnds => {
            let ending = Ending::from_str(my_str).unwrap();
            let mine = match ending {
                Ending::Win => winning_play(&theirs),
                Ending::Lose => losing_play(&theirs),
                Ending::Draw => drawing_play(&theirs),
            };
            (theirs, mine)
        }
    }
}

impl Tournament {
    fn parse(input: &str, mode: Encoding) -> Tournament {
        //println!("Parsing <{}>", input);
        let mut rounds: Vec<Round> = Vec::new();

        for (_i, line_str) in input.trim().split("\n").enumerate() {
            let _trimmed = line_str.trim();

            let (theirs, mine) = parse_choices(line_str, &mode);

            rounds.push(Round {
                theirs: theirs,
                mine: mine,
            });
        }

        Tournament { rounds }
    }

    fn part_1(&self) -> usize {
        self.rounds.iter().map(|r| r.score()).sum()
    }

    fn part_2(&self) -> usize {
        self.rounds.iter().map(|r| r.score()).sum()
    }
}

impl Round {
    fn score(&self) -> usize {
        let choice_score = match self.mine {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        let win_score = match self.mine {
            Choice::Rock => {
                match self.theirs {
                    Choice::Rock => 3,     // tie
                    Choice::Paper => 0,    // they won
                    Choice::Scissors => 6, // I won
                }
            }
            Choice::Paper => {
                match self.theirs {
                    Choice::Rock => 6,     // I win
                    Choice::Paper => 3,    // Tie
                    Choice::Scissors => 0, // I lost!
                }
            }
            Choice::Scissors => {
                match self.theirs {
                    Choice::Rock => 0,     // I lose
                    Choice::Paper => 6,    // I win!
                    Choice::Scissors => 3, // Tie
                }
            }
        };

        choice_score + win_score
    }
}

pub fn read_stdin() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), std::io::Error> {
    let input = &read_stdin()?;
    let tourney = Tournament::parse(input, Encoding::WhatToPlay);
    println!("Part 1: {}", tourney.part_1());

    let tourney = Tournament::parse(input, Encoding::HowItEnds);
    println!("Part 2: {}", tourney.part_2());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const SAMPLE: &str = r#"
A Y
B X
C Z
"#;

    #[test]
    fn baby_steps() {
        let tourney = super::Tournament::parse(SAMPLE, Encoding::WhatToPlay);

        assert_eq!(tourney.rounds.len(), 3);
        assert_eq!(tourney.rounds[0].theirs, Choice::Rock);
        assert_eq!(tourney.rounds[0].mine, Choice::Paper);
        assert_eq!(tourney.rounds[0].score(), 8);

        assert_eq!(tourney.rounds[1].theirs, Choice::Paper);
        assert_eq!(tourney.rounds[1].mine, Choice::Rock);
        assert_eq!(tourney.rounds[1].score(), 1);

        assert_eq!(tourney.rounds[2].theirs, Choice::Scissors);
        assert_eq!(tourney.rounds[2].mine, Choice::Scissors);
        assert_eq!(tourney.rounds[2].score(), 6);
    }

    #[test]
    fn part_1() {
        let mut tourney = super::Tournament::parse(SAMPLE, Encoding::WhatToPlay);
        assert_eq!(15, tourney.part_1());
    }

    #[test]
    fn part_2() {
        let mut tourney = super::Tournament::parse(SAMPLE, Encoding::HowItEnds);
        assert_eq!(12, tourney.part_2());
    }
}
