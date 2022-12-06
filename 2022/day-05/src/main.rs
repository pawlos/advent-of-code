use crate::cntr::aoc::StackOfContainers;
use crate::crt::aoc::Container;
use crate::prsr::parser;

use crate::crn::aoc::move_containers;
use crate::crn::aoc::move_containers_part2;
mod cntr;
mod crn;
mod crt;
mod instrn;
mod prsr;

fn main() {
    let (mut crates, instr) = parser::read_from_file("input-05.txt");
    move_containers(instr, &mut crates);
    let mut s = String::new();
    for c in crates.iter_mut() {
        s += &c.pop().value.to_string();
    }
    println!("Part1: {:?}", s);
    let (mut crates2, instr) = parser::read_from_file("input-05.txt");
    move_containers_part2(instr, &mut crates2);
    let mut s = String::new();
    for c in crates2.iter_mut() {
        s += &c.pop().value.to_string();
    }
    println!("Part2: {:?}", s);
}
