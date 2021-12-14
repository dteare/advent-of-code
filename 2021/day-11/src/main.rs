use std::fmt;

/// Day 11

#[derive(Clone, Copy, Debug)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

struct Puzzle {
    flash_count: usize,
    step: usize,
    consortium: Vec<Vec<Octopus>>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        println!("Parsing <{}>", input);
        let mut consortium: Vec<Vec<Octopus>> = Vec::new();

        for (_i, line_str) in input.split("\n").enumerate() {
            let trimmed = line_str.trim();
            if trimmed.len() == 0 {
                continue;
            }

            let mut row: Vec<Octopus> = Vec::new();
            for (_j, char) in trimmed.chars().enumerate() {
                let energy: u32 = char.to_digit(10).unwrap();
                row.push(Octopus {
                    energy,
                    flashed: false,
                });
            }
            consortium.push(row);
        }

        Puzzle {
            flash_count: 0,
            step: 0,
            consortium,
        }
    }

    // Run the flash simulation n times.
    fn step(&mut self, n: usize) {
        let height = self.consortium.len();
        let width = self.consortium[0].len();

        for _ in 0..n {
            for line in self.consortium.iter_mut() {
                for octopus in line.iter_mut() {
                    octopus.energy += 1;
                }
            }

            let mut flashed = false;
            loop {
                // println!("Looping for flashes");
                for i in 0..height {
                    for j in 0..width {
                        let mut octopus = self.consortium[i][j];
                        if octopus.energy > 9 && !octopus.flashed {
                            // println!("   flash triggered!");
                            flashed = true;
                            octopus.flashed = true;
                            self.flash_count += 1;
                            self.increment_adjacent(i, j);
                            self.consortium[i][j] = octopus;
                        }
                    }
                }
                // println!("Finished scan. Was there a flash triggered? {}", flashed);
                if !flashed {
                    break;
                }
                flashed = false;
            }

            for line in self.consortium.iter_mut() {
                for octopus in line.iter_mut() {
                    if octopus.energy > 9 {
                        octopus.energy = 0;
                        octopus.flashed = false;
                    }
                }
            }
        }

        self.step += n;
    }

    fn increment_adjacent(&mut self, row: usize, col: usize) {
        let height = self.consortium.len();
        let width = self.consortium[0].len();

        // println!("Incrementing adjacent for ({},{})", row, col);

        // Above
        if row > 0 {
            self.consortium[row - 1][col].energy += 1;
        }

        // Above right
        if row > 0 && col < width - 1 {
            // println!("   Incremented ({},{}) – above right", row-1, col+1);
            self.consortium[row - 1][col + 1].energy += 1;
        }

        // Right
        if col < width - 1 {
            self.consortium[row][col + 1].energy += 1;
        }

        // Below right
        if row < height - 1 && col < width - 1 {
            self.consortium[row + 1][col + 1].energy += 1;
        }

        // Below
        if row < height - 1 {
            self.consortium[row + 1][col].energy += 1;
        }

        // Below left
        if row < height - 1 && col > 0 {
            self.consortium[row + 1][col - 1].energy += 1;
        }

        // Left
        if col > 0 {
            self.consortium[row][col - 1].energy += 1;
        }

        // Above left
        if row > 0 && col > 0 {
            self.consortium[row - 1][col - 1].energy += 1;
        }
        // println!("({},{}) done!", row, col);
    }

    fn part_1(&mut self) -> usize {
        self.step(100);
        self.flash_count
    }

    fn part_2(&mut self) -> usize {
        loop {
            self.step(1);

            let mut simultaneous_flash_occurred = true;
            for line in self.consortium.iter_mut() {
                for octopus in line.iter_mut() {
                    if octopus.energy != 0 {
                        simultaneous_flash_occurred = false;
                    }
                }
            }

            if simultaneous_flash_occurred {
                break;
            }
        }
        self.step
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = "".to_string();

        for (_i, line) in self.consortium.iter().enumerate() {
            for (_j, octopus) in line.iter().enumerate() {
                display.push_str(octopus.energy.to_string().as_str());
            }
            display.push_str("\n");
        }

        write!(f, "{}", display)
    }
}

