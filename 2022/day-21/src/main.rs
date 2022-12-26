use std::{
    fmt::{format, Display},
    fs,
};

#[derive(Debug)]
enum Operation {
    Number(f64),
    Compute(String),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    to_do: Operation,
}

enum Value {
    Expression(String),
    Const(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Expression(x) => write!(f, "{}", x),
            Value::Const(x) => write!(f, "{}", x),
        }
    }
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("input-21.txt")?;
    let monkeys: Vec<Monkey> = content
        .split("\r\n")
        .map(|l| {
            let (name, to_do) = l.split_once(": ").unwrap();
            let name = name.to_string();
            match to_do.contains(' ') {
                true => Monkey {
                    name,
                    to_do: Operation::Compute(to_do.to_string()),
                },
                false => Monkey {
                    name,
                    to_do: Operation::Number(to_do.parse::<f64>().unwrap()),
                },
            }
        })
        .collect::<Vec<_>>();

    let root = monkeys.iter().find(|m| m.name == "root").unwrap();
    let result = compute(&monkeys, root);
    println!("Part1 - {:?}", result);

    let root = monkeys.iter().find(|m| m.name == "root").unwrap();
    /*for i in 0u64..u64::MAX {
        if i % 100000 == 0 {
            println!(".");
        }
        let (result, ok) = compute2(&monkeys, root, i);
        if ok {
            println!("Part2 - {:?}", result);
            break;
        }
    }*/
    let eq = compute2(&monkeys, root);
    println!("{}", eq);
    Ok(())
}

fn compute2(monkeys: &[Monkey], elem: &Monkey) -> Value {
    match &elem.to_do {
        Operation::Number(x) => match elem.name.as_str() {
            "humn" => Value::Expression("x".to_string()),
            _ => Value::Const(*x),
        },
        Operation::Compute(op) => {
            let ops = op.split(' ').collect::<Vec<_>>();

            let lhs_monkey = monkeys.iter().find(|m| m.name == ops[0]).unwrap();
            let rhs_monkey = monkeys.iter().find(|m| m.name == ops[2]).unwrap();
            let lhs = compute2(monkeys, lhs_monkey);
            let rhs = compute2(monkeys, rhs_monkey);

            if elem.name == "root" {
                return Value::Expression(format!("{} == {}", lhs, rhs));
            }

            match ops[1].chars().next().unwrap() {
                '+' => match (&lhs, &rhs) {
                    (Value::Const(x), Value::Const(y)) => Value::Const(x + y),
                    _ => Value::Expression(format!("({} + {})", lhs, rhs)),
                },
                '-' => match (&lhs, &rhs) {
                    (Value::Const(x), Value::Const(y)) => Value::Const(x - y),
                    _ => Value::Expression(format!("({} - {})", lhs, rhs)),
                },
                '*' => match (&lhs, &rhs) {
                    (Value::Const(x), Value::Const(y)) => Value::Const(x * y),
                    _ => Value::Expression(format!("({} * {})", lhs, rhs)),
                },
                '/' => match (&lhs, &rhs) {
                    (Value::Const(x), Value::Const(y)) => Value::Const(x / y),
                    _ => Value::Expression(format!("({} / {})", lhs, rhs)),
                },
                _ => panic!("unknown operator"),
            }
        }
    }
}

fn compute(monkeys: &Vec<Monkey>, root: &Monkey) -> f64 {
    match &root.to_do {
        Operation::Number(x) => *x,
        Operation::Compute(op) => {
            let ops = op.split(' ').collect::<Vec<_>>();
            //println!("{:?}", ops);
            let lhs_monkey = monkeys.iter().find(|m| m.name == ops[0]).unwrap();
            let rhs_monkey = monkeys.iter().find(|m| m.name == ops[2]).unwrap();
            let lhs = compute(monkeys, lhs_monkey);
            let rhs = compute(monkeys, rhs_monkey);

            match ops[1].chars().next().unwrap() {
                '+' => lhs + rhs,
                '-' => lhs - rhs,
                '*' => lhs * rhs,
                '/' => lhs / rhs,
                _ => panic!("unknown operator"),
            }
        }
    }
}
