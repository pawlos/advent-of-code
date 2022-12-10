use std::fs;

#[derive(Clone, Copy, Debug)]
struct Command {
    delta_x: i32,
    delta_y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply(&mut self, command: Command) {
        self.x += command.delta_x;
        self.y += command.delta_y;
    }

    fn deduce_tail_move(&self, tail: &Self) -> Command {
        let distance_x_from = self.x - tail.x;
        let distance_y_from = self.y - tail.y;

        let delta_x = if distance_x_from.abs() > 1 && self.y == tail.y {
            distance_x_from.signum()
        } else {
            0
        };

        let delta_y = if distance_y_from.abs() > 1 && self.x == tail.x {
            distance_y_from.signum()
        } else {
            0
        };

        let (delta_x, delta_y) = if (distance_x_from.abs() > 1 && self.y != tail.y)
            || (distance_y_from.abs() > 1 && self.x != tail.x)
        {
            (distance_x_from.signum(), distance_y_from.signum())
        } else {
            (delta_x, delta_y)
        };

        Command { delta_x, delta_y }
    }
}

impl Command {
    fn up() -> Self {
        Command {
            delta_x: 0,
            delta_y: -1,
        }
    }

    fn down() -> Self {
        Command {
            delta_x: 0,
            delta_y: 1,
        }
    }

    fn left() -> Self {
        Command {
            delta_x: -1,
            delta_y: 0,
        }
    }

    fn right() -> Self {
        Command {
            delta_x: 1,
            delta_y: 0,
        }
    }
}

fn simulate_rope(commands: &Vec<Command>, knots: usize) -> usize {
    let mut rope = vec![Point { x: 0, y: 0 }; knots];
    let mut visited: Vec<Point> = Vec::new();
    for command in commands {
        rope[0].apply(*command);
        for i in 0..rope.len() - 1 {
            let elems = &mut rope[i..i + 2];
            let tail_cmd = elems[0].deduce_tail_move(&elems[1]);
            elems[1].apply(tail_cmd);
        }
        visited.push(rope[rope.len() - 1]);
    }
    visited.sort_by_key(|point| (point.x, point.y));
    visited.dedup_by(|p1, p2| p1.x == p2.x && p1.y == p2.y);
    visited.len()
}
fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input-09.txt")?;
    let commands = content
        .lines()
        .flat_map(|l| {
            if let Some((dir, steps)) = l.split_once(' ') {
                match dir {
                    "U" => vec![Command::up(); steps.parse::<usize>().unwrap()],
                    "D" => vec![Command::down(); steps.parse::<usize>().unwrap()],
                    "L" => vec![Command::left(); steps.parse::<usize>().unwrap()],
                    "R" => vec![Command::right(); steps.parse::<usize>().unwrap()],
                    _ => panic!("What to do???"),
                }
            } else {
                panic!("")
            }
        })
        .collect::<Vec<_>>();

    println!("Part1 - {:?}", simulate_rope(&commands, 2));
    println!("Part2 - {:?}", simulate_rope(&commands, 10));
    Ok(())
}
