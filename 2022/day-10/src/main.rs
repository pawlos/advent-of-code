use std::fs;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop,
    Addx(i8),
}

fn signal_strength_at(n: usize, instructions: Vec<Instruction>) -> i32 {
    (1 + instructions
        .iter()
        .take(n - 1)
        .map(|i| match i {
            Instruction::Nop => 0,
            Instruction::Addx(n) => *n as i32,
        })
        .sum::<i32>())
        * (n as i32)
}

fn main() -> std::io::Result<()> {
    println!("🎄 Ostra Piła - Advent of Code 2022 - Day 10 🎄");
    let content = fs::read_to_string("input-10.txt")?;
    let instructions = content
        .lines()
        .flat_map(|l| match l {
            "noop" => vec![Instruction::Nop],
            _ => {
                let (_, n) = l.split_once(' ').unwrap();
                let n = n.parse::<i8>().unwrap();
                vec![Instruction::Nop, Instruction::Addx(n)]
            }
        })
        .collect::<Vec<_>>();

    println!(
        "Part1: {:?}",
        vec![20, 60, 100, 140, 180, 220]
            .iter()
            .map(|c| signal_strength_at(*c as usize, instructions.clone()))
            .inspect(|e| println!("{:?}", e))
            .sum::<i32>()
    );

    let mut x = 1i32;
    (0i32..240i32).for_each(|c| {
        let instr = instructions[c as usize];
        let cc = c % 40;
        if cc >= x - 1 && cc <= x + 1 {
            print!("🎄")
        } else {
            print!("🤍");
        }
        if cc == 39 {
            println!();
        }
        x += match instr {
            Instruction::Nop => 0,
            Instruction::Addx(n) => n as i32,
        }
    });
    println!();
    Ok(())
}
