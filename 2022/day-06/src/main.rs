use std::fs;

fn find_marker(input: &str, marker_len: usize) -> (usize, &str) {
    input
        .as_bytes()
        .windows(marker_len)
        .enumerate()
        .filter(|(_, chunk)| {
            let mut v = chunk.to_vec();
            v.sort_unstable();
            v.dedup();
            v.len() == marker_len
        })
        .map(|(idx, chunk)| (idx + marker_len, std::str::from_utf8(chunk).unwrap()))
        .next()
        .unwrap()
}

fn main() {
    let content = fs::read_to_string("input-06.txt").unwrap();
    println!("Part1 - {:?}", find_marker(&content, 4));
    println!("Part2 - {:?}", find_marker(&content, 14));
}
