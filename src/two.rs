extern crate nom;

pub mod solve {
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, space1};
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;
    use nom::IResult;
    use std::collections::HashMap;

    #[allow(dead_code)]
    fn parse_game(input: &str) -> IResult<&str, (&str, &str)> {
        return separated_pair(alpha1, space1, alpha1)(input);
    }

    #[allow(dead_code)]
    fn parse_lines(input: &str) -> IResult<&str, Vec<String>> {
        return separated_list0(
            tag("\n"),
            map(parse_game, |choice| format!("{}{}", choice.0, choice.1)),
        )(input);
    }

    #[allow(dead_code)]
    pub fn first_puzzle(input: String) -> usize {
        let outcomes = HashMap::from([
            ("AX", 4),
            ("AY", 8),
            ("AZ", 3),
            ("BX", 1),
            ("BY", 5),
            ("BZ", 9),
            ("CX", 7),
            ("CY", 2),
            ("CZ", 6),
        ]);

        let (_, games) = parse_lines(&input[..]).unwrap();

        let mut res = 0;
        for game in games {
            res += outcomes.get(&game[..]).unwrap();
        }

        return res;
    }

    #[allow(dead_code)]
    pub fn second_puzzle(input: String) -> usize {
        let outcomes = HashMap::from([
            ("AX", 3),
            ("AY", 4),
            ("AZ", 8),
            ("BX", 1),
            ("BY", 5),
            ("BZ", 9),
            ("CX", 2),
            ("CY", 6),
            ("CZ", 7),
        ]);

        let (_, games) = parse_lines(&input[..]).unwrap();

        let mut res = 0;
        for game in games {
            res += outcomes.get(&game[..]).unwrap();
        }

        return res;
    }
}
