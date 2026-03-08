use std::env;
use std::fs;

enum Instruction {
    // Stack namipulation
    Push(i64),
    Dup,
    Swap,
    Drop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Heap Access
    Store,
    Load,

    // Flow Control
    Label(u64),
    Call(u64),
    Jump(u64),
    JumpZero(u64),
    JumpNeg(u64),
    Return,
    End,

    // I/O
    PrintChar,
    PrintNum,
    ReadChar,
    ReadNum,
}

fn parse(source: &Vec<u8>) -> Vec<Instruction> {
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

    match chars[*pc] {
        b' ' => parse_arithmetic(chars, pc, program),
        b'\t' => parse_heap_access(chars, pc, program),
        b'\n' => parse_io(chars, pc, program),
        _ => { panic!("invalid instruction"); },
    }
}

fn parse_arithmetic(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;
    let c1 = chars[*pc];

    *pc += 1;
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

    match chars[*pc] {
        b' ' => program.push(Instruction::Store),
        b'\t' => program.push(Instruction::Load),
        _ => panic!("invalid heap instruction"),
    }

    *pc += 1;
}

fn parse_io(chars: &Vec<u8>, pc: &mut usize, program: &mut Vec<Instruction>) {
    *pc += 1;
    let c1 = chars[*pc];

    *pc += 1;
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
    let c1 = chars[*pc];

    *pc += 1;
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
    let sign = if chars[*pc] == b' ' { 1 } else { -1 };
    *pc += 1;
    
    let mut value = 0;
    while chars[*pc] != b'\n' {
        value = value * 2;

        if chars[*pc] == b'\t' {
            value += 1;
        }

        *pc += 1;
    }

    *pc += 1;


    return sign * value;
        
}

fn parse_label(chars: &Vec<u8>, pc: &mut usize) -> u64 {

    let mut value = 0;
    while chars[*pc] != b'\n' {
        value = value * 2;

        if chars[*pc] == b'\t' {
            value += 1;
        }

        *pc += 1;
    }

    *pc += 1;

    return value;
}

fn read_file(filename: String) -> Vec<u8> {
    let data = fs::read(filename);
    if data.is_err() {
        return Vec::<u8>::new();
    }
    return data
        .ok()
        .unwrap()
        .into_iter()
        .filter(|c| *c == b' ' || *c == b'\t' || *c == b'\n')
        .collect();
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Expected file input");
        return;
    }

    let filename = &args[1];
    let whitespace_source = read_file(filename.to_string());

    let _instructions = parse(&whitespace_source);

}
