pub mod aoc {
    #[derive(Debug)]
    pub struct Instruction {
        pub count: usize,
        pub from: usize,
        pub to: usize,
    }

    impl Instruction {
        fn new(count: usize, from: usize, to: usize) -> Self {
            Instruction {
                from: from,
                to: to,
                count: count,
            }
        }

        pub fn parse_from(line: &str) -> Self {
            let values: Vec<&str> = line.split(' ').collect();
            Instruction::new(
                values[1].parse::<usize>().unwrap(),
                values[3].parse::<usize>().unwrap(),
                values[5].parse::<usize>().unwrap(),
            )
        }
    }
}
