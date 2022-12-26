use core::fmt::Display;
use std::fs;

const X: usize = 1000;
const Y: usize = 1000;

enum StopReson {
    Stationary,
    FallIntoAbyss,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Occupant {
    Empty,
    Rock,
    Sand,
}

impl Display for Occupant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Occupant::Empty => write!(f, "."),
            Occupant::Rock => write!(f, "#"),
            Occupant::Sand => write!(f, "o"),
        }
    }
}

#[derive(Clone, Copy)]
struct Cave {
    space: [[Occupant; Y]; X],
    bottom: usize,
    top: usize,
    left: usize,
    right: usize,
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0 as usize..=self.bottom + 1 {
            for x in 0..=self.right + 1 {
                write!(f, "{}", self.space[y][x]);
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

impl Cave {
    fn create(lines: Vec<&str>) -> Self {
        let mut cave = [[Occupant::Empty; Y]; X];
        let mut left = X;
        let mut right = 0;
        let mut top = Y;
        let mut bottom = 0;
        for line in lines {
            let entries = line.split(" -> ");
            entries.collect::<Vec<_>>().windows(2).for_each(|e| {
                let points = e
                    .iter()
                    .map(|xy| {
                        let (x, y) = xy.split_once(',').unwrap();
                        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                    })
                    .collect::<Vec<_>>();

                let (x1, y1) = points[0];
                let (x2, y2) = points[1];

                left = left.min(x1.min(x2));
                top = top.min(y1.min(y2));
                right = right.max(x1.max(x2));
                bottom = bottom.max(y1.max(y2));

                if x1 == x2 {
                    // veritcal line
                    let y_diff = (y2 as i32 - y1 as i32).signum();
                    for i in 0..=y2.abs_diff(y1) {
                        cave[(y1 as i32 + i as i32 * y_diff) as usize][x1] = Occupant::Rock;
                    }
                } else if y1 == y2 {
                    //horizontal line
                    let x_diff = (x2 as i32 - x1 as i32).signum();
                    for i in 0..=x2.abs_diff(x1) {
                        cave[y1][(x1 as i32 + i as i32 * x_diff) as usize] = Occupant::Rock;
                    }
                } else {
                    panic!("Not horizontal or vertical line. DA FUK?")
                }
            });
        }
        Cave {
            space: cave,
            left,
            top,
            bottom,
            right,
        }
    }

    fn move_sand(&mut self) -> StopReson {
        let mut x = 500;
        let mut y = 0;
        let mut fall_into_abyss = None;
        loop {
            if fall_into_abyss.is_some() {
                break;
            }
            if self.space[y + 1][x] == Occupant::Empty {
                (self.space[y][x], self.space[y + 1][x]) = (self.space[y + 1][x], self.space[y][x]);
                y += 1;
                if y > self.bottom {
                    fall_into_abyss = Some(StopReson::FallIntoAbyss);
                }
                continue;
            }
            if self.space[y + 1][x - 1] == Occupant::Empty {
                (self.space[y][x], self.space[y + 1][x - 1]) =
                    (self.space[y + 1][x - 1], self.space[y][x]);
                y += 1;
                x -= 1;
                continue;
            }
            if self.space[y + 1][x + 1] == Occupant::Empty {
                (self.space[y][x], self.space[y + 1][x + 1]) =
                    (self.space[y + 1][x + 1], self.space[y][x]);
                y += 1;
                x += 1;
                continue;
            }
            fall_into_abyss = Some(StopReson::Stationary);
        }
        fall_into_abyss.unwrap()
    }

    fn simulate_sand(&mut self) -> usize {
        let mut counter = 0;
        loop {
            self.space[0][500] = Occupant::Sand;
            match self.move_sand() {
                StopReson::Stationary => (),
                StopReson::FallIntoAbyss => break,
            };
            counter += 1;
        }
        counter
    }

    fn simulate_sand2(&mut self) -> usize {
        let mut counter = 0;
        for x in 0..X {
            self.space[self.bottom + 2][x] = Occupant::Rock;
        }
        self.bottom += 2;
        loop {
            if self.space[0][500] == Occupant::Sand {
                break;
            }
            self.space[0][500] = Occupant::Sand;
            match self.move_sand() {
                StopReson::Stationary => (),
                StopReson::FallIntoAbyss => break,
            };
            counter += 1;
        }
        counter
    }
}

fn main() -> std::io::Result<()> {
    println!("🎄 Ostra Piła - Advent of Code 2022 - Day 14 🎄");
    let content = fs::read_to_string("input-14.txt")?;

    let lines = content.split("\r\n").collect::<Vec<_>>();
    let mut cave = Cave::create(lines);
    let mut cave2 = cave.clone();
    let count = cave.simulate_sand();
    println!("Part1 - {:?}", count);
    let count = cave2.simulate_sand2();
    println!("Part2 - {:?}", count);
    Ok(())
}
