use std::fs;

fn count_priority(c: char) -> u32 {
    match c {
        _ if c.is_lowercase() => ((c as u8) - 0x61 + 1) as u32,
        _ if c.is_uppercase() => ((c as u8) - 0x41 + 27) as u32,
        _ => panic!("WAT???"),
    }
}

fn main() {
    let content = fs::read_to_string("input-03.txt").unwrap();
    let backpacks = content.split('\n').map(|l| {
        let splited_line = l.split_at(l.len() / 2);
        (splited_line.0, splited_line.1)
    });

    let common_chars = backpacks.map(|b: (&str, &str)| {
        for c1 in b.0.chars() {
            if b.1.contains(c1) {
                return c1;
            }
        }
        panic!("Did not found common char.");
    });

    let sum = common_chars.map(|c: char| count_priority(c));

    println!("Part1: {:?}", sum.sum::<u32>());

    let lines: Vec<&str> = content.split('\n').collect();

    let three_packs = lines.chunks_exact(3).map(|c| c);
    let common_chars = three_packs.map(|b| {
        for c1 in b[0].chars() {
            if b[1].contains(c1) && b[2].contains(c1) {
                return c1;
            }
        }
        panic!("Did not found common char.");
    });
    let sum = common_chars.map(|c: char| count_priority(c));
    println!("Part2: {:?}", sum.sum::<u32>());
}
