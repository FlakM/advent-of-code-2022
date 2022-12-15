use itertools::Itertools;
use std::{
    cmp::{max, min},
    fmt::{self, Formatter},
}; // 0.9.0

pub fn main() {
    let input = include_str!("../../day14a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perform(input: &str) -> usize {
    let mut cave = Cave::new();
    dbg!("cave inited");
    cave.add_rocks(input);

    while !cave.drop_sand() {}
    println!("{}", cave);

    cave.sand_counter
}

#[derive(Copy, Clone)]
enum Material {
    Sand,
    Air,
    Rock,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Material::Sand => write!(f, "o"),
            Material::Air => write!(f, "."),
            Material::Rock => write!(f, "#"),
        }
    }
}

struct Cave {
    area: Vec<Vec<Material>>,
    sand_counter: usize,
    max_y: usize,
}

#[derive(Debug)]
enum Move {
    Stuck((usize, usize)),
    Abyss,
}

impl Cave {
    fn new() -> Self {
        Cave {
            area: vec![vec![Material::Air; 4000]; 4000],
            sand_counter: 0,
            max_y: 0,
        }
    }

    fn move_sane(&self, y: usize, x: usize) -> Move {
        if y == self.area.len() - 1 || x == 0 || x == self.area[0].len() {
            return Move::Abyss;
        };

        let left = self.area[y + 1][x - 1];
        let mut y_mut = y;
        if matches!(left, Material::Air) {
            y_mut += (y + 1..self.area.len())
                .take_while(|y_down| matches!(self.area[*y_down][x - 1], Material::Air))
                .count();
            return self.move_sane(y_mut, x - 1);
        } else {
            let right = self.area[y + 1][x + 1];
            if matches!(right, Material::Air) {
                y_mut += (y + 1..self.area.len())
                    .take_while(|y_down| matches!(self.area[*y_down][x + 1], Material::Air))
                    .count();
                return self.move_sane(y_mut, x + 1);
            } else {
                return Move::Stuck((y, x));
            }
        }
    }

    fn find_drop(&mut self, (y, x): (usize, usize)) -> Move {
        self.move_sane(y, x)
    }

    /// returns true if abyss is found
    fn drop_sand(&mut self) -> bool {
        if !matches!(self.area[0][500], Material::Air) {
            return true;
        }
        let index_of_blocked = (0..self.area.len())
            .find(|i| !matches!(self.area[*i][500], Material::Air))
            .unwrap()
            - 1;

        match self.find_drop((index_of_blocked, 500)) {
            Move::Stuck((y, x)) => {
                self.area[y][x] = Material::Sand;
                self.sand_counter += 1;
                false
            }
            Move::Abyss => {
                dbg!(self.sand_counter, index_of_blocked);
                panic!("should not happen")
            }
        }
    }

    fn add_rocks(&mut self, coordinates: &str) {
        let mut max_y = 0;
        coordinates.lines().for_each(|l| {
            let rocks: Vec<(usize, usize)> = l
                .split(" -> ")
                .map(|r| {
                    let (a, b) = r.split_once(',').unwrap();
                    let (a, b) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
                    max_y = max(max_y, b);
                    (a, b)
                })
                .collect::<Vec<_>>();

            for ((x1, y1), (x2, y2)) in rocks.into_iter().tuple_windows() {
                for x in min(x1, x2)..=max(x1, x2) {
                    for y in min(y1, y2)..=max(y1, y2) {
                        dbg!(y, x);
                        self.area[y][x] = Material::Rock;
                    }
                }
            }
        });
        self.max_y =  2 + max_y;
        self.area[2 + max_y]
            .iter_mut()
            .for_each(|a| *a = Material::Rock);
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.area[0..self.max_y] {
            for material in &row[450..=550] {
                write!(f, "{}", material)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(perform(test_input), 93);
    }
}
