use std::collections::HashMap;

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut input = input.into_iter();
    let mut tape_ptr = 0;
    let mut inst_ptr = 0;
    let mut tape: HashMap<i32, u8> = HashMap::new();
    let mut output: Vec<u8> = vec![];
    let instructions: Vec<_> = code.chars().collect();
    let brackets = {
        let mut brackets_stack = vec![];
        let mut brackets = HashMap::new();
        for (i, ch) in instructions
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '[' || **c == ']')
        {
            if *ch == ']' {
                let (ch1, i1) = brackets_stack.pop().unwrap();
                assert_eq!(ch1, '[');
                brackets.insert(i1, i);
                brackets.insert(i, i1);
            } else {
                brackets_stack.push((*ch, i))
            }
        }
        assert_eq!(brackets_stack.len(), 0);
        brackets
    };

    while inst_ptr < instructions.len() {
        match instructions[inst_ptr] {
            '>' => tape_ptr += 1,
            '<' => tape_ptr -= 1,
            '+' => {
                tape.entry(tape_ptr)
                    .and_modify(|v| *v = v.overflowing_add(1).0)
                    .or_insert(1);
            }

            '-' => {
                tape.entry(tape_ptr)
                    .and_modify(|v| *v = v.overflowing_sub(1).0)
                    .or_insert(255);
            }
            '.' => output.push(*tape.get(&tape_ptr).unwrap_or(&0)),
            ',' => {
                tape.insert(tape_ptr, input.next().unwrap());
            }
            '[' => {
                if *tape.get(&tape_ptr).unwrap_or(&0) == 0 {
                    inst_ptr = brackets[&inst_ptr];
                }
            }

            ']' => {
                if *tape.get(&tape_ptr).unwrap_or(&0) != 0 {
                    inst_ptr = brackets[&inst_ptr];
                }
            }
            _ => panic!("unknown instruction"),
        }
        inst_ptr += 1;
    }
    output
}
