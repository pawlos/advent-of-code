use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy)]
struct Drop {
    x: i32,
    y: i32,
    z: i32,
    neighbours: i32,
}

fn count_neighbours(xyz: (i32, i32, i32), droplet: &[Drop]) -> i32 {
    droplet
        .iter()
        .filter(|ld| {
            ((ld.x == xyz.0 + 1 || ld.x == xyz.0 - 1) && ld.y == xyz.1 && ld.z == xyz.2)
                || (ld.x == xyz.0 && (ld.y == xyz.1 + 1 || ld.y == xyz.1 - 1) && ld.z == xyz.2)
                || (ld.x == xyz.0 && ld.y == xyz.1 && (ld.z == xyz.2 + 1 || ld.z == xyz.2 - 1))
        })
        .count() as i32
}

fn main() -> std::io::Result<()> {
    println!("🎄 Ostra Piła - Advent of Code 2022 - Day 18 🎄");

    let content = fs::read_to_string("input-18.txt")?;

    let mut lava_drops = content
        .split("\r\n")
        .map(|l| {
            let coords = l
                .split(',')
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            Drop {
                x: coords[0],
                y: coords[1],
                z: coords[2],
                neighbours: 0,
            }
        })
        .collect::<Vec<_>>();

    let lava_drops2 = lava_drops.clone();

    for mut drop in lava_drops.as_mut_slice() {
        let neighbours = count_neighbours((drop.x, drop.y, drop.z), &lava_drops2);
        drop.neighbours += neighbours;
    }
    let lava_drops3 = lava_drops
        .iter()
        .map(|f| (f.x, f.y, f.z))
        .collect::<Vec<_>>();

    let part1 =
        lava_drops2.len() as i32 * 6 - lava_drops.into_iter().map(|d| d.neighbours).sum::<i32>();

    println!("Part1 - {:?}", part1);

    /*let mut visited: Vec<(i32, i32, i32)> = vec![];
    let mut reachable: Vec<(i32, i32, i32)> = vec![];
    let mut result = vec![];
    for point in lava_drops3.iter() {
        let mut air = find_reachable(*point, &lava_drops3, &mut reachable, &mut visited);
        result.append(&mut air);
    }

    result.sort();
    result.dedup();*/

    let min_x = lava_drops3.iter().map(|e| e.0).min().unwrap();
    let max_x = lava_drops3.iter().map(|e| e.0).max().unwrap();

    let min_y = lava_drops3.iter().map(|e| e.1).min().unwrap();
    let max_y = lava_drops3.iter().map(|e| e.1).max().unwrap();

    let min_z = lava_drops3.iter().map(|e| e.2).min().unwrap();
    let max_z = lava_drops3.iter().map(|e| e.2).max().unwrap();

    let mut air = vec![];
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if let Some(min_z) = lava_drops3.iter().filter(|f| f.0 == x && f.1 == y).min() {
                air.push((x, y, min_z.2 - 1));
            }
            if let Some(max_z) = lava_drops3.iter().filter(|f| f.0 == x && f.1 == y).max() {
                air.push((x, y, max_z.2 + 1));
            }
        }
    }

    for z in min_z..=max_z {
        for y in min_y..=max_y {
            if let Some(min_x) = lava_drops3.iter().filter(|f| f.2 == z && f.1 == y).min() {
                air.push((min_x.0 - 1, y, z));
            }
            if let Some(max_x) = lava_drops3.iter().filter(|f| f.2 == z && f.1 == y).max() {
                air.push((max_x.0 + 1, y, z));
            }
        }
    }

    for x in min_x..=max_x {
        for z in min_z..=max_z {
            if let Some(min_y) = lava_drops3.iter().filter(|f| f.0 == x && f.2 == z).min() {
                air.push((x, min_y.1 - 1, z));
            }
            if let Some(max_y) = lava_drops3.iter().filter(|f| f.0 == x && f.2 == z).max() {
                air.push((x, max_y.1 + 1, z));
            }
        }
    }

    let mut result: HashSet<(i32, i32, i32)> = air.clone().into_iter().collect();
    for point in air.iter() {
        let reached_air = find_reachable_air(*point, &air, &lava_drops3);
        reached_air.into_iter().for_each(|e| {
            result.insert(e);
        });
    }

    //println!("{:?}", result);
    //result.sort();
    //result.dedup();
    let count: i32 = result
        .iter()
        .map(|a| (a, count_neighbours(*a, &lava_drops2)))
        //.inspect(|(e, v)| print!("({},{},{}), {}; ", e.0, e.1, e.2, v))
        .map(|e| e.1)
        .sum();

    println!("Part2 - {:?}", count);

    Ok(())
}

fn find_reachable_air(
    point: (i32, i32, i32),
    air: &[(i32, i32, i32)],
    lava: &[(i32, i32, i32)],
) -> Vec<(i32, i32, i32)> {
    let mut to_vist: Vec<(i32, i32, i32)> = vec![];
    let mut reachable: Vec<(i32, i32, i32)> = vec![];
    to_vist.push(point);
    while !to_vist.is_empty() {
        let p = to_vist.pop().unwrap();
        reachable.push(p);
        for (dx, dy, dz) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let new_point = (point.0 + dx, point.1 + dy, point.2 + dz);

            if new_point.0 < 0 || new_point.0 > 22 {
                continue;
            }
            if new_point.1 < 0 || new_point.1 > 22 {
                continue;
            }
            if new_point.2 < 0 || new_point.2 > 22 {
                continue;
            }
            if reachable.contains(&new_point) {
                continue;
            }
            if reachable.contains(&new_point) {
                continue;
            }
            if air.contains(&new_point) {
                continue;
            }
            if lava.contains(&new_point) {
                continue;
            }
            to_vist.push(new_point);
        }
    }
    reachable.to_vec()
}
