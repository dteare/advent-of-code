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
    marked: bool,
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
    last_called_number: Option<u32>,
}

impl Board {
    /// Calculate the score of the winning board
    fn score(&self, last:u32) -> u32 {
        self.sum_unmarked_numbers() * last
    }

    /// Look for a winning condition. Note that only horizontal and vertical rows are considered (no diagonals)
    fn is_winner(&self) -> bool {

        // Check rows
        for (row, squares) in self.squares.chunks(5).enumerate() {
            println!("Checking row {}: [{}, {}, {}, {}, {}]", row, squares[0].number, squares[1].number, squares[2].number, squares[3].number, squares[4].number);

            let mut row_all_marked = true;
            for i in 0..squares.len() {
                if squares[i].marked == false {
                    row_all_marked = false;
                }
            }

            if row_all_marked {
                println!("🎉 Winner winner! Chicken dinner! Row {} is complete.", row);
                return true;
            }
        }

        // Check columns
        for col in 0..5 {
            println!("Checking column {}", col);

            let mut col_all_marked = true;
            for row_squares in self.squares.chunks(5) {    
                println!("   {}", row_squares[col].number);
                if row_squares[col].marked == false {
                    println!("   {} of column {} was not marked, killing it", row_squares[col].number, col);
                    col_all_marked = false;
                }
            }

            if col_all_marked {
                println!("🎉 Winner winner! Chicken dinner! Column {} is complete.", col);
                return true;
            }
        }

        false
    }

    /// Mark our matching squares, if any. Returns true if one is found.
    fn mark_number_called(&mut self, number:u32) -> bool {
        let mut marked = false;
        for i in 0..self.squares.len() {
            let mut square = &mut self.squares[i];
            if square.number == number {
                square.marked = true;
                marked = true; // Repeating numbers typically are not allowed but we continue as it wasn't explicitly stated
            }
        }

        marked
    }

    /// Calculate the sum of all unmarked numbers (e.g. the numbers that were not called yet)
    fn sum_unmarked_numbers(&self) -> u32 {
        let mut sum = 0;
        for square in self.squares.iter() {
            if !square.marked {
                sum += square.number;
            }
        }
        return sum;
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
//            println!("Parsing board #{}", count);
            for (_, line) in seed.iter().skip(1).enumerate() {
//                println!("    Line {} = {}", i, line);
                for n in line.split_whitespace() {
    
//                    println!("   n={}", n);
                    let square = Square{number:n.parse::<u32>().unwrap(), marked:false};
                    squares.push(square);
                }
            }
    
            boards.push(Board{squares});
        }
    
        Game{numbers, boards, last_called_number:None}
    }

    // Plays the game by calling each number in turn, and checking for a winner after each pass. Returns the index of the winning Board, if any.
    fn play(&mut self) -> Option<usize> {
        for i in 0..self.numbers.len() {
            let number = self.numbers[i];
            println!("📣📣📣📣 CALLING – Step #{} of the game is now calling number {}", i, number);

            self.mark_number_called(number);

            let winner_idx = self.check_for_winner();
            match winner_idx {
                Some(idx) => {
                    println!("Winner! Step #{} triggered the win when calling {}", i, number);
                    self.last_called_number = Some(number);
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

    /// See if there are any winning boards. Simplifying assumption that there are no ties. Ties are left as an exercise for the reader.
    fn check_for_winner(&self) -> Option<usize> {
        for (i, board) in self.boards.iter().enumerate() {
            if board.is_winner() {
                return Some(i);
            }
        }

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
    let mut game = load_game_from_file("../bingo.txt");

    println!("Game setup has {} numbers to call and {} boards", game.numbers.len(), game.boards.len());

    let winning_idx = game.play().unwrap();
    let board = &game.boards[winning_idx];

    println!("⭐️ Analysis:");
    println!("   Board {} won", winning_idx);
    println!("   Sum of unmarked numbers = {}", board.sum_unmarked_numbers());
    println!("   Final score = {}", board.score(game.last_called_number.unwrap()));

    Ok(())
}

fn star2() -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    const INPUT_OFFICIAL_TEST: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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

const INPUT_SHUFFLED_TEST: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
21 17 24  04 14 

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6"#;

const INPUT_ROW_WINNER_TEST: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

18  8 23 26 20
22 11 13  6  5
7  4  9  5 11
2  0 12  3  7
21 17 24  04 14 

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6"#;


const INPUT_COL_WINNER_TEST: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

71  4  9  5  7
18  8 23 26  4
22 11 13  6  9
2   0 12  3  5
21 17 24  4 11 

 7 13 17 11  0
 4  2 23  4 24
 9  9 14 16  7
 5 10  3 18  5
17 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6"#;

    #[test]
    fn play_game_1_official_data() {
        let mut game = super::Game::parse_game_setup(INPUT_OFFICIAL_TEST);

        assert_eq!(game.numbers.len(), 27);
        assert_eq!(game.boards.len(), 3);

        for (i, board) in game.boards.iter().enumerate() {
            assert_eq!(board.squares.len(), 25, "Board {} does not have 25 squares", i)
        }

        assert_eq!(game.boards[0].sum_unmarked_numbers(), 300);
        assert_eq!(game.boards[1].sum_unmarked_numbers(), 324);
        assert_eq!(game.boards[2].sum_unmarked_numbers(), 325);

        let winner_idx = game.play();

        assert!(winner_idx.is_some(), "There must be a winner or the players get angry");

        let winning_board = &game.boards[winner_idx.unwrap()];
        assert_eq!(winning_board.sum_unmarked_numbers(), 188);
        assert_eq!(winning_board.score(game.last_called_number.unwrap()), 4512);
    }

    #[test]
    fn play_game_1_shuffled_data() {
        let mut game = super::Game::parse_game_setup(INPUT_SHUFFLED_TEST);
        let winner_idx = game.play();

        assert!(winner_idx.is_some(), "There must be a winner or the players get angry");

        let winning_board = &game.boards[winner_idx.unwrap()];
        assert_eq!(winning_board.sum_unmarked_numbers(), 188);
        assert_eq!(winning_board.score(game.last_called_number.unwrap()), 4512);
    }

    #[test]
    fn play_game_1_row_winner() {
        let mut game = super::Game::parse_game_setup(INPUT_ROW_WINNER_TEST);
        let winner_idx = game.play().unwrap();

        assert_eq!(winner_idx, 0);
    }

    #[test]
    fn play_game_1_col_winner() {
        let mut game = super::Game::parse_game_setup(INPUT_COL_WINNER_TEST);
        let winner_idx = game.play().unwrap();

        assert_eq!(winner_idx, 0);
    }
}