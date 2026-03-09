use crate::instruction::Instruction;

pub fn parse(source: &Vec<u8>) -> Vec<Instruction> {
    let mut pc: usize = 0;
    let mut program = Vec::new();

    while pc < source.len() {
        match source[pc] {
            b' ' => parse_stack(source, &mut pc, &mut program),
            b'\t' => parse_tab(source, &mut pc, &mut program),
            b'\n' => parse_flow(source, &mut pc, &mut program),
            _ => {}
        }
    }
    program
}

fn parse_stack(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;

    if *pc >= chars.len() {
        return;
    }
    match chars[*pc] {
        b' ' => {
            *pc += 1;
            let value = parse_number(chars, pc);
            program.push(Instruction::Push(value));
        },

        b'\n' => {
            *pc += 1;
            match chars[*pc] {
                b' ' => program.push(Instruction::Dup),
                b'\t' => program.push(Instruction::Swap),
                b'\n' => program.push(Instruction::Drop),
                _ => panic!("invalid instruction"),
            }
            *pc += 1;
        },

        _ => { panic!("invalid stack instruction"); }
    }
}

fn parse_tab(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;

    if *pc >= chars.len() {
        return;
    }
    match chars[*pc] {
        b' ' => parse_arithmetic(chars, pc, program),
        b'\t' => parse_heap_access(chars, pc, program),
        b'\n' => parse_io(chars, pc, program),
        _ => { panic!("invalid instruction"); },
    }
}

fn parse_arithmetic(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;
    if *pc >= chars.len() {
        return;
    }
    let c1 = chars[*pc];

    *pc += 1;
    if *pc >= chars.len() {
        return;
    }
    let c2 = chars[*pc];

    match (c1, c2) {
        (b' ', b' ') => program.push(Instruction::Add),
        (b' ', b'\t') => program.push(Instruction::Sub),
        (b' ', b'\n') => program.push(Instruction::Mul),
        (b'\t', b' ') => program.push(Instruction::Div),
        (b'\t', b'\t') => program.push(Instruction::Mod),
        _ => { panic!("invalid arithmetic instruction"); },
    }
}

fn parse_heap_access(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;

    if *pc >= chars.len() {
        return;
    }
    match chars[*pc] {
        b' ' => program.push(Instruction::Store),
        b'\t' => program.push(Instruction::Load),
        _ => panic!("invalid heap instruction"),
    }

    *pc += 1;
}

fn parse_io(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;
    if *pc >= chars.len() {
        return;
    }
    let c1 = chars[*pc];

    *pc += 1;
    if *pc >= chars.len() {
        return;
    }
    let c2 = chars[*pc];

    *pc += 1;

    match (c1, c2) {
        (b' ', b' ') => program.push(Instruction::PrintChar),
        (b' ', b'\t') => program.push(Instruction::PrintNum),
        (b'\t', b' ') => program.push(Instruction::ReadChar),
        (b'\t', b'\t') => program.push(Instruction::ReadNum),
        _ => panic!("invalid IO instruction"),
    } 
}

fn parse_flow(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;
    if *pc >= chars.len() {
        return;
    }
    let c1 = chars[*pc];

    *pc += 1;
    if *pc >= chars.len() {
        return;
    }
    let c2 = chars[*pc];

    *pc += 1;

    match (c1, c2) {
        (b' ', b' ') => {
            let label = parse_label(chars, pc);
            program.push(Instruction::Label(label));
        },
        (b' ', b'\t') => {
            let label = parse_label(chars, pc);
            program.push(Instruction::Call(label));
        },
        (b' ', b'\n') => {
            let label = parse_label(chars, pc);
            program.push(Instruction::Jump(label));
        },
        (b'\t', b' ') => {
            let label = parse_label(chars, pc);
            program.push(Instruction::JumpZero(label));
        },
        (b'\t', b'\t') => {
            let label = parse_label(chars, pc);
            program.push(Instruction::JumpNeg(label));
        },
        (b'\t', b'\n') => {
            program.push(Instruction::Return);
        },
        (b'\n', b'\n') => {
            program.push(Instruction::End);
        },
        _ => { panic!("invalid flow control instruction") },

    }
}

fn parse_number(chars: &Vec<u8>, pc: &mut usize) -> i64 {
    if *pc >= chars.len() {
        panic!("could not parse number");
    }
    let sign = if chars[*pc] == b' ' { 1 } else { -1 };
    *pc += 1;
    if *pc >= chars.len() {
        return sign;
    }
    
    let mut value = 0;
    while chars[*pc] != b'\n' {
        value = value * 2; 

        if chars[*pc] == b'\t' {
            value += 1;
        }

        *pc += 1;
        if *pc >= chars.len() {
            panic!("No \\n terminating number");
        }
    }

    *pc += 1;


    return sign * value;
        
}

