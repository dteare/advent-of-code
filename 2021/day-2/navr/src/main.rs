use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 2! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

#[derive (Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive (Debug)]
struct Nav {
    direction: Direction,
    magnitude: usize,
}

fn parse_instruction(line: &str) -> Nav {
    let mut iter = line.split_whitespace();
    let dir_str = iter.next().unwrap();
    let mag_str = iter.next().unwrap();
    assert_eq!(None, iter.next());

    let magnitude = mag_str.parse::<usize>().unwrap();
    let direction = match dir_str {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("Blimey!")
    };

    Nav{direction, magnitude}
}

fn star1() -> std::io::Result<()> {
    let mut input = File::open("../navigation.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    let instructions = lines.iter().map(|l| parse_instruction(l) ).collect::<Vec<Nav>>();

    println!("#of instructions: {}", lines.len());

    let mut horiz_position = 0;
    let mut depth = 0;
    for (i, instruction) in instructions.iter().enumerate() {
        println!("Parsed line <{}> as {:?}", lines[i], instruction);

        match instruction.direction {
            Direction::Forward => {
                horiz_position += instruction.magnitude;
            },
            Direction::Down => {
                depth += instruction.magnitude;
            },
            Direction::Up => {
                depth -= instruction.magnitude;
            },
        }

        println!("After processing instruction {} we have (postion, depth)=({}, {})", i, horiz_position, depth);
    }

    let answer = horiz_position * depth;

    println!("⭐️ Analysis:");
    println!("   The valet parked your sub at position {}, depth {}", horiz_position, depth);
    println!("   Multipled, position x depth = {}", answer);

    Ok(())
}

fn star2() -> std::io::Result<()> {
    let mut input = File::open("../navigation.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    let instructions = lines.iter().map(|l| parse_instruction(l) ).collect::<Vec<Nav>>();

    println!("#of instructions: {}", lines.len());

    let mut horiz_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (i, instruction) in instructions.iter().enumerate() {
        println!("Parsed line <{}> as {:?}", lines[i], instruction);

        match instruction.direction {
            Direction::Forward => {
                horiz_position += instruction.magnitude;
                depth += aim * instruction.magnitude;
            },
            Direction::Down => {
                aim += instruction.magnitude;
            },
            Direction::Up => {
                aim -= instruction.magnitude;
            },
        }

        println!("After processing instruction {} we have (postion, depth, aim)=({}, {}, {})", i, horiz_position, depth, aim);
    }

    let answer = horiz_position * depth;

    println!("⭐️⭐️ Analysis:");
    println!("   The valet parked your sub at position {}, depth {}", horiz_position, depth);
    println!("   Multipled, position x depth = {}", answer);

    Ok(())
}
