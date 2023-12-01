use itertools::Itertools;

pub fn one() {
    let input = std::fs::read_to_string("./input/1.txt").unwrap();

    let result = input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .map(|v| match (v.first(), v.last()) {
            (Some(c1), Some(c2)) => 10 * ((*c1 as u32) - 48) + *c2 as u32 - 48,
            _ => 0,
        })
        .sum::<u32>();

    println!("result: {result}");
}

pub fn two() {
    let input = std::fs::read_to_string("./input/1.txt").unwrap();

    let result = input
        .lines()
        .map(|l| {
            l.chars()
                .chain("     ".chars())
                .tuple_windows::<(_, _, _, _, _)>()
                .map(|quingram| match quingram {
                    digit @ ('0'..='9', _, _, _, _) => Some(digit.0 as u32 - 48),
                    ('o', 'n', 'e', _, _) => Some(1),
                    ('t', 'w', 'o', _, _) => Some(2),
                    ('t', 'h', 'r', 'e', 'e') => Some(3),
                    ('f', 'o', 'u', 'r', _) => Some(4),
                    ('f', 'i', 'v', 'e', _) => Some(5),
                    ('s', 'i', 'x', _, _) => Some(6),
                    ('s', 'e', 'v', 'e', 'n') => Some(7),
                    ('e', 'i', 'g', 'h', 't') => Some(8),
                    ('n', 'i', 'n', 'e', _) => Some(9),
                    _ => None,
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .map(|v| match (v.first(), v.last()) {
            (Some(v1), Some(v2)) => 10 * v1 + v2,
            _ => 0,
        })
        .sum::<u32>();

    println!("result: {result}");
}
