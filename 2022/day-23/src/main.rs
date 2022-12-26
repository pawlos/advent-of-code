#![feature(slice_group_by)]
use std::{fmt::Display, fs};

#[derive(PartialEq, PartialOrd, Debug)]
enum Directions {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy)]
struct Elf {
    pos: (i32, i32),
    next_pos: Option<(i32, i32)>,
}

#[derive(Clone)]
struct Ground {
    elfs: Vec<Elf>,
}

impl Ground {
    fn empty_space(&self) -> i32 {
        let min_x = self.elfs.iter().map(|e| e.pos.0).min().unwrap();
        let max_x = self.elfs.iter().map(|e| e.pos.0).max().unwrap();

        let min_y = self.elfs.iter().map(|e| e.pos.1).min().unwrap();
        let max_y = self.elfs.iter().map(|e| e.pos.1).max().unwrap();

        let count = self.elfs.len() as i32;

        ((max_x - min_x) + 1) * ((max_y - min_y) + 1) - count
    }
}

impl Display for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.elfs.iter().map(|e| e.pos.0).min().unwrap();
        let max_x = self.elfs.iter().map(|e| e.pos.0).max().unwrap();

        let min_y = self.elfs.iter().map(|e| e.pos.1).min().unwrap();
        let max_y = self.elfs.iter().map(|e| e.pos.1).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elfs.iter().any(|e| e.pos == (x, y)) {
                    write!(f, "#")?
                } else {
                    write!(f, ".")?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}
fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input-23.txt")?;

    let mut ground = Ground {
        elfs: content
            .split("\r\n")
            .enumerate()
            .flat_map(|(j, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(i, c)| match c {
                        '#' => Elf {
                            pos: (i as i32, j as i32),
                            next_pos: None,
                        },
                        _ => panic!("Invalid character"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    };

    let mut ground2 = ground.clone();

    for i in 0..10 {
        calculate_movements(&mut ground, i);
        eliminate_duplicates(&mut ground);
        do_movements(&mut ground);
    }
    println!("Part 1 - {}", ground.empty_space());

    let mut no_movements = 0;
    for i in 0..i32::MAX {
        let copy = ground2.elfs.clone();
        calculate_movements(&mut ground2, i);
        eliminate_duplicates(&mut ground2);
        if no_movements_detected(&mut ground2) {
            no_movements = i + 1;
            break;
        }
        do_movements(&mut ground2);
    }
    println!("Part2 - {}", no_movements);
    Ok(())
}

fn no_movements_detected(ground: &mut Ground) -> bool {
    !ground.elfs.iter().any(|e| e.next_pos.is_some())
}

fn do_movements(ground: &mut Ground) {
    ground
        .elfs
        .iter_mut()
        .filter(|e| e.next_pos.is_some())
        .for_each(|e| {
            e.pos = e.next_pos.unwrap();
            e.next_pos = None;
        });
}

fn eliminate_duplicates(ground: &mut Ground) {
    let copy = ground.elfs.clone();
    ground
        .elfs
        .iter_mut()
        .filter(|e| e.next_pos.is_some())
        .filter(|e1| copy.iter().filter(|e2| e2.next_pos == e1.next_pos).count() > 1)
        //.inspect(|grp| println!("Grp: {:?}", grp))
        .for_each(|x: &mut Elf| x.next_pos = None);
}

fn calculate_movements(ground: &mut Ground, round: i32) {
    let elfs_copy = ground.elfs.clone();
    let directions = [
        Directions::North,
        Directions::South,
        Directions::West,
        Directions::East,
    ];
    for elf in ground.elfs.iter_mut() {
        if !check_neighbours(elf, &elfs_copy) {
            continue;
        }
        for i in 0..4 {
            let direction = &directions[(round + i) as usize % 4];
            let can_move = can_move(elf, &elfs_copy, direction);
            if can_move.is_some() {
                elf.next_pos = can_move;
                break;
            }
        }
    }
}

fn check_neighbours(elf: &Elf, elfs: &Vec<Elf>) -> bool {
    //print!("Check neighbours for {:?} -> ", elf.pos);
    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let point = (elf.pos.0 + dx, elf.pos.1 + dy);
        if elfs.iter().any(|e| e.pos == point) {
            //println!("true");
            return true;
        }
    }
    //println!("false");
    false
}

fn can_move(elf: &Elf, elfs: &Vec<Elf>, direction: &Directions) -> Option<(i32, i32)> {
    //println!("{:?}", direction);
    if *direction == Directions::North && can_move_north(elf, elfs) {
        return Some((elf.pos.0, elf.pos.1 - 1));
    }
    if *direction == Directions::South && can_move_south(elf, elfs) {
        return Some((elf.pos.0, elf.pos.1 + 1));
    }
    if *direction == Directions::West && can_move_west(elf, elfs) {
        return Some((elf.pos.0 - 1, elf.pos.1));
    }
    if *direction == Directions::East && can_move_east(elf, elfs) {
        return Some((elf.pos.0 + 1, elf.pos.1));
    }

    None
}

fn can_move_east(elf: &Elf, elfs: &Vec<Elf>) -> bool {
    for (dx, dy) in [(1, -1), (1, 0), (1, 1)] {
        let new_pos = (elf.pos.0 + dx, elf.pos.1 + dy);

        if elfs.iter().any(|p| p.pos == new_pos) {
            return false;
        }
    }
    true
}

fn can_move_west(elf: &Elf, elfs: &Vec<Elf>) -> bool {
    for (dx, dy) in [(-1, -1), (-1, 0), (-1, 1)] {
        let new_pos = (elf.pos.0 + dx, elf.pos.1 + dy);

        if elfs.iter().any(|p| p.pos == new_pos) {
            return false;
        }
    }
    true
}

fn can_move_south(elf: &Elf, elfs: &Vec<Elf>) -> bool {
    for (dx, dy) in [(-1, 1), (0, 1), (1, 1)] {
        let new_pos = (elf.pos.0 + dx, elf.pos.1 + dy);

        if elfs.iter().any(|p| p.pos == new_pos) {
            return false;
        }
    }
    true
}

fn can_move_north(elf: &Elf, elfs: &Vec<Elf>) -> bool {
    for (dx, dy) in [(-1, -1), (0, -1), (1, -1)] {
        let new_pos = (elf.pos.0 + dx, elf.pos.1 + dy);

        if elfs.iter().any(|p| p.pos == new_pos) {
            return false;
        }
    }
    true
}
