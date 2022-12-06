use std::{fs, ops::Range};

#[derive(Debug)]
struct Camp {
    first_elf: Range<u16>,
    second_elf: Range<u16>,
}
trait RangeCompare {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps_range(&self, other: &Self) -> bool;
}

impl<T: Ord> RangeCompare for Range<T> {
    fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        self.start <= other.end && self.start >= other.start
            || self.end >= other.start && self.end <= other.end
    }
}

impl Camp {
    fn is_overlapping(&self) -> bool {
        self.first_elf.contains_range(&self.second_elf)
            || self.second_elf.contains_range(&self.first_elf)
    }

    fn is_partially_overlapping(&self) -> bool {
        self.first_elf.overlaps_range(&self.second_elf)
            || self.second_elf.overlaps_range(&self.first_elf)
    }
}

fn main() {
    let content = fs::read_to_string("input-04.txt").unwrap();
    let lines = content.lines();
    let raw_camps = lines.map(|l| l.split(',').collect());

    let camps = raw_camps.map(|e: Vec<_>| {
        let (first_start, first_end) = e[0].split_once('-').unwrap();
        let (second_start, second_end) = e[1].split_once('-').unwrap();
        Camp {
            first_elf: Range {
                start: first_start.parse::<u16>().unwrap(),
                end: first_end.parse::<u16>().unwrap(),
            },
            second_elf: Range {
                start: second_start.parse::<u16>().unwrap(),
                end: second_end.parse::<u16>().unwrap(),
            },
        }
    });

    let contains: Vec<_> = camps
        .map(|c| (c.is_overlapping(), c.is_partially_overlapping()))
        .collect();
    println!("Part1: {:?}", contains.iter().filter(|i| i.0).count());
    println!("Part2: {:?}", contains.iter().filter(|i| i.1).count());
}
