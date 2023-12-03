use std::{convert::Infallible, str::FromStr};

#[derive(Default, Clone, Debug)]
struct Game {
    id: u32,
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Game::default();

        let mut id_games = s.trim().split(":");
        res.id = id_games
            .next()
            .expect("String is empty")
            .trim()
            .get(5..)
            .expect("Couldn't get id characters")
            .parse()
            .expect("Couldn't turn what should be a number into a number");

        id_games
            .next()
            .expect("Couldn't get list of games")
            .trim()
            .replace(';', ",")
            .split(",")
            .into_iter()
            .for_each(|s| {
                let mut num_col = s.trim().split(" ");
                let (num, col): (u32, _) = (
                    num_col
                        .next()
                        .expect("Couldn't get number in line")
                        .parse()
                        .expect("Couldn't turn what should be a number into a number"),
                    num_col.next().expect("Couldn't get color in line"),
                );
                match col {
                    "red" => res.reds.push(num),
                    "green" => res.greens.push(num),
                    "blue" => res.blues.push(num),
                    _ => unreachable!("Color other than red, blue or green encountered: {}", col),
                }
            });

        Ok(res)
    }
}

impl Game {
    fn possible(&self, r: u32, g: u32, b: u32) -> bool {
        let r_max = self.reds.iter().max().copied().unwrap_or(0);
        let g_max = self.greens.iter().max().copied().unwrap_or(0);
        let b_max = self.blues.iter().max().copied().unwrap_or(0);

        r_max <= r && g_max <= g && b_max <= b
    }

    fn power(&self) -> u32 {
        let r_min = self.reds.iter().max().copied().unwrap_or(0);
        let g_min = self.greens.iter().max().copied().unwrap_or(0);
        let b_min = self.blues.iter().max().copied().unwrap_or(0);

        r_min * g_min * b_min
    }
}

pub fn one() -> u32 {
    let input = std::fs::read_to_string("./input/2.txt").unwrap();

    input.lines()
        .map(Game::from_str)
        .flatten()
        .filter(|g| g.possible(12, 13, 14))
        .map(|g| g.id)
        .sum()
}

pub fn two() -> u32 {
    let input = std::fs::read_to_string("./input/2.txt").unwrap();

    input.lines()
        .map(Game::from_str)
        .flatten()
        .map(|g| g.power())
        .sum()
}
