/// Day 10

#[derive(Debug, PartialEq)]
enum NavSubsystemLineSyntax {
    Valid,
    Incomplete,
    Corrupted(usize),
}

fn parse_nav_subsystem_line(input: &str) -> NavSubsystemLineSyntax {
    let mut opening_chunks: Vec<char> = Vec::new();

    let openers = "([{<";
    let closers = ")]}>";

    let opener_for = std::collections::HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    let corruption_score = std::collections::HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    println!("🔎: {}", input);
    for (i, c) in input.chars().enumerate() {

        if openers.contains(c) {
            opening_chunks.push(c);
        }
        else if closers.contains(c) {
            match opener_for.get(&c) {
                Some(opener) => {
                    println!("Looking for {} to match closing {}", opener, c);
                    let last = opening_chunks.pop();

                    match last {
                        Some(last) => {
                            if last == *opener {
                                // All good!
                            }
                            else {
                                println!("Encountered unexpected closing {} when current chunk was opened with {}.", last, opener);
                                println!("Looking up {} in corruption score hash map: {:?}", c, corruption_score);
                                return NavSubsystemLineSyntax::Corrupted(*corruption_score.get(&c).unwrap());
                            }
                        },
                        None => {
                            println!("Encountered closing {} when no chunk opener was available.", c);
                            return NavSubsystemLineSyntax::Corrupted(*corruption_score.get(&c).unwrap()); 
                        }
                    }
                },
                None => {
                    panic!("No opener defined for {}", c);
                }
            }
        }
        else {
            panic!("Unexpected character \"{}\" provided by nav subsystem at index {}: {}", c, i, input);
        }
    }

    if opening_chunks.len() == 0 {
        NavSubsystemLineSyntax::Valid
    }
    else {
        NavSubsystemLineSyntax::Incomplete
    }
}
struct Puzzle {
    lines: Vec<String>,
}

impl Puzzle {
    fn parse(input: &str) -> Puzzle {
        println!("Parsing <{}>", input);
        let mut lines: Vec<String> = Vec::new();

        for (_i, line_str) in input.split("\n").enumerate() {
            lines.push(line_str.to_string());
        }

        Puzzle{ lines }
    }

    fn part_1(&self) -> usize {
        let mut result:usize = 0;
        for (i, line) in self.lines.iter().enumerate() {
            let syntax = parse_nav_subsystem_line(line.as_str());

            match syntax {
                NavSubsystemLineSyntax::Corrupted(score) => {
                    println!("Line {} is corrupted with a score of {}", i, score);
                    result += score;
                },
                _ => {}
            }
        }

        result
    }
}

pub fn read_stdin() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), std::io::Error> {
    let puzzle = Puzzle::parse(&read_stdin()?);

    println!("Part 1: {}", puzzle.part_1());

    Ok(())
}

mod test {
    #[allow(unused_imports)] // wtf?
    use super::*;

    #[allow(dead_code)] // wtf?
    const INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[allow(dead_code)] // wtf?
    fn verify_syntax(input: &str, expected:NavSubsystemLineSyntax) {
        let actual = parse_nav_subsystem_line(input);

        assert!(actual == expected, "@verify_syntax expected {:?}, was {:?}: {}", expected, actual, input);
    }

    #[test]
    fn syntax() {
        verify_syntax("{}", NavSubsystemLineSyntax::Valid);
        verify_syntax("[]", NavSubsystemLineSyntax::Valid);
        verify_syntax("([])", NavSubsystemLineSyntax::Valid);
        verify_syntax("{()()()}", NavSubsystemLineSyntax::Valid);
        verify_syntax("<([{}])>", NavSubsystemLineSyntax::Valid);
        verify_syntax("[<>({}){}[([])<>]]", NavSubsystemLineSyntax::Valid);
        verify_syntax("(((((((((())))))))))", NavSubsystemLineSyntax::Valid);

        verify_syntax("(]", NavSubsystemLineSyntax::Corrupted(57));
        verify_syntax("{()()()>", NavSubsystemLineSyntax::Corrupted(25137));
        verify_syntax("(((()))}", NavSubsystemLineSyntax::Corrupted(1197));
        verify_syntax("<([]){()}[{}])", NavSubsystemLineSyntax::Corrupted(3));

        verify_syntax("{([(<{}[<>[]}>{[]{[(<()>", NavSubsystemLineSyntax::Corrupted(1197));
        verify_syntax("[[<[([]))<([[{}[[()]]]", NavSubsystemLineSyntax::Corrupted(3));
        verify_syntax("[{[{({}]{}}([{[{{{}}([]", NavSubsystemLineSyntax::Corrupted(57));
        verify_syntax("[<(<(<(<{}))><([]([]()", NavSubsystemLineSyntax::Corrupted(3));
        verify_syntax("<{([([[(<>()){}]>(<<{{", NavSubsystemLineSyntax::Corrupted(25137));


        verify_syntax("[({(<(())[]>[[{[]{<()<>>", NavSubsystemLineSyntax::Incomplete); 
        verify_syntax("[(()[<>])]({[<{<<[]>>(", NavSubsystemLineSyntax::Incomplete); 
        verify_syntax("(((({<>}<{<{<>}{[]{[]{}", NavSubsystemLineSyntax::Incomplete); 
        verify_syntax("{<[[]]>}<{[{[{[]{()[[[]", NavSubsystemLineSyntax::Incomplete); 
        verify_syntax("<{([{{}}[<[[[<>{}]]]>[]]", NavSubsystemLineSyntax::Incomplete); 
    }

    #[test]
    fn part_1() {
        let puzzle = super::Puzzle::parse(INPUT);
        assert_eq!(puzzle.part_1(), 26397);
    }
}