fn parse_label(chars: &Vec<u8>, pc: &mut usize) -> u64 {
    if *pc >= chars.len() {
        panic!("could not parse label");
    }

    let mut value = 0;
    while chars[*pc] != b'\n' {
        value = value * 2;

        if chars[*pc] == b'\t' {
            value += 1;
        }

        *pc += 1;
        if *pc >= chars.len() {
            panic!("No \\n terminating label");
        }
    }

    *pc += 1;

    return value;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_push() {
        let input = vec![b' ', b' ', b' ', b'\t', b' ', b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Push(2)]);
    }

    #[test]
    fn parse_push_number() {
        let input = vec![
            b' ', b' ', 
            b' ',       // positive
            b'\t', b' ', // binary 10
            b'\n'
        ];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Push(2)]);
    }

    #[test]
    fn parse_duplicate_number() {
        let input = vec![b' ', b'\n', b' '];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Dup]);

    }
    
    #[test]
    fn parse_swap_numbers() {
        let input = vec![b' ', b'\n', b'\t'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Swap]);
    }

    #[test]
    fn parse_drop_number() {
        let input = vec![b' ', b'\n', b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Drop]);
    }

    #[test]
    fn parse_add() {
        let input = vec![b'\t', b' ', b' ', b' '];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Add]);
    }

    #[test]
    fn parse_sub() {
        let input = vec![b'\t', b' ', b' ', b'\t'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Sub]);
    }

    #[test]
    fn parse_mul() {
        let input = vec![b'\t', b' ', b' ', b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Mul]);
    }

    #[test]
    fn parse_integer_div() {
        let input = vec![b'\t', b' ', b'\t', b' '];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Div]);
    }

    #[test]
    fn parse_modulo() {
        let input = vec![b'\t', b' ', b'\t', b'\t'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Mod]);
    }

    #[test]
    fn parse_store() {
        let input = vec![b'\t', b'\t', b' '];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Store]);
    }

    #[test]
    fn parse_read() {
        let input = vec![b'\t', b'\t', b'\t'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Load]);
    }

    #[test]
    fn parse_set_label() {
        let input = vec![b'\n', b' ', b' ',
                         b' ', b' ', b' ', // 0 binary
                         b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Label(0)]); 
    }

    #[test]
    fn parse_call() {
        let input = vec![b'\n', b' ', b'\t',
                         b'\t', b' ', b' ', // 4 binary
                         b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Call(4)]);
    }

    #[test]
    fn parse_jump() {
        let input = vec![b'\n', b' ', b'\n',
                         b'\t', b'\t', b' ', // 6 binary
                         b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Jump(6)]);
    }

    #[test]
    fn parse_jump_zero() {
        let input = vec![b'\n', b'\t', b' ',
                         b'\t', b' ', b'\t', // 5 binary
                         b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::JumpZero(5)]);
    }

    #[test]
    fn parse_jump_negative() {
        let input = vec![b'\n', b'\t', b'\t',
                         b' ', b' ', b'\t', // 1 binary
                         b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::JumpNeg(1)]);
    }

    #[test]
    fn parse_end() {
        let input = vec![b'\n', b'\n', b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::End]);
    }

    #[test]
    fn parse_return() {
        let input = vec![b'\n', b'\t', b'\n'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::Return]);
    }

    #[test]
    fn parse_output_char() {
        let input = vec![b'\t', b'\n', b' ', b' '];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::PrintChar]);
    }

    #[test]
    fn parse_output_number() {
        let input = vec![b'\t', b'\n', b' ', b'\t'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::PrintNum]);
    }

    #[test]
    fn parse_read_char() {
        let input = vec![b'\t', b'\n', b'\t', b' '];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::ReadChar]);
    }

    #[test]
    fn parse_read_number() {
        let input = vec![b'\t', b'\n', b'\t', b'\t'];
        let program = parse(&input);
        assert_eq!(program, vec![Instruction::ReadNum]);
    }


    #[test]
    fn test_parse_number() {
        let input = vec![
            b' ', b'\n',
            b' ', b'\t', b'\n',
            b'\t', b'\n',
            b'\t', b'\t', b'\n',
            b' ', b'\t', b'\t', b'\n'
        ];
        let mut pc: usize = 0;
        let mut num = parse_number(&input, &mut pc);
        assert_eq!(num, 0);
        num = parse_number(&input, &mut pc);
        assert_eq!(num, 1);
        num = parse_number(&input, &mut pc);
        assert_eq!(num, 0);
        num = parse_number(&input, &mut pc);
        assert_eq!(num, -1);
        num = parse_number(&input, &mut pc);
        assert_eq!(num, 3);
    }

    #[test]
    fn test_parse_label() {
        let input = vec![
            b' ', b'\n',
            b' ', b'\t', b'\n',
            b'\t', b'\n',
            b'\t', b'\t', b'\n',
            b' ', b'\t', b'\t', b'\n'
        ];
        let mut pc: usize = 0;
        let mut num = parse_label(&input, &mut pc);
        assert_eq!(num, 0);
        num = parse_label(&input, &mut pc);
        assert_eq!(num, 1);
        num = parse_label(&input, &mut pc);
        assert_eq!(num, 1);
        num = parse_label(&input, &mut pc);
        assert_eq!(num, 3);
        num = parse_label(&input, &mut pc);
        assert_eq!(num, 3);
    }



    #[test]
    fn parse_simple_program() {
        let input = vec![
            b' ', b' ', b' ', b'\t', b'\n', // push 1
            b' ', b'\n', b' ',              // dup
        ];

        let program = parse(&input);

        println!("{:?}", program);
    }
}
