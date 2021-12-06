use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 5! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

#[derive (Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive (Debug, Clone, Copy)]
struct VentLine {
    start: Point,
    end: Point,
}

#[derive (Debug)]
struct OceanFloor {
    vents: Vec<VentLine>,
    vent_layout: Vec<Vec<usize>>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for VentLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} -> {}", self.start, self.end)
    }
}

impl std::fmt::Display for OceanFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vents:\n", self.vents.len());
        
        let max_x = self.vent_layout.len();
        let max_y = self.vent_layout[0].len();

        for y in 0 .. max_y {
            for x in 0 .. max_x {
                write!(f, "{:3} ", self.vent_layout[x][y]);
            }
            write!(f, "\n");
        }
        write!(f, "\n")
      }
}

impl Point {
    fn parse(serialized_point: &str) -> Point {
        let parts:Vec<&str> = serialized_point.split(",").collect();

        assert_eq!(2, parts.len(), "Bad input - each point must consist of exactly two points: <{}>", serialized_point);

        let x = parts[0].parse::<usize>().unwrap();
        let y = parts[1].parse::<usize>().unwrap();

        Point{x, y}
    }
}

impl VentLine {
    fn parse(serialized_vent_line: &str) -> VentLine {
        let parts:Vec<&str> = serialized_vent_line.split(" -> ").collect();

        assert_eq!(2, parts.len(), "Bad input: each line must consist of exactly two points");

        let start = Point::parse(parts[0]);
        let end = Point::parse(parts[1]);

        VentLine{start, end}
    }
}

impl OceanFloor {
    fn parse(serialized_vents: &str) -> OceanFloor {
        let vent_lines: Vec<&str> = serialized_vents.split('\n').collect();
        println!("Parsing {} serialized vents", vent_lines.len());

        let mut vents = Vec::new();
        for (i, vent_line) in vent_lines.iter().enumerate() {
            print!("   loading vent {}: {} => ", i, vent_line);
            let vent = VentLine::parse(vent_line);
            println!("{}", vent);
            vents.push(vent);
        }

        let vent_layout = Vec::new();
        OceanFloor{vents, vent_layout}
    }

    fn init_layout(&mut self) {
        // Setup vent_layout with the appropriate dimensions 
        // (2d arrays with dynamic sizes in Rust eludes me)
        let mut max_x = 0;
        let mut max_y = 0;
        for vent_line in self.vents.iter() {
            if vent_line.start.x > max_x {
                max_x = vent_line.start.x;
            }
            if vent_line.start.y > max_y {
                max_y = vent_line.start.y;
            }

            if vent_line.end.x > max_x {
                max_x = vent_line.end.x;
            }
            if vent_line.end.y > max_y {
                max_y = vent_line.end.y;
            }
        }

        max_x += 1; // We're 0 based
        max_y += 1;
        println!("   allocating set of 2d vecs of {} x {}", max_x, max_y);
        self.vent_layout = Vec::with_capacity(max_x);
        self.vent_layout.resize(max_x, vec![0; max_y]);
        // println!("{}", self);
    }

    fn walk_line(&mut self, vent_line: VentLine, allow_diagonals: bool) {
        if vent_line.start.x == vent_line.end.x {
            let x = vent_line.start.x;
            let mut y_start = vent_line.start.y;
            let mut y_end = vent_line.end.y;

            if y_start > y_end {
                // Flip 'em
                y_start = vent_line.end.y;
                y_end = vent_line.start.y;
            }

            for y in y_start .. y_end + 1 { // +1 to make it inclusive. i.e. "<="
                let current = self.vent_layout[x][y];
                self.vent_layout[x][y] = current + 1;
            }
            println!("      ✅ walked vertical {}", vent_line);
        }
        else if vent_line.start.y == vent_line.end.y {
            let y = vent_line.start.y;
            let mut x_start = vent_line.start.x;
            let mut x_end = vent_line.end.x;

            if x_start > x_end {
                // Flip 'em
                x_start = vent_line.end.x;
                x_end = vent_line.start.x;
            }

            for x in x_start .. x_end + 1 { // +1 to make it inclusive. i.e. "<="
                let current = self.vent_layout[x][y];
                self.vent_layout[x][y] = current + 1;
            }
            println!("      ✅ walked horizontal vent {}", vent_line);
        }
        else {
            if allow_diagonals == false {
                println!("      💥 skipping diagonal for vent {}", vent_line);
                return;
            }

            println!("      🚶🏻‍♀️ walking the diagonal");
        }
    }

