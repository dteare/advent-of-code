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
struct GameSetup {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl GameSetup {
    fn as_string(&self) -> String {
        format!("Numbers: {}", self.numbers.len())
    }
}

fn parse_game_setup(setup: &str) -> GameSetup {
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

    GameSetup{numbers, boards}
}

fn load_game_setup(file_name: &str) -> GameSetup {
    let mut input = File::open(file_name).unwrap();
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).unwrap();
    parse_game_setup(input_buffer.as_str())
}

fn star1() -> std::io::Result<()> {
    let game = load_game_setup("../bingo-test.txt");

    println!("Game setup: {}", game.as_string());

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
        let game = super::parse_game_setup(INPUT);

        assert_eq!(game.numbers.len(), 27);
        assert_eq!(game.boards.len(), 3);

        for (i, board) in game.boards.iter().enumerate() {
            assert_eq!(board.squares.len(), 25, "Board {} does not have 25 squares", i)
        }
    }
}