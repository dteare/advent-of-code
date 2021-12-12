use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Advent of Code day 9! 🙌");

    star1()?;
    println!("\n\n\n---------------------------------------------------\n\n");
    star2()?;
    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    height: usize,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Heightmap {
    cells: Vec<Vec<Cell>>,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "({},{})={}", self.y, self.x, self.height)
    }
}

impl Cell {
    fn risk_level(&self) -> usize {
        self.height + 1
    }

    // mark this cell as non-navigatible for future walkers
    // This method makes assumptions and should be removed since the walking table could be modelled with optionals
    fn mark_as_non_walkable(&mut self) {
        self.height = 9;
    }
}

impl Heightmap {
    fn parse(input: &str) -> Heightmap {
        println!("Parsing <{}>", input);
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for (y, line_str) in input.split("\n").enumerate() {
            let mut line:Vec<Cell> = Vec::new();

            for (x, c) in line_str.chars().enumerate() {
                let u32 = c.to_digit(10).unwrap();
                let height:usize = u32.try_into().unwrap();
                line.push(Cell { height, y, x })
            }

            cells.push(line);
        }

        Heightmap { cells }
    }

    fn adjacent_sum(&self, y:usize, x:usize) -> usize {
        let mut sum = 0;

        if y > 0 {
            sum += self.cells[y-1][x].height;
        }
        if x > 0 {
            sum += self.cells[y][x-1].height;
        }

        if y < self.cells.len() - 1 {
            sum += self.cells[y+1][x].height;
        }
        if x < self.cells[0].len() - 1 {
            sum += self.cells[y][x+1].height;
        }

        sum
    }

    // A cell is the low point if its height is *lower* than any of its adjacent locations. Note the *lower* and not *less than or equal*. 
    fn is_low_point(&self, y:usize, x:usize) -> bool {
        let me = self.cells[y][x].height;
        if y > 0 && self.cells[y-1][x].height <= me {
            return false;
        }
        if x > 0 && self.cells[y][x-1].height <= me {
            return false;
        }

        if y < self.cells.len() - 1 && self.cells[y+1][x].height <= me {
            return false;
        }
        if x < self.cells[0].len() - 1 && self.cells[y][x+1].height <= me {
            return false;
        }

        true
    }

    fn low_points(&self) -> Vec<Cell> {
        let mut result:Vec<Cell> = Vec::new();

        println!("@low_points");
        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                let is_low = self.is_low_point(y, x);
                // println!("   ({}, {}) is low? {}", y, x, is_low);

                if is_low {
                    result.push(self.cells[y][x].clone());
                }
            }
        }

        result
    }

    // The size of the basin (not the sum of the cells within it)
    fn basin_size_for_low_point(&self, c:Cell) -> usize {
        let mut remaining_cells = self.cells.clone();
        self.basin_explorer(c.y, c.x, &mut remaining_cells)
    }

    fn product_of_3_largest_basins(&self, low_points: &Vec<Cell>) -> usize {
        let mut basin_sizes:Vec<usize> = Vec::new();
        
        for cell in low_points.iter() {
            basin_sizes.push(self.basin_size_for_low_point(*cell));
        }

        basin_sizes.sort_by(|a, b| b.cmp(a));

        println!("Sorted basin sizes: {:?}", basin_sizes);

        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    }

    // Walk through the (remaining) cells in the basin, count what can be navigated to, and return the size 
    fn basin_explorer(&self, y:usize, x:usize, remaining_cells: & mut Vec<Vec<Cell>>) -> usize {
        let width = self.cells[0].len();
        let height = self.cells.len();

        if remaining_cells[y][x].height == 9 {
            return 0
        }

        let mut count = 1; // my cell plus any adjacent ones we'll count below
        remaining_cells[y][x].mark_as_non_walkable();

        if y > 0 {
            count += self.basin_explorer(y-1, x, remaining_cells);
        }
        if x > 0 {
            count += self.basin_explorer(y, x-1, remaining_cells);
        }

        if y < height - 1 {
            count += self.basin_explorer(y+1, x, remaining_cells);
        }
        if x < width - 1 {
            count += self.basin_explorer(y, x+1, remaining_cells);
        }

        count 
    }
}

fn load_from_file(file_name: &str) -> Heightmap {
    //  let input = File::open(file_name).expect("can't open {}", file_name);
    //  io::BufReader::new(input).lines().enumerate()
    let mut input = File::open(file_name).unwrap();
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer).unwrap();
    Heightmap::parse(input_buffer.as_str())
}