    fn navigate_horiz_and_vert_vents_to_complete_layout(&mut self) {
        println!("@navigate_horiz_and_vert_vents_to_complete_layout");
        self.init_layout();

        for (_i, vent_line) in self.vents.iter_mut().enumerate() {
            println!("   walking the line {}", vent_line);
            // Would love this but Rust is angry:
            //    cannot borrow `*self` as mutable more than once at a time
            // self.walk_line(*vent_line, false);

            // TODO -- use walk_line instead when possible
            if vent_line.start.x == vent_line.end.x {
                let x = vent_line.start.x;
                let mut y_start = vent_line.start.y;
                let mut y_end = vent_line.end.y;
    
                if y_start > y_end {
                    // Flip 'em
                    y_start = vent_line.end.y;
                    y_end = vent_line.start.y;
                }
    
                for y in y_start .. y_end + 1 { // +1 to make it inclusive. i.e. "<="
                    let current = self.vent_layout[x][y];
                    self.vent_layout[x][y] = current + 1;
                }
                println!("      ✅ walked vertical {}", vent_line);
            }
            else if vent_line.start.y == vent_line.end.y {
                let y = vent_line.start.y;
                let mut x_start = vent_line.start.x;
                let mut x_end = vent_line.end.x;
    
                if x_start > x_end {
                    // Flip 'em
                    x_start = vent_line.end.x;
                    x_end = vent_line.start.x;
                }
    
                for x in x_start .. x_end + 1 { // +1 to make it inclusive. i.e. "<="
                    let current = self.vent_layout[x][y];
                    self.vent_layout[x][y] = current + 1;
                }
                println!("      ✅ walked horizontal vent {}", vent_line);
            }
            else {
                println!("      💥 skipping diagonal for vent {}", vent_line);
            }
    

            // println!("Layout after walking line #{} {}:\n{}", i, vent_line, self);
        }

        // println!("Layout after walking horiz&vert vent lines:\n{}", self);
    }

    fn navigate_all_vents_to_complete_layout(&mut self) {
        println!("@navigate_all_vents_to_complete_layout");
        self.init_layout();

        for (_i, vent_line) in self.vents.iter().enumerate() {
            // Would love this but Rust is angry:
            //    cannot borrow `*self` as mutable more than once at a time
            // self.walk_line(*vent_line, true);

            // TODO -- use walk_line instead when possible
            if vent_line.start.x == vent_line.end.x {
                let x = vent_line.start.x;
                let mut y_start = vent_line.start.y;
                let mut y_end = vent_line.end.y;
    
                if y_start > y_end {
                    // Flip 'em
                    y_start = vent_line.end.y;
                    y_end = vent_line.start.y;
                }
    
                for y in y_start .. y_end + 1 { // +1 to make it inclusive. i.e. "<="
                    let current = self.vent_layout[x][y];
                    self.vent_layout[x][y] = current + 1;
                }
                println!("      ✅ walked vertical {}", vent_line);
            }
            else if vent_line.start.y == vent_line.end.y {
                let y = vent_line.start.y;
                let mut x_start = vent_line.start.x;
                let mut x_end = vent_line.end.x;
    
                if x_start > x_end {
                    // Flip 'em
                    x_start = vent_line.end.x;
                    x_end = vent_line.start.x;
                }
    
                for x in x_start .. x_end + 1 { // +1 to make it inclusive. i.e. "<="
                    let current = self.vent_layout[x][y];
                    self.vent_layout[x][y] = current + 1;
                }
                println!("      ✅ walked horizontal vent {}", vent_line);
            }
            else {
                // First set things up so we're always going left to right
                let mut start = vent_line.start;
                let mut end = vent_line.end;
                if start.x > end.x {
                    println!("FLIPPING!");
                    start = vent_line.end;
                    end = vent_line.start;
                }

                println!("      🚶🏻‍♀️ walking diagonal for line {} from {} to {}", vent_line, start.x, end.x);
                let mut slope:i32 = 1;
                if start.y > end.y {
                    slope = -1;
                }

                let mut y = start.y;
                for x in start.x .. end.x { // exclusive; we'll catch it at the end
                    println!("        @x={}, @y={}",x, y);
                    self.vent_layout[x][y] += 1;
                    
                    if slope == 1 {
                        y += 1;
                    }
                    else {
                        y -= 1;
                    }
                }
                self.vent_layout[end.x][end.y] += 1;
            }

            // println!("Layout after walking line #{} {}:\n{}", i, vent_line, self);
        }

        // println!("Layout after walking horiz&vert vent lines:\n{}", self);
    }


