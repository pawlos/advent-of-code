use std::{cmp::Ordering, fs};

fn lt(lhs: &json::JsonValue, rhs: &json::JsonValue) -> Option<bool> {
    if lhs.is_number() && rhs.is_number() {
        return lt_num(lhs, rhs);
    }
    if lhs.is_array() && rhs.is_array() {
        return lt_array(lhs, rhs);
    }
    if lhs.is_number() && rhs.is_array() {
        return lt_num_array(lhs, rhs);
    }
    if lhs.is_array() && rhs.is_number() {
        return lt_array_num(lhs, rhs);
    }
    panic!("Should not happen");
}

fn lt_num_array(lhs: &json::JsonValue, rhs: &json::JsonValue) -> Option<bool> {
    let mut lhs_as_array = json::JsonValue::new_array();
    lhs_as_array.push(lhs.clone()).unwrap();
    lt_mixed(&lhs_as_array, rhs)
}

fn lt_array_num(lhs: &json::JsonValue, rhs: &json::JsonValue) -> Option<bool> {
    let mut rhs_as_array = json::JsonValue::new_array();
    rhs_as_array.push(rhs.clone()).unwrap();
    lt_mixed(lhs, &rhs_as_array)
}

fn lt_mixed(lhs: &json::JsonValue, rhs: &json::JsonValue) -> Option<bool> {
    lt_array(lhs, rhs)
}

fn lt_array(lhs: &json::JsonValue, rhs: &json::JsonValue) -> Option<bool> {
    let lhs_size = lhs.members().len();
    let rhs_size = rhs.members().len();

    for (i, elem) in lhs.members().enumerate() {
        if i >= rhs.members().len() {
            return Some(false);
        }
        if let Some(v) = lt(elem, &rhs.members().as_slice()[i]) {
            return Some(v);
        }
    }
    if lhs_size < rhs_size {
        Some(true)
    } else {
        None
    }
}

fn lt_num(lhs: &json::JsonValue, rhs: &json::JsonValue) -> Option<bool> {
    if lhs.as_i32().unwrap() == rhs.as_i32().unwrap() {
        None
    } else {
        Some(lhs.as_i32().unwrap() < rhs.as_i32().unwrap())
    }
}

fn main() -> std::io::Result<()> {
    println!("🎄 Ostra Piła - Advent of Code 2022 - Day 13 🎄");
    let content = fs::read_to_string("input-13.txt")?;
    let packets = content
        .split("\r\n")
        .filter(|l| !l.is_empty())
        .map(|l| json::parse(l).unwrap())
        .collect::<Vec<_>>();

    let part_1 = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, chunk)| lt(&chunk[0], &chunk[1]).unwrap())
        .map(|(idx, _)| idx + 1)
        .sum::<usize>();
    println!("Part1 - {:?}", part_1);

    let divider_2 = json::parse("[[2]]").unwrap();
    let divider_6 = json::parse("[[6]]").unwrap();

    let mut packets = packets;
    packets.push(divider_2);
    packets.push(divider_6);

    packets.sort_by(|p1, p2| {
        if lt(p1, p2).unwrap() {
            Ordering::Less
        } else if lt(p2, p1).unwrap() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let mut packets = packets
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p.dump()))
        .collect::<Vec<_>>();

    packets.retain(|(_, v)| v == "[[2]]" || v == "[[6]]");
    println!(
        "Part2 - {:?}",
        packets.iter().map(|(i, _)| i).product::<usize>()
    );

    Ok(())
}
