use std::cmp::Ordering;
use std::collections::HashMap;

#[inline]
fn is_register(x: &str) -> bool {
    x.len() == 1 && ('a'..='z').contains(&x.chars().next().unwrap())
}

#[inline]
fn resolve(x: &str, registers: &HashMap<&str, i64>) -> i64 {
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

#[inline]
fn arithmetics(op: &str, lhs: i64, rhs: i64) -> i64 {
    match op {
        "add" => lhs + rhs,
        "sub" => lhs - rhs,
        "mul" => lhs * rhs,
        "div" => lhs / rhs,
        _ => panic!("invalid arithmetics op {op}"),
    }
}

#[inline]
fn need_jump(op: &str, ord: Ordering) -> bool {
    use Ordering::*;
    match op {
        "jne" => ord != Equal,
        "je" => ord == Equal,
        "jge" => ord == Equal || ord == Greater,
        "jg" => ord == Greater,
        "jle" => ord == Equal || ord == Less,
        "jl" => ord == Less,
        _ => panic!("invalid jump op {op}"),
    }
}

enum ArgKind {
    Text(String),
    Var(char),
}

pub struct AssemblerInterpreter {}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        let program = input
            .lines()
            .filter_map(|l| match l.trim() {
                x if x.len() > 0 && !x.starts_with(';') => match x.rsplit_once(";") {
                    None => Some(x),
                    Some((left, _)) => Some(left),
                },
                _ => None,
            })
            .map(|l| {
                if l.starts_with("msg") {
                    l.to_string()
                } else {
                    l.replace(',', "")
                }
            })
            .collect::<Vec<_>>();
        let labels = program
            .iter()
            .enumerate()
            .filter_map(|(i, l)| match l.split_once(':') {
                Some((v, "")) if v.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') => {
                    Some((v, i))
                }
                _ => None,
            })
            .collect::<HashMap<&str, usize>>();
        let mut registers: HashMap<&str, i64> = HashMap::new();
        let mut stack = vec![];
        let mut buf = Vec::with_capacity(3);
        let mut output: Option<String> = None;
        let mut last_cmp = None;
        let mut idx: usize = 0;
        loop {
            let inst = program.get(idx)?;
            buf.clear();
            buf.extend(inst.split_whitespace());
            match buf[..] {
                ["mov", x, y] if is_register(x) => {
                    registers.insert(x, resolve(y, &registers));
                    idx += 1;
                }
                ["inc", x] if is_register(x) => {
                    registers.entry(x).and_modify(|v| *v += 1);
                    idx += 1;
                }
                ["dec", x] if is_register(x) => {
                    registers.entry(x).and_modify(|v| *v -= 1);
                    idx += 1;
                }
                [op @ ("add" | "sub" | "mul" | "div"), x, y] if is_register(x) => {
                    let val = resolve(y, &registers);
                    registers
                        .entry(x)
                        .and_modify(|v| *v = arithmetics(op, *v, val));
                    idx += 1;
                }
                ["jmp", lbl] => idx = *labels.get(lbl)?,
                ["cmp", x, y] => {
                    last_cmp = Some(resolve(x, &registers).cmp(&resolve(y, &registers)));
                    idx += 1;
                }
                [op @ ("jne" | "je" | "jge" | "jg" | "jle" | "jl"), lbl] => {
                    if need_jump(op, last_cmp.take()?) {
                        idx = *labels.get(lbl)?;
                    } else {
                        idx += 1;
                    }
                }
                ["call", lbl] => {
                    stack.push(idx + 1);
                    idx = *labels.get(lbl)?;
                }
                ["ret"] => {
                    idx = stack.pop()?;
                }
                ["msg", ..] => {
                    let (_, rest) = inst.split_once("msg").unwrap();
                    let mut args = vec![];
                    let mut msg_buf = None;
                    for ch in rest.chars() {
                        // todo: make an iterator
                        match (ch, msg_buf.take()) {
                            ('\'', None) => msg_buf = Some(String::new()),
                            ('\'', Some(v)) => {
                                args.push(ArgKind::Text(v));
                                msg_buf = None;
                            }
                            (',' | ' ', None) => {}
                            (c, None) => args.push(ArgKind::Var(c)),
                            (c, Some(mut v)) => {
                                v.push(c);
                                msg_buf = Some(v);
                            }
                        }
                    }

                    let out_part = args
                        .into_iter()
                        .map(|arg| match arg {
                            ArgKind::Text(txt) => txt,
                            ArgKind::Var(c) => resolve(&c.to_string(), &registers).to_string(),
                        })
                        .collect::<String>();
                    output = match output.take() {
                        None => Some(out_part),
                        Some(mut v) => {
                            v.push_str(&out_part);
                            Some(v)
                        }
                    };
                    idx += 1;
                }
                ["end"] => break,
                _ => {
                    idx += 1;
                }
            }
        }
        output
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let simple_programs = &[
            "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
            "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
            "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
            "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
            "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
            "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
            "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n"];

        let expected = &[
            Some(String::from("(5+1)/2 = 3")),
            Some(String::from("5! = 120")),
            Some(String::from("Term 8 of Fibonacci series is: 21")),
            Some(String::from("mod(11, 3) = 2")),
            Some(String::from("gcd(81, 153) = 9")),
            None,
            Some(String::from("2^10 = 1024")),
        ];

        for (prg, exp) in simple_programs.iter().zip(expected) {
            let actual = AssemblerInterpreter::interpret(*prg);
            assert_eq!(actual, *exp);
        }
    }
}