    fn count_danger_areas(&self) -> usize {
        println!("@count_danger_areas");
        let mut r = 0;
        let threshold: usize = 1;
        for lines in self.vent_layout.iter() {
            for c in lines.iter() {
                if *c > threshold {
                    r += 1;
                }
            }
        }
        r
    }
}

fn load_ocean_floor_from_file(file_name: &str) -> OceanFloor {
    //  let input = File::open(file_name).expect("can't open {}", file_name);
    //  io::BufReader::new(input).lines().enumerate()
    let mut input = File::open(file_name).unwrap();
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).unwrap();
    OceanFloor::parse(input_buffer.as_str())
}

fn star1() -> std::io::Result<()> {
    let mut ocean_floor = load_ocean_floor_from_file("../hydrothermal-vents.txt");

    println!("Ocean floor has {} vents", ocean_floor.vents.len());

    ocean_floor.navigate_horiz_and_vert_vents_to_complete_layout();

    let danger_count = ocean_floor.count_danger_areas();

    println!("⭐️ Analysis:");
    println!("   Vents: {}", ocean_floor.vents.len());
    println!("   Danger areas: {}", danger_count);

    Ok(())
}

fn star2() -> std::io::Result<()> {
    let mut ocean_floor = load_ocean_floor_from_file("../hydrothermal-vents.txt");

    println!("Ocean floor has {} vents", ocean_floor.vents.len());

    ocean_floor.navigate_all_vents_to_complete_layout();

    let danger_count = ocean_floor.count_danger_areas();

    println!("⭐️⭐️ Analysis:");
    println!("   Vents: {}", ocean_floor.vents.len());
    println!("   Danger areas: {}", danger_count);

    Ok(())
}

#[cfg(test)]
mod test {
    const INPUT_SAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    const INPUT_PARTIAL_OFFICIAL: &str = r#"456,846 -> 221,846
980,926 -> 73,19
682,930 -> 562,930
766,592 -> 274,100
247,685 -> 247,21"#;

   #[test]
    fn first_star_official_sample() {
        let mut ocean_floor = super::OceanFloor::parse(INPUT_SAMPLE);

        assert_eq!(10, ocean_floor.vents.len());

        ocean_floor.navigate_horiz_and_vert_vents_to_complete_layout();

        assert_eq!(5, ocean_floor.count_danger_areas());
    }

    #[test]
    fn first_star_first_part_of_official() {
        let mut ocean_floor = super::OceanFloor::parse(INPUT_PARTIAL_OFFICIAL);

        assert_eq!(5, ocean_floor.vents.len());

        ocean_floor.navigate_horiz_and_vert_vents_to_complete_layout();

        assert_eq!(0, ocean_floor.count_danger_areas());
    }

    #[test]
    fn second_star_given_sample() {
        let mut ocean_floor = super::OceanFloor::parse(INPUT_SAMPLE);

        assert_eq!(10, ocean_floor.vents.len());

        ocean_floor.navigate_all_vents_to_complete_layout();

        assert_eq!(12, ocean_floor.count_danger_areas());
    }
}