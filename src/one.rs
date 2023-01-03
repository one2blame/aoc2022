extern crate nom;

pub mod solve {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit0;
    use nom::multi::separated_list0;
    use nom::IResult;
    use std::cmp::max;
    use std::str::FromStr;

    #[allow(dead_code)]
    fn parse_integer_strings(input: &str) -> IResult<&str, Vec<&str>> {
        return separated_list0(tag("\n"), digit0)(input);
    }

    #[allow(dead_code)]
    pub fn first_puzzle(input: String) -> usize {
        let (_, integer_str_list) = parse_integer_strings(&input[..]).unwrap();

        let mut res = 0;
        let mut acc = 0;
        for integer_str in integer_str_list {
            if integer_str.is_empty() {
                res = max(res, acc);
                acc = 0;
            } else {
                let integer = usize::from_str(integer_str).unwrap();
                acc += integer;
            }
        }

        return res;
    }

    #[allow(dead_code)]
    pub fn second_puzzle(input: String) -> usize {
        let (_, integer_str_list) = parse_integer_strings(&input[..]).unwrap();

        let mut res: Vec<usize> = Vec::new();
        let mut acc = 0;
        for integer_str in integer_str_list {
            if integer_str.is_empty() {
                res.push(acc);
                acc = 0;
            } else {
                let integer = usize::from_str(integer_str).unwrap();
                acc += integer;
            }
        }

        res.sort();
        res.reverse();
        return res[0..3].iter().sum();
    }
}