fn star1() -> std::io::Result<()> {
    let heightmap = load_from_file("../heightmap");
    let low_points = heightmap.low_points();

    let risk:usize = low_points.iter().map(|c| c.risk_level()).sum();

    println!("⭐️ Analysis:");
    println!("   Heightmap size: {}x{}", heightmap.cells.len(), heightmap.cells[0].len());
    println!("   Low points: {}", low_points.len());
    println!("   Risk: {}", risk);

    Ok(())
}

fn star2() -> std::io::Result<()> {
    let heightmap = load_from_file("../heightmap");
    let low_points:Vec<Cell> = heightmap.low_points();

    println!("Low points: {:?}", low_points);

    let answer = heightmap.product_of_3_largest_basins(&low_points);

    println!("⭐️ Analysis:");
    println!("   Heightmap size: {}x{}", heightmap.cells.len(), heightmap.cells[0].len());
    println!("   Low points: {}", low_points.len());
    println!("   Product of 3 largest basins: {}", answer);

    Ok(())
}

#[cfg(test)]
mod test {
    const GIVEN_EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn part_1() {
        let heightmap = super::Heightmap::parse(GIVEN_EXAMPLE);

        // Loaded everything?
        assert_eq!(5, heightmap.cells.len());
        assert_eq!(10, heightmap.cells[0].len());

        // Does our coordinate mental model match up?
        assert_eq!(2, heightmap.cells[0][0].height);
        assert_eq!(0, heightmap.cells[0][9].height);
        assert_eq!(9, heightmap.cells[4][0].height);
        assert_eq!(8, heightmap.cells[4][9].height);

        // Test adjacent calculations (unneeded in part 1 but ensures sanity)
        assert_eq!(4, heightmap.adjacent_sum(0, 0));
        assert_eq!(2, heightmap.adjacent_sum(0, 9));
        assert_eq!(16, heightmap.adjacent_sum(4, 0));
        assert_eq!(16, heightmap.adjacent_sum(4, 9));
        assert_eq!(34, heightmap.adjacent_sum(2, 5));

        // Individual low points
        assert_eq!(false, heightmap.is_low_point(0, 0));
        assert_eq!(true, heightmap.is_low_point(0, 1));
        assert_eq!(false, heightmap.is_low_point(0, 8));
        assert_eq!(true, heightmap.is_low_point(0, 9));
        assert_eq!(false, heightmap.is_low_point(4, 0));
        assert_eq!(true, heightmap.is_low_point(4, 6));
        assert_eq!(false, heightmap.is_low_point(4, 9));

        // Aggregate low points
        let low_points = heightmap.low_points();
        println!("Low points: {:?}", low_points);

        assert_eq!(4, low_points.len());


        // Overall risk (final answer)
        let risk:usize = low_points.iter().map(|c| c.risk_level()).sum();
        assert_eq!(15, risk);
    }

    const REAL_DATA_FIRST_LINE: &str = r#"8654434789432446987654321056789235678953245798764212456789656568977654232457898754567898765431335999"#;
        
    #[test]
    fn p1_with_real_data_first_line() {
        let heightmap = super::Heightmap::parse(REAL_DATA_FIRST_LINE);

        // Loaded everything?
        assert_eq!(1, heightmap.cells.len());
        assert_eq!(100, heightmap.cells[0].len());

        // Does our coordinate mental model match up?
        assert_eq!(8, heightmap.cells[0][0].height);
        assert_eq!(6, heightmap.cells[0][1].height);
        assert_eq!(9, heightmap.cells[0][99].height);

        // Test adjacent calculations (unneeded in part 1 but ensures sanity)
        assert_eq!(6, heightmap.adjacent_sum(0, 0));
        assert_eq!(13, heightmap.adjacent_sum(0, 1));
        assert_eq!(10, heightmap.adjacent_sum(0, 2));
        assert_eq!(18, heightmap.adjacent_sum(0, 98));
        assert_eq!(9, heightmap.adjacent_sum(0, 99));

        // Individual low points
        assert_eq!(true, heightmap.is_low_point(0, 5));

        // Aggregate low points
        let low_points = heightmap.low_points();
        println!("Low points:\n{:?}", low_points);

        // // Overall risk (final answer)
        let risk:usize = low_points.iter().map(|c| c.risk_level()).sum();
        println!("Aggregate risk level: {}", risk);
    }

