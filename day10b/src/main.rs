pub fn main() {
    let input = include_str!("../../day10a/input.txt");
    let response = perform(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        print_screen(&response)
    }
}

struct CpuState {
    cycle: u32,
    x_reg: isize,
}

fn update_screen(screen: &mut [[bool; 40]; 6], state: &CpuState) {
    let cycle = state.cycle - 1;
    let positions = [state.x_reg - 1, state.x_reg, state.x_reg + 1];

    let row = (cycle) / 40;
    let column = (cycle) % 40;

    let render = positions.contains(&(column as isize));
    screen[row as usize][column as usize] = render;
}

fn print_screen(screen: &[[bool; 40]; 6]) {
    for y in 0..screen.len() {
        for x in 0..screen[0].len() {
            let c = if screen[y][x] { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
}

fn perform(input: &str) -> [[bool; 40]; 6] {
    let mut screen = [[false; 40]; 6];
    let mut cpu = CpuState { cycle: 1, x_reg: 1 };
    update_screen(&mut screen, &cpu);

    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .next()
                .map(|e| e.parse::<isize>().unwrap())
        })
        .for_each(|curr| {
            if cpu.cycle == 240 {
                return;
            };

            match curr {
                None => {
                    let cycle_num = &mut cpu.cycle;
                    *cycle_num += 1;
                    update_screen(&mut screen, &mut cpu);
                }

                Some(delta) => {
                    for i in 0..2 {
                        let cycle_num = &mut cpu.cycle;
                        *cycle_num += 1;
                        if i == 1 {
                            cpu.x_reg += delta;
                        }
                        update_screen(&mut screen, &mut cpu);
                    }
                }
            }
        });
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        print_screen(&perform(test_input));
        assert!(false)
    }
}
