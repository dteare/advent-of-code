use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 4! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

#[derive (Debug)]
struct Square {
    number: u32,
    called: bool,
}

#[derive (Debug)]
struct Board {
    squares: Vec<Square>,
}

#[derive (Debug)]
/// Game is the setup for the entire _room_. All players, boards, and the numbers to be called. 
struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Board {
    /// Calculate the score of the winning board
    fn score(&self) -> u32 {
        0
    }

    /// Look for a winning condition. Note that only horizontal and vertical rows are considered (no diagonals)
    fn is_winner(&self) -> bool {
        false
    }

    /// Mark our matching squares, if any. Returns true if one is found.
    fn mark_number_called(&mut self, number:u32) -> bool {
        for i in 0..self.squares.len() {
            let mut square = &mut self.squares[i];
            if square.number == number {
                square.called = true;
                return true; // Repeating numbers aren't allowed
            }
        }

        false
    }
}

impl Game {
    fn parse_game_setup(setup: &str) -> Game {
        let mut lines: Vec<&str> = setup.split('\n').collect();
    
        // First line is a comma-separated list of numbers, in the order to be called
        let numbers_csv = lines.remove(0);
        let numbers: Vec<u32> = numbers_csv.split(",").map(|v| v.parse::<u32>().unwrap()).collect();
    
        assert_eq!(lines.len() % 6, 0, "Each board must consist of 5 lines with 1 leading newline");
    
        let mut boards:Vec<Board> = Vec::new();
        for (count, seed) in lines.chunks(6).enumerate() {
            assert_eq!(seed[0], "","Expected first line of board {} to be empty: <{}>", count, seed[0]);
        
            let mut squares:Vec<Square> = Vec::new();
            println!("Parsing board #{}", count);
            for (i, line) in seed.iter().skip(1).enumerate() {
                println!("    Line {} = {}", i, line);
                for n in line.split_whitespace() {
    
                    println!("   n={}", n);
                    let square = Square{number:n.parse::<u32>().unwrap(), called:false};
                    squares.push(square);
                }
            }
    
            boards.push(Board{squares});
        }
    
        Game{numbers, boards}
    }

    // Plays the game by calling each number in turn, and checking for a winner after each pass. Returns the index of the winning Board, if any.
    fn play(&mut self) -> Option<usize> {
        for i in 0..self.numbers.len() {
            let number = self.numbers[i];
            println!("Step #{} of the game is now calling number {}", i, number);

            self.mark_number_called(number);

            let winner_idx = self.check_for_winner();
            match winner_idx {
                Some(idx) => {
                    println!("Winner!");
                    return Some(idx);
                }
                None => {
                    println!("No winners yet!");
                }
            }
        }

        None
    }

    fn mark_number_called(&mut self, number:u32) {
        for i in 0..self.boards.len() {
            let board = &mut self.boards[i];
            let marked = board.mark_number_called(number);
            if marked {
                println!("Marked! Board {} had a match for {}", i, number);
            }
        }
    }

    fn check_for_winner(&self) -> Option<usize> {
        None
    }
}


fn load_game_from_file(file_name: &str) -> Game {
    let mut input = File::open(file_name).unwrap();
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).unwrap();
    Game::parse_game_setup(input_buffer.as_str())
}

fn star1() -> std::io::Result<()> {
    let game = load_game_from_file("../bingo-test.txt");

    println!("Game setup has {} numbers to call and {} boards", game.numbers.len(), game.boards.len());

    // let answer = horiz_position * depth;

    // println!("⭐️ Analysis:");
    // println!("   The valet parked your sub at position {}, depth {}", horiz_position, depth);
    // println!("   Multipled, position x depth = {}", answer);

    Ok(())
}

fn star2() -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn parse_game_setup() {
        let game = super::Game::parse_game_setup(INPUT);

        assert_eq!(game.numbers.len(), 27);
        assert_eq!(game.boards.len(), 3);

        for (i, board) in game.boards.iter().enumerate() {
            assert_eq!(board.squares.len(), 25, "Board {} does not have 25 squares", i)
        }
    }

    #[test]
    fn play() {
        let mut game = super::Game::parse_game_setup(INPUT);
        let winner_idx = game.play();

        assert!(winner_idx.is_some(), "There must be a winner or the players get angry");

        assert_eq!(game.boards[winner_idx.unwrap()].score(), 4512);
    }
}