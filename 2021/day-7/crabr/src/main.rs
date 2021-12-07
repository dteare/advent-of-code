use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 7! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

#[derive(Debug)]
struct Crab {
    horizontal_pos: usize,
}

#[derive(Debug)]
struct Swarm {
    crabs: Vec<Crab>,
}

impl Crab {
    // Calculate how far this crab is from horizontal position p
    fn distance_from(&self, p: usize) -> usize {
        if p > self.horizontal_pos {
            p - self.horizontal_pos
        } else {
            self.horizontal_pos - p
        }
    }

    fn fuel_cost_to_move_to(&self, p:usize) -> usize {
        let distance = self.distance_from(p);
        let mut sum = 0;
        for i in 0..distance + 1 {
            sum += i;
        }

        sum
    }
}

impl Swarm {
    fn parse(input: &str) -> Swarm {
        println!("Parsing <{}>", input);
        let mut crabs = Vec::new();

        for s in input.split(",") {
            let horizontal_pos = s.parse::<usize>().unwrap();
            crabs.push(Crab { horizontal_pos })
        }

        Swarm { crabs }
    }

    fn max_hpos(&self) -> usize {
        let mut max = 0;
        for crab in self.crabs.iter() {
            if crab.horizontal_pos > max {
                max = crab.horizontal_pos;
            }
        }

        max
    }

    fn total_fuel_cost_to_position_using_constant_fuel(&self, hpos: usize) -> usize {
        let mut total = 0;
        for crab in self.crabs.iter() {
            total += crab.distance_from(hpos);
        }
        total
    }

    fn ideal_horizontal_pos_using_constant_fuel(&mut self) -> usize {
        let mut min_fuel_cost = usize::MAX;
        let mut ideal_position = usize::MAX;

        for hcol in 0..self.max_hpos() {
            let fuel = self.total_fuel_cost_to_position_using_constant_fuel(hcol);
            if fuel < min_fuel_cost {
                min_fuel_cost = fuel;
                ideal_position = hcol;
            }

            println!("   Fuel cost to {:2}: {:5}", hcol, fuel);
        }

        ideal_position
    }

    fn total_fuel_cost_to_position_using_exponential_fuel(&self, hpos: usize) -> usize {
        let mut total = 0;
        for crab in self.crabs.iter() {
            total += crab.fuel_cost_to_move_to(hpos);
        }
        total
    }

    fn ideal_horizontal_pos_using_exponential_fuel(&mut self) -> (usize, usize) {
        let mut min_fuel_cost = usize::MAX;
        let mut ideal_position = usize::MAX;

        for hcol in 0..self.max_hpos() {
            let fuel = self.total_fuel_cost_to_position_using_exponential_fuel(hcol);
            if fuel < min_fuel_cost {
                min_fuel_cost = fuel;
                ideal_position = hcol;
            }

            println!("   Fuel cost to {:2}: {:5}", hcol, fuel);
        }

        (ideal_position, min_fuel_cost)
    }
}

fn load_from_file(file_name: &str) -> Swarm {
    //  let input = File::open(file_name).expect("can't open {}", file_name);
    //  io::BufReader::new(input).lines().enumerate()
    let mut input = File::open(file_name).unwrap();
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).unwrap();
    Swarm::parse(input_buffer.as_str())
}

fn star1() -> std::io::Result<()> {
    let mut swarm = load_from_file("../crab-heroes.txt");
    let ideal = swarm.ideal_horizontal_pos_using_constant_fuel();

    println!("⭐️ Analysis:");
    println!("   {} crab heroes rescued us", swarm.crabs.len());
    println!("   Best position: {}", swarm.max_hpos());
    println!(
        "   Fuel cost to get everyone to {}: {}",
        ideal,
        swarm.total_fuel_cost_to_position_using_constant_fuel(ideal)
    );
    println!("   {} is the best position for our heroes", ideal);

    Ok(())
}

fn star2() -> std::io::Result<()> {
    let mut swarm = load_from_file("../crab-heroes.txt");
    let (ideal, fuel) = swarm.ideal_horizontal_pos_using_exponential_fuel();

    println!("⭐️⭐️ Analysis:");
    println!("   Crab hero rescuers: {}", swarm.crabs.len());
    println!("   Max position: {}", swarm.max_hpos());
    println!("   Ideal position: {}", ideal);
    println!("   Fuel cost to get to {}: {}", ideal, fuel);

    Ok(())
}

#[cfg(test)]
mod test {
    const INPUT_SAMPLE: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn part_1() {
        let mut swarm = super::Swarm::parse(INPUT_SAMPLE);

        assert_eq!(10, swarm.crabs.len());
        assert_eq!(16, swarm.max_hpos());

        assert_eq!(5, swarm.crabs[2].distance_from(7));
        assert_eq!(2, swarm.crabs[2].distance_from(0));

        assert_eq!(2, swarm.ideal_horizontal_pos_using_constant_fuel());
    }


    #[test]
    fn part_2() {
        let crab = super::Crab{horizontal_pos: 5};
        assert_eq!(0, crab.fuel_cost_to_move_to(5));

        assert_eq!(1, crab.fuel_cost_to_move_to(6));
        assert_eq!(1, crab.fuel_cost_to_move_to(4));

        assert_eq!(3, crab.fuel_cost_to_move_to(7));
        assert_eq!(3, crab.fuel_cost_to_move_to(3));

        assert_eq!(6, crab.fuel_cost_to_move_to(8));
        assert_eq!(6, crab.fuel_cost_to_move_to(2));

        assert_eq!(10, crab.fuel_cost_to_move_to(9));
        assert_eq!(10, crab.fuel_cost_to_move_to(1));

        assert_eq!(15, crab.fuel_cost_to_move_to(10));
        assert_eq!(15, crab.fuel_cost_to_move_to(0));

        let mut swarm = super::Swarm::parse(INPUT_SAMPLE);

        assert_eq!(10, swarm.crabs.len());
        assert_eq!(16, swarm.max_hpos());



        let (ideal, cost) = swarm.ideal_horizontal_pos_using_exponential_fuel();
        assert_eq!(5, ideal);
        assert_eq!(168, cost);
    }
}
