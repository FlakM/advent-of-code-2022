use std::cmp::min;

pub fn main() {
    let input = include_bytes!("../input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

fn perform(input: &[u8]) -> usize {
    let grid = input.split(|b| *b == b'\n').filter(|a|!a.is_empty()).collect::<Vec<&[u8]>>();
    let x_size = grid[0].len();
    let y_size = grid.len();

    let mut visible = 0;
    for x in 1..x_size-1 {
        for y in 1..y_size-1 {
            let elem = grid[y][x];
            let left = (0..x).map(|n| grid[y][n]).max().unwrap() < elem;
            let top = (0..y).map(|n| grid[n][x]).max().unwrap() < elem;
            //let x_right = x+1;
            let right = (x+1..=x_size).map(|n| grid[y][min(n,x_size-1)]).max().unwrap() < elem;
            let bottom = (y+1..=y_size).map(|n| grid[min(n,y_size-1)][x]).max().unwrap() < elem;

            if left || top || right || bottom {
                visible += 1;
            }
        }
    }
    visible + 2 * x_size + 2 * y_size - 4
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
        assert_eq!(perform(test_input.as_bytes()), 21);
    }
}
