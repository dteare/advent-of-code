use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 1! 🙌");

    // star1()?;
    // star1_improved()?;

    // star2()?;
    star2_improved()?;

    Ok(())
}

fn star1() -> std::io::Result<()> {
    let mut input = File::open("../../sonar-depths-official.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    println!("Data file line count: {}", lines.len());

    let mut previous = -1;
    let mut count = 0;
    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut constant_count = 0;
    for (i, current_str) in lines.iter().enumerate() {
        match i {
            0 => {
                previous = current_str.parse::<i32>().unwrap();
            },
            _ => {
                let current = current_str.parse::<i32>().unwrap();

                count = count + 1;
                if current > previous {
                    increase_count += 1;
                }
                else if current < previous {
                    decrease_count += 1;
                }
                else {
                    constant_count += 1;
                }

                // Get ready for next pass
                previous = current;
            }
        }
    }

    println!("⭐️ Analysis:");
    println!("  * Comparisons: {}", count);
    println!("  * Increases in depth: {}", increase_count);
    println!("  * Decreases in depth: {}", decrease_count);
    println!("  * Constant depth / no change: {}", constant_count);

    Ok(())
}

fn star1_improved() -> std::io::Result<()> {
    let mut input = File::open("../../sonar-depths-official.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    println!("Data file line count: {}", lines.len());

    let mut previous = lines[0].parse::<i32>().unwrap();
    let mut count = 0;
    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut constant_count = 0;
    for (i, current_str) in lines.iter().skip(1).enumerate() {
        let current = current_str.parse::<i32>().unwrap();

        count = count + 1;
        if current > previous {
            println!("Depth INCREASED ☝🏻");
            increase_count += 1;
        }
        else if current < previous {
            println!("Depth DECREASED 👇🏻");
            decrease_count += 1;
        }
        else {
            println!("Depth stayed constant 🔁");
            constant_count += 1;
        }

        // Get ready for next pass
        previous = current;
        println!("Updated previous: {}", previous)
    }

    println!("⭐️ʹ Analysis:");
    println!("  * Comparisons: {}", count);
    println!("  * Increases in depth: {}", increase_count);
    println!("  * Decreases in depth: {}", decrease_count);
    println!("  * Constant depth / no change: {}", constant_count);

    Ok(())
}


fn star2() -> std::io::Result<()> {
    let mut input = File::open("../../sonar-depths-official.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    println!("Data file line count: {}", lines.len());

    let mut previous_sum = -1;
    let mut previous = -1;
    let mut previous_previous = -1;
    let mut count = 0;
    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut constant_count = 0;
    for (i, current_str) in lines.iter().enumerate() {
        match i {
            0 => {
                previous_previous = current_str.parse::<i32>().unwrap();
            },
            1 => {
                previous = current_str.parse::<i32>().unwrap();
            },
            _ => {
                let current = current_str.parse::<i32>().unwrap();

                let sum = previous_previous + previous + current;

                if previous_sum == -1 {
                    previous_sum = sum;
                    previous_previous = previous;
                    previous = current;
                    continue;
                }

                count = count + 1;
                if sum > previous_sum {
                    increase_count += 1;
                }
                else if sum < previous_sum {
                    decrease_count += 1;
                }
                else {
                    constant_count += 1;
                }

                // Get ready for next pass
                previous_sum = sum;
                previous_previous = previous;
                previous = current;
            }
        }
    }

    println!("⭐️⭐️ Analysis:");
    println!("  * Comparisons: {}", count);
    println!("  * Increases in depth: {}", increase_count);
    println!("  * Decreases in depth: {}", decrease_count);
    println!("  * Constant depth / no change: {}", constant_count);

    Ok(())
}

fn star2_improved() -> std::io::Result<()> {
    let mut input = File::open("../../sonar-depths-official.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    println!("Data file line count: {}", lines.len());

    let depths = lines.iter().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut previous_sum = depths[0] + depths[1] + depths[2];
    let mut count = 0;
    let mut increase_count = 0;
    let mut decrease_count = 0;
    let mut constant_count = 0;
    for window in depths.windows(3).skip(1) {
        let previous_previous = window[0];
        let previous = window[1];
        let current = window[2];

        let sum = previous_previous + previous + current;
        println!("Calculated sum: {}+{}+{} = {}",previous_previous, previous, current, sum);

        count = count + 1;
        if sum > previous_sum {
            println!("Depth INCREASED ☝🏻");
            increase_count += 1;
        }
        else if sum < previous_sum {
            println!("Depth DECREASED 👇🏻");
            decrease_count += 1;
        }
        else {
            println!("Depth stayed constant 🔁");
            constant_count += 1;
        }

        // Get ready for next pass
        previous_sum = sum;
        println!("Updated (previous_sum, previous_previous, previous): ({},{},{})", previous_sum, previous_previous, previous)
    }

    println!("⭐️⭐️ʹ Analysis:");
    println!("  * Comparisons: {}", count);
    println!("  * Increases in depth: {}", increase_count);
    println!("  * Decreases in depth: {}", decrease_count);
    println!("  * Constant depth / no change: {}", constant_count);

    Ok(())
}