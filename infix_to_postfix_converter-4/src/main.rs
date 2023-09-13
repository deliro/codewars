#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    LeftPar,
    RightPar,
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
}

impl Op {
    fn precedence(&self) -> u8 {
        match self {
            Op::LeftPar | Op::RightPar => 0,
            Op::Plus | Op::Minus => 1,
            Op::Multiply | Op::Divide => 2,
            Op::Exponent => 3,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Op::LeftPar => '(',
            Op::RightPar => ')',
            Op::Plus => '+',
            Op::Minus => '-',
            Op::Multiply => '*',
            Op::Divide => '/',
            Op::Exponent => '^',
        }
    }

    fn left_associative(&self) -> bool {
        match self {
            Op::Exponent => false,
            _ => true,
        }
    }
}

impl TryFrom<char> for Op {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Op::LeftPar),
            ')' => Ok(Op::RightPar),
            '+' => Ok(Op::Plus),
            '-' => Ok(Op::Minus),
            '*' => Ok(Op::Multiply),
            '/' => Ok(Op::Divide),
            '^' => Ok(Op::Exponent),
            _ => Err(()),
        }
    }
}

fn to_postfix(infix: &str) -> String {
    let mut op_stack: Vec<Op> = vec![];
    let mut output = String::new();
    infix.chars().for_each(|c| {
        if let Ok(op) = Op::try_from(c) {
            match op {
                Op::LeftPar => op_stack.push(op),
                Op::RightPar => {
                    while let Some(x) = op_stack.pop() {
                        if x == Op::LeftPar {
                            break;
                        } else {
                            output.push(x.to_char())
                        }
                    }
                }
                _ => {
                    while let Some(x) = op_stack.last() {
                        if x.precedence() > op.precedence()
                            || (x.precedence() == op.precedence() && op.left_associative())
                        {
                            output.push(x.to_char());
                            op_stack.pop();
                        } else {
                            break;
                        }
                    }
                    op_stack.push(op);
                }
            }
        } else {
            assert!(c.is_digit(10));
            output.push(c);
        }
    });

    while let Some(op) = op_stack.pop() {
        output.push(op.to_char())
    }
    output
}
