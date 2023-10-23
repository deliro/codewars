use std::collections::HashMap;
use std::iter::repeat;

use itertools::Itertools;

#[inline]
fn to_bits(val: u8) -> [u8; 8] {
    [
        (val >> 7) & 1,
        (val >> 6) & 1,
        (val >> 5) & 1,
        (val >> 4) & 1,
        (val >> 3) & 1,
        (val >> 2) & 1,
        (val >> 1) & 1,
        val & 1,
    ]
}

#[inline(always)]
fn byte_bit(x: u64) -> (u64, u64) {
    (x / 64, x % 64)
}

#[inline]
fn bit_at(tape: &HashMap<u64, u64>, ptr: u64) -> u8 {
    let (byte, bit) = byte_bit(ptr);
    let byte_val = *tape.get(&byte).unwrap_or(&0);
    ((byte_val >> bit) & 1) as u8
}

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut input = input
        .into_iter()
        .map(|v| to_bits(v).into_iter().rev())
        .flatten()
        .chain(repeat(0));
    let instructions: Vec<_> = code.chars().collect();
    let mut tape: HashMap<u64, u64> = HashMap::new();
    let mut tape_ptr: u64 = 0;
    let mut inst_ptr = 0;
    let mut output: Vec<u8> = vec![];
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
            '>' => {
                tape_ptr = tape_ptr.overflowing_add(1).0;
            }
            '<' => {
                tape_ptr = tape_ptr.overflowing_sub(1).0;
            }
            '+' => {
                let (byte, bit) = byte_bit(tape_ptr);
                tape.entry(byte)
                    .and_modify(|v| *v = *v ^ (1 << bit))
                    .or_insert(1 << bit);
            }
            ';' => output.push(bit_at(&tape, tape_ptr)),
            ',' => {
                let (byte, bit) = byte_bit(tape_ptr);
                let val = (input.next().unwrap() as u64) << bit;
                tape.entry(byte).and_modify(|v| *v |= val).or_insert(val);
            }
            '[' => {
                if bit_at(&tape, tape_ptr) == 0 {
                    inst_ptr = brackets[&inst_ptr];
                }
            }

            ']' => {
                if bit_at(&tape, tape_ptr) != 0 {
                    inst_ptr = brackets[&inst_ptr];
                }
            }
            _ => {}
        }
        inst_ptr += 1;
    }

    if output.len() % 8 != 0 {
        let aligned = ((output.len() / 8) + 1) * 8;
        (0..aligned - output.len()).for_each(|_| output.push(0));
    }

    output
        .into_iter()
        .chunks(8)
        .into_iter()
        .map(|chunk| chunk.zip(0..8).map(|(bit, shift)| bit << shift).sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_cases() {
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}
