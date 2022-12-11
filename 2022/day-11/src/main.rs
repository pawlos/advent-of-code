use std::fs;

#[derive(Debug)]
enum Operation {
    Add(u8),
    Mul(u8),
    Pow2,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test_val: u8,
    true_throw: usize,
    false_throw: usize,
    inspect_count: u32,
}

impl Monkey {
    fn do_op(&self, elem: u64, common_div: Option<u64>) -> u64 {
        match common_div {
            Some(v) => match self.op {
                Operation::Add(n) => (elem + n as u64) % v,
                Operation::Pow2 => (elem * elem) % v,
                Operation::Mul(n) => (elem * (n as u64)) % v,
            },
            None => match self.op {
                Operation::Add(n) => (elem + n as u64) / 3,
                Operation::Pow2 => (elem * elem) / 3,
                Operation::Mul(n) => (elem * (n as u64)) / 3,
            },
        }
    }

    fn check_stress(&self, elem: u64) -> bool {
        elem % (self.test_val as u64) == 0
    }

    fn do_round(&mut self, common_div: Option<u64>) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        loop {
            if self.items.is_empty() {
                break;
            }
            self.inspect_count += 1;
            let elem = self.items.remove(0); //drain
            let new_elem = self.do_op(elem, common_div);
            if self.check_stress(new_elem) {
                result.push((new_elem, self.true_throw));
            } else {
                result.push((new_elem, self.false_throw))
            }
        }

        result
    }
    fn parse_items(line: &str) -> Vec<u64> {
        let starting_items = line.trim().strip_prefix("Starting items: ").unwrap();

        starting_items
            .split(", ")
            .map(|el| el.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    }

    fn parse_test_val(line: &str) -> u8 {
        //Test: divisible by 23
        let test_val = line.trim().strip_prefix("Test: divisible by ").unwrap();
        test_val.parse::<u8>().unwrap()
    }

    fn append_item(&mut self, elem: u64) {
        self.items.push(elem);
    }

    fn parse_true_throw(line: &str) -> usize {
        //If true: throw to monkey 2
        let test_val = line
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap();
        test_val.parse::<usize>().unwrap()
    }

    fn parse_false_throw(line: &str) -> usize {
        //If false: throw to monkey 3
        let test_val = line
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap();
        test_val.parse::<usize>().unwrap()
    }

    fn parse_op(line: &str) -> Operation {
        let op = line.trim().strip_prefix("Operation: new = old ").unwrap();
        match op.split_once(' ') {
            Some(("+", n)) => Operation::Add(n.parse::<u8>().unwrap()),
            Some(("*", "old")) => Operation::Pow2,
            Some(("*", n)) => Operation::Mul(n.parse::<u8>().unwrap()),
            _ => panic!("oO"),
        }
    }

    pub fn parse(lines: Vec<&str>) -> Self {
        /*
        Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
         */

        Monkey {
            items: Self::parse_items(lines[1]),
            op: Self::parse_op(lines[2]),
            test_val: Self::parse_test_val(lines[3]),
            true_throw: Self::parse_true_throw(lines[4]),
            false_throw: Self::parse_false_throw(lines[5]),
            inspect_count: 0,
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("🎄 Ostra Piła - Advent of Code 2022 - Day 11 🎄");
    let content = fs::read_to_string("input-11.txt")?;

    let mut monkeys = content
        .split("\r\n\r\n")
        .map(|m| Monkey::parse(m.split("\r\n").collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    (0..20).for_each(|_| {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let items = m.do_round(None);
            for (item, m_idx) in items {
                monkeys[m_idx].append_item(item);
            }
        }
    });
    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys.reverse();

    let m1 = &monkeys[0..2];
    println!(
        "Part1 - {:?}",
        (m1[0].inspect_count as u128) * (m1[1].inspect_count as u128)
    );
    let common_div = Some(monkeys.iter().map(|m| m.test_val as u64).product());

    let mut monkeys = content
        .split("\r\n\r\n")
        .map(|m| Monkey::parse(m.split("\r\n").collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    (0..10_000).for_each(|_| {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let items = m.do_round(common_div);
            for (item, m_idx) in items {
                monkeys[m_idx].append_item(item);
            }
        }
    });
    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys.reverse();
    let m1 = &monkeys[0..2];
    println!(
        "Part2 - {:?}",
        (m1[0].inspect_count as u128) * (m1[1].inspect_count as u128)
    );
    Ok(())
}
