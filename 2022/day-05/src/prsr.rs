/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
 */
pub mod parser {
    use crate::instrn::aoc::Instruction;
    use crate::{Container, StackOfContainers};
    use std::fs;

    pub fn read_from_file(file_name: &str) -> (Vec<StackOfContainers>, Vec<Instruction>) {
        let mut temp: Vec<Vec<Container>> = Vec::new();
        let mut instr: Vec<Instruction> = Vec::new();

        for (_index, line) in fs::read_to_string(file_name).unwrap().lines().enumerate() {
            if line.contains('[') {
                let containers: _ = line
                    .as_bytes()
                    .chunks(4)
                    .map(|c| Container::new(std::str::from_utf8(c).unwrap()))
                    .collect();
                temp.push(containers);
            }
            if line.contains("move") {
                instr.push(Instruction::parse_from(line))
            }
        }

        temp.reverse();
        let count = temp[0].len();
        let mut result: Vec<StackOfContainers> = Vec::new();

        for _ in 0..count {
            result.push(StackOfContainers::new());
        }
        for row in temp {
            for (idx, container) in row.iter().enumerate() {
                result[idx].push(*container)
            }
        }
        (result, instr)
    }
}