    const REAL_DATA_SAMPLE: &str = r#"8654434789432446987654321056789235678953245798764212456789656568977654232457898754567898765431335999
8743125678931234599987643238993123567894345989542101234678943459865432101345789643479999878540129877
9651056799320125678998764369895644678965956976543233458799202568986574519499996532445989998761298956
6543234789632347899219865456789795789879899898754755567893212456998986798987897621234678979872987645
7887645696543456799423989978998989893989767789865676679995343689879497986596789432349899865989876434
8998967789999577897999799999457678902497645699876799798789459798767329876455678943456798764699987569
9999878899878988956986678953234567893989534578997998987689567976543213984323589894967899643209898978
0987999999767899549875439870146789999875323466899886596578979987954201987312398789898978965799769989
2996567899656795423964321981269893987983212345689765434499898799865332975201297598789467979987654292
9875456998945789019875992984378942496543201456789897521345679689965449964319976465694356797998893101"#;
        
    #[test]
    fn p1_with_more_real_data() {
        let heightmap = super::Heightmap::parse(REAL_DATA_SAMPLE);

        // Loaded everything?
        assert_eq!(10, heightmap.cells.len());
        assert_eq!(100, heightmap.cells[0].len());

        // Does our coordinate mental model match up?
        assert_eq!(8, heightmap.cells[0][0].height);
        assert_eq!(9, heightmap.cells[0][99].height);
        assert_eq!(9, heightmap.cells[2][0].height);
        assert_eq!(6, heightmap.cells[2][99].height);
        assert_eq!(9, heightmap.cells[9][7].height);

        // Test adjacent calculations (unneeded in part 1 but ensures sanity)
        assert_eq!(14, heightmap.adjacent_sum(0, 0));
        assert_eq!(16, heightmap.adjacent_sum(0, 99));
        assert_eq!(10, heightmap.adjacent_sum(9, 0));
        assert_eq!(2, heightmap.adjacent_sum(9, 99));
        assert_eq!(11, heightmap.adjacent_sum(2, 5));
        assert_eq!(20, heightmap.adjacent_sum(7, 0));

        // Individual low points
        assert_eq!(false, heightmap.is_low_point(0, 5));
        assert_eq!(true, heightmap.is_low_point(2, 4));
        assert_eq!(true, heightmap.is_low_point(7, 0));

        // Aggregate low points
        let low_points = heightmap.low_points();
        println!("Low points:\n{:?}", low_points);

        // // Overall risk (final answer)
        let risk:usize = low_points.iter().map(|c| c.risk_level()).sum();
        println!("Aggregate risk level: {}", risk);
    }


    #[test]
    fn part_2() {
        let heightmap = super::Heightmap::parse(GIVEN_EXAMPLE);
        let low_points = heightmap.low_points();

        assert_eq!(4, low_points.len());
        println!("Low points: {:?}", low_points);

        // Each low point has a basin...
        assert_eq!(3, heightmap.basin_size_for_low_point(low_points[0]));
        assert_eq!(9, heightmap.basin_size_for_low_point(low_points[1]));
        assert_eq!(14, heightmap.basin_size_for_low_point(low_points[2]));
        assert_eq!(9, heightmap.basin_size_for_low_point(low_points[3]));

        assert_eq!(1134, heightmap.product_of_3_largest_basins(&low_points));
    }

    #[test]
    fn part_2_debugging_wtf() {
        const SINGLE_SMALLEST_BASIN: &str = r#"999999999
999999999
999999999
999999999
999919999
999999999
999999999
999999999
999999999"#;
        
        let mut heightmap = super::Heightmap::parse(SINGLE_SMALLEST_BASIN);
        let mut low_points = heightmap.low_points();

        assert_eq!(1, low_points.len());
        assert_eq!(1, heightmap.basin_size_for_low_point(low_points[0]));


        const HIDDEN_CREVICE: &str = r#"999999999
912992222
992992992
992992992
992992992
992992992
992922992
992999992
992222222"#;
        
        heightmap = super::Heightmap::parse(HIDDEN_CREVICE);
        low_points = heightmap.low_points();

        assert_eq!(1, low_points.len());
        assert_eq!(31, heightmap.basin_size_for_low_point(low_points[0]));


        const SINGLE_X_BASIN: &str = r#"799999997
779999977
977777779
979979979
977979779
999919999
977979779
979979979
977777779
779999977
799999997"#;
        
        heightmap = super::Heightmap::parse(SINGLE_X_BASIN);
        low_points = heightmap.low_points();

        assert_eq!(1, low_points.len());
        assert_eq!(43, heightmap.basin_size_for_low_point(low_points[0]));


        const SINGLE_X_BASIN_THAT_COULD_LOOP: &str = r#"799999997
779999977
977777779
979979979
977979779
979919979
977979779
979979979
977777779
779999977
799999997"#;
        
        heightmap = super::Heightmap::parse(SINGLE_X_BASIN_THAT_COULD_LOOP);
        low_points = heightmap.low_points();

        assert_eq!(1, low_points.len());
        assert_eq!(45, heightmap.basin_size_for_low_point(low_points[0]));
    }
}
