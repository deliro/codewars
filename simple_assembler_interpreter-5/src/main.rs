use std::collections::HashMap;

#[inline]
fn is_register(x: &str) -> bool {
    x.len() == 1 && ('a'..='z').contains(&x.chars().next().unwrap())
}

#[inline]
fn resolve(x: &str, registers: &HashMap<String, i64>) -> i64 {
    if is_register(x) {
        registers
            .get(x)
            .expect("register is empty but is requested")
            .clone()
    } else {
        x.parse::<i64>()
            .expect("value must be either a register or a number")
    }
}
fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers: HashMap<String, i64> = HashMap::new();
    let mut buf = Vec::with_capacity(3);
    let mut idx = 0;
    while idx < program.len() {
        buf.clear();
        buf.extend(program[idx].split_ascii_whitespace());
        match buf[..] {
            ["mov", x, y] if is_register(x) => {
                registers.insert(x.to_string(), resolve(y, &registers));
                idx += 1;
            }
            ["inc", x] if is_register(x) => {
                registers.entry(x.to_string()).and_modify(|v| *v += 1);
                idx += 1;
            }
            ["dec", x] if is_register(x) => {
                registers.entry(x.to_string()).and_modify(|v| *v -= 1);
                idx += 1;
            }
            ["jnz", x, y] => {
                let steps = y.parse::<i64>().expect("steps are expected to be integer");
                if resolve(x, &registers) != 0 {
                    if steps > 0 {
                        idx += steps as usize
                    } else if steps < 0 {
                        idx -= (-steps) as usize
                    }
                } else {
                    idx += 1;
                }
            }
            _ => panic!("unknown instruction"),
        }
    }
    registers
}
