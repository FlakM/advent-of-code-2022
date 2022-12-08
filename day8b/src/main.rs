use std::cmp::max;

pub fn main() {
    let input = include_bytes!("../../day8a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perform(input: &[u8]) -> usize {
    let grid = input
        .split(|b| *b == b'\n')
        .filter(|a| !a.is_empty())
        .collect::<Vec<&[u8]>>();
    let x_size = grid[0].len();
    let y_size = grid.len();

    let mut visible = 0;
    for x in 0..x_size {
        for y in 0..y_size {
            let elem = grid[y][x];
            let left = (0..x)
                .rev()
                .map(|n| grid[y][n])
                .fold((0, None), |acc, i| {
                    if let Some(_) = acc.1 {
                        acc
                    } else {
                        if i >= elem {
                            (acc.0 + 1, Some(i))
                        } else {
                            (acc.0 + 1, None)
                        }
                    }
                })
                .0;
            let top = (0..y)
                .rev()
                .map(|n| grid[n][x])
                .fold((0, None), |acc, i| {
                    if let Some(_) = acc.1 {
                        acc
                    } else {
                        if i >= elem {
                            (acc.0 + 1, Some(i))
                        } else {
                            (acc.0 + 1, None)
                        }
                    }
                })
                .0;

            let right = (x + 1..x_size)
                .map(|n| grid[y][n])
                .fold((0, None), |acc, i| {
                    if let Some(_) = acc.1 {
                        acc
                    } else {
                        if i >= elem {
                            (acc.0 + 1, Some(i))
                        } else {
                            (acc.0 + 1, None)
                        }
                    }
                })
                .0;
            let bottom = (y + 1..y_size)
                .map(|n| grid[n][x])
                .fold((0, None), |acc, i| {
                    if let Some(_) = acc.1 {
                        acc
                    } else {
                        if i >= elem {
                            (acc.0 + 1, Some(i))
                        } else {
                            (acc.0 + 1, None)
                        }
                    }
                })
                .0;

            let curr = left as usize * top as usize * right as usize * bottom as usize;

            if y == 3 && x == 2 {
                println!(
                    "found new max: {x},{y}  l:{left}, r:{right}, top: {top}, bottom: {bottom}"
                );
                println!("right: {} - {}", x + 1, x_size);
            }
            visible = max(visible, curr);
        }
    }
    //80028 - to low
    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "30373
25512
65332
33549
35390";
        assert_eq!(perform(test_input.as_bytes()), 8);
    }
}