pub fn read_stdin() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), std::io::Error> {
    let mut puzzle = Puzzle::parse(&read_stdin()?);

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const SAMPLE: &str = r#"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "#;

    #[allow(dead_code)] // wtf?
    fn assert_eq_energy_levels(puzzle: &Puzzle, energy_map: &str) {
        println!("@assert_eq_energy_levels step #{}", puzzle.step);
        let expected_puzzle = super::Puzzle::parse(energy_map);

        let height = puzzle.consortium.len();
        let width = puzzle.consortium[0].len();


        for i in 0..height {
            for j in 0..width {
                let actual = puzzle.consortium[i][j];
                let expected = expected_puzzle.consortium[i][j];

                if actual.energy != expected.energy {
                    println!("💥 Disruptance in the (energy) force after step {}:\n{}", puzzle.step, puzzle);
                    assert!(false, 
                        "Consortium energy level mismatch at ({},{}) at step #{}. Expected {}; was {}.",
                        i, j, puzzle.step, expected.energy, actual.energy
                    );
                }
            }
        }
    }

    #[test]
    fn tiny_5x5() {
        let mut puzzle = super::Puzzle::parse(
            r#"
        11111
        19991
        19191
        19991
        11111
    "#,
        );

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            34543
            40004
            50005
            40004
            34543
        "#,
        );

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            45654
            51115
            61116
            51115
            45654
        "#,
        );
    }

    #[test]
    fn part_1() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(1656, puzzle.part_1());
    }

    #[test]
    fn part_1_baby_steps() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637
       "#,
        );

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848
        "#,
        );

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            0050900866
            8500800575
            9900000039
            9700000041
            9935080063
            7712300000
            7911250009
            2211130000
            0421125000
            0021119000
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            2263031977
            0923031697
            0032221150
            0041111163
            0076191174
            0053411122
            0042361120
            5532241122
            1532247211
            1132230211
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            4484144000
            2044144000
            2253333493
            1152333274
            1187303285
            1164633233
            1153472231
            6643352233
            2643358322
            2243341322
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            5595255111
            3155255222
            3364444605
            2263444496
            2298414396
            2275744344
            2264583342
            7754463344
            3754469433
            3354452433
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            6707366222
            4377366333
            4475555827
            3496655709
            3500625609
            3509955566
            3486694453
            8865585555
            4865580644
            4465574644
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            7818477333
            5488477444
            5697666949
            4608766830
            4734946730
            4740097688
            6900007564
            0000009666
            8000004755
            6800007755
            
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            9060000644
            7800000976
            6900000080
            5840000082
            5858000093
            6962400000
            8021250009
            2221130009
            9111128097
            7911119976
        "#,
        );
        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            0481112976
            0031112009
            0041112504
            0081111406
            0099111306
            0093511233
            0442361130
            5532252350
            0532250600
            0032240000
        "#,
        );

        assert_eq!(204, puzzle.flash_count);

        
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            3936556452
            5686556806
            4496555690
            4448655580
            4456865570
            5680086577
            7000009896
            0000000344
            6000000364
            4600009543
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            0643334118
            4253334611
            3374333458
            2225333337
            2229333338
            2276733333
            2754574565
            5544458511
            9444447111
            7944446119
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            6211111981
            0421111119
            0042111115
            0003111115
            0003111116
            0065611111
            0532351111
            3322234597
            2222222976
            2222222762
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            9655556447
            4865556805
            4486555690
            4458655580
            4574865570
            5700086566
            6000009887
            8000000533
            6800000633
            5680000538
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            2533334200
            2743334640
            2264333458
            2225333337
            2225333338
            2287833333
            3854573455
            1854458611
            1175447111
            1115446111
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            8211111164
            0421111166
            0042111114
            0004211115
            0000211116
            0065611111
            0532351111
            7322235117
            5722223475
            4572222754
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            1755555697
            5965555609
            4486555680
            4458655580
            4570865570
            5700086566
            7000008666
            0000000990
            0000000800
            0000000000
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            7433333522
            2643333522
            2264333458
            2226433337
            2222433338
            2287833333
            2854573333
            4854458333
            3387779333
            3333333333
        "#,
        );
        puzzle.step(10);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            0397666866
            0749766918
            0053976933
            0004297822
            0004229892
            0053222877
            0532222966
            9322228966
            7922286866
            6789998766
        "#,
        );
        assert_eq!(1656, puzzle.flash_count);
    }

    #[test]
    fn part_2() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);
        assert_eq!(195, puzzle.part_2());
    }

    #[test]
    fn part_2_validation_of_simultaneous_flash() {
        let mut puzzle = super::Puzzle::parse(SAMPLE);

        puzzle.step(193);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            5877777777
            8877777777
            7777777777
            7777777777
            7777777777
            7777777777
            7777777777
            7777777777
            7777777777
            7777777777
       "#,
        );

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            6988888888
            9988888888
            8888888888
            8888888888
            8888888888
            8888888888
            8888888888
            8888888888
            8888888888
            8888888888
       "#,
        );

        puzzle.step(1);
        assert_eq_energy_levels(
            &puzzle,
            r#"
            0000000000
            0000000000
            0000000000
            0000000000
            0000000000
            0000000000
            0000000000
            0000000000
            0000000000
            0000000000
       "#,
        );
    }
}
