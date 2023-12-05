use std::collections::HashSet;

use itertools::Itertools;

#[derive(Clone, Debug)]
struct GridNum {
    index: u32,
    values: Vec<u32>,
}

#[test]
fn chunk() {
    let c = GridNum::new(6, '5').add(5).add(9).add(0);
    assert_eq!(c.index, 6);
    assert_eq!(c.len(), 4);
    assert_eq!(c.values, vec![5, 5, 9, 0]);
    assert_eq!(c.value(), 5590);
}

impl GridNum {
    fn new(index: u32, digit: char) -> Self {
        Self {
            index,
            values: vec![digit as u32 - 48],
        }
    }

    fn len(&self) -> u32 {
        self.values.len() as u32
    }

    fn last_index(&self) -> u32 {
        self.index + self.len() - 1
    }

    fn add(mut self, digit: u32) -> Self {
        self.values.push(digit);
        self
    }

    fn value(&self) -> u32 {
        self.values
            .iter()
            .rev()
            .enumerate()
            .map(|(i, val)| val * 10u32.pow(i as u32))
            .sum()
    }

    fn is_adjacent(&self, line_n: u32, symbol_locations: &HashSet<(u32, u32)>) -> bool {
        ((line_n).saturating_sub(1)..=(line_n + 1))
            .into_iter()
            .cartesian_product(self.index.saturating_sub(1)..=(self.index + self.len()))
            .any(|coord| symbol_locations.contains(&coord))
    }

    fn includes_index(&self, index: u32) -> bool {
        index >= self.index && index <= self.index + self.len() - 1
    }
}

pub fn one() -> u32 {
    let input = std::fs::read_to_string("./input/3.txt").unwrap();

    let symbol_locations = input
        .lines()
        .enumerate()
        .map(|(line_n, line)| {
            line.trim()
                .char_indices()
                .filter(|(_, c)| !(c.is_digit(10) || *c == '.'))
                .map(move |(i, _)| (line_n as u32, i as u32))
        })
        .flatten()
        .collect::<HashSet<(u32, u32)>>();

    input
        .lines()
        .enumerate()
        .map(|(line_n, line)| {
            line.char_indices()
                .filter(|(_, c)| c.is_digit(10))
                .map(|(i, c)| GridNum::new(i as u32, c))
                .coalesce(|prev, curr| {
                    if prev.last_index() + 1 == curr.index {
                        Ok(prev.add(curr.values[0]))
                    } else {
                        Err((prev, curr))
                    }
                })
                .filter(|num| num.is_adjacent(line_n as u32, &symbol_locations))
                .map(|n| n.value())
                .sum::<u32>()
        })
        .sum()
}

pub fn two() -> u32 {
    let input = std::fs::read_to_string("./input/3.txt").unwrap();

    let grid = input
        .lines()
        .map(|line| {
            line.char_indices()
                .filter(|(_, c)| c.is_digit(10))
                .map(|(i, c)| GridNum::new(i as u32, c))
                .coalesce(|prev, curr| {
                    if prev.last_index() + 1 == curr.index {
                        Ok(prev.add(curr.values[0]))
                    } else {
                        Err((prev, curr))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    input
        .lines()
        .enumerate()
        .map(|(line_n, line)| {
            line.char_indices()
                .filter(|(_, c)| *c == '*')
                .map(|(i, _)| i)
                .filter_map(|i| {
                    let adj_nums = (line_n.saturating_sub(1)..=(line_n + 1))
                        .cartesian_product(i.saturating_sub(1)..=(i + 1))
                        .map(|(row_n, col)| match grid.get(row_n) {
                            Some(row) => row
                                .iter()
                                .filter(|num| num.includes_index(col as u32))
                                .map(|num| (row_n, num))
                                .next(),
                            _ => None,
                        })
                        .flatten()
                        .unique_by(|(row_n, num)| (*row_n, num.index))
                        .map(|(_, num)| num)
                        .collect::<Vec<_>>();

                    adj_nums
                        .len()
                        .eq(&2)
                        .then(|| adj_nums[0].value() * adj_nums[1].value())
                })
                .sum::<u32>()
        })
        .sum()
}
