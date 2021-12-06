use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 3! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

fn star1() -> std::io::Result<()> {
    let mut input = File::open("../diagnostics.txt")?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    let lines: Vec<&str> = input_buffer.split('\n').collect();

    // let values = lines.iter().map(|l| usize::from_str_radix(l, 2).unwrap() ).collect::<Vec<usize>>();

    println!("#of lines: {}", lines.len());

    let column_count = lines[0].len();
    let mut gamma_vec = Vec::new();
    let mut epsilon_vec = Vec::new();
    for column in 0..column_count {
        let mut zero_count = 0;
        let mut one_count = 0;
        for (i, line) in lines.iter().enumerate() {

            assert_eq!(line.len(), column_count, "Each input line must have {} digits since that's what the first line had. Line {} violated this: <{}>", column_count, i, line);

            // Rust is rightfully fighting me every step of the way as it wants me doing unicode
            match line.chars().collect::<Vec<char>>()[column].to_string().as_str() {
                "0" => zero_count += 1,
                "1" => one_count += 1,
                _ => panic!("Bad character at column {} of line {}: {}", column, i, line),
            }
        }

        assert_eq!(zero_count+one_count, lines.len());

        println!("    column {} had {} zeros and {} ones", column, zero_count, one_count);

        if zero_count > one_count {
            gamma_vec.push("0");
            epsilon_vec.push("1");
        }
        else if zero_count < one_count {
            gamma_vec.push("1");
            epsilon_vec.push("0");
        }
        else {
            panic!("Counts are equal. Undefined expectations here.");
        }

        println!("      -> Gamma is now {}", gamma_vec.join(""));

    }
    

    let gamma_rate = usize::from_str_radix(gamma_vec.join("").as_str(), 2).unwrap();
    println!("Parsed gamma vec {} as {} (decimal)", gamma_vec.join(""), gamma_rate);

    let epsilon_rate = usize::from_str_radix(epsilon_vec.join("").as_str(), 2).unwrap();
    println!("Parsed gamma vec {} as {} (decimal)", epsilon_vec.join(""), epsilon_rate);

    // This would have been perfect and was super nice when we knew the number of columns a priori. How can I make this dynamic?
    // Advice from Future Dave: So, remember (2^n - 1) in decimal maps to n set bits in binary. You could calculate the number of bits (n) earlier on and here performed `!gamma_rate & (2^n-1)`
    // let epsilon_rate = !(gamma_rate) & 0b11111;


    let power_consumption = gamma_rate * epsilon_rate;

    println!("⭐️ Analysis:");
    println!("   Gamma rate   {:#09b}, decimal: {:#7}", gamma_rate, gamma_rate);
    println!("   Epsilon rate {:#09b}, decimal: {:#7}", epsilon_rate, epsilon_rate);
    println!("   Power consumption = {}", power_consumption);

    Ok(())
}

fn star2() -> std::io::Result<()> {
    Ok(())
}
