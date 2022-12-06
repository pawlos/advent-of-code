pub mod aoc {
    use crate::instrn::aoc::Instruction;
    use crate::StackOfContainers;

    pub fn move_containers(instrs: Vec<Instruction>, stack: &mut Vec<StackOfContainers>) {
        for instr in instrs {
            let f = &mut stack[instr.from - 1];
            let v = f.pop_many(instr.count);
            let t = &mut stack[instr.to - 1];
            t.push_many(v);
        }
    }

    pub fn move_containers_part2(instrs: Vec<Instruction>, stack: &mut Vec<StackOfContainers>) {
        for instr in instrs {
            let f = &mut stack[instr.from - 1];
            let mut v = f.pop_many(instr.count);
            v.reverse();
            let t = &mut stack[instr.to - 1];

            t.push_many(v);
        }
    }
}
