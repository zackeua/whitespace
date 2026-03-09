use std::collections::HashMap;
use std::io::{self, Read, Write};

use crate::instruction::Instruction;

pub struct VM {
    stack: Vec<i64>,
    heap: HashMap<i64, i64>,
    call_stack: Vec<usize>,
    labels: HashMap<u64, usize>,
    pc: usize,
}

impl VM {
    fn new(program: &Vec<Instruction>) -> Self {
        let mut labels = HashMap::new();

        for (i, instruction) in program.iter().enumerate() {
            if let Instruction::Label(id) = instruction {
                labels.insert(*id, i);
            }
        }

        VM {
            stack: Vec::new(),
            heap: HashMap::new(),
            call_stack: Vec::new(),
            labels,
            pc: 0,
        }
    }
}

pub fn interpret(program: &Vec<Instruction>) -> VM {
    let mut vm = VM::new(program);
    run(program, &mut vm);
    vm
}

fn run(program: &Vec<Instruction>, vm: &mut VM) {
    while vm.pc < program.len() {
        match program[vm.pc] {

            Instruction::Push(n) => vm.stack.push(n),
            Instruction::Dup => {
                let a = vm.stack.pop().expect("stack underflow");
                vm.stack.push(a);
                vm.stack.push(a);
            },
            Instruction::Swap => {
                let b = vm.stack.pop().expect("stack underflow");
                let a = vm.stack.pop().expect("stack underflow");
                vm.stack.push(b);
                vm.stack.push(a);
            },
            Instruction::Drop => { vm.stack.pop().expect("stack underflow"); },

            Instruction::Add => {
                let b = vm.stack.pop().expect("stack underflow");
                let a = vm.stack.pop().expect("stack underflow");
                vm.stack.push(a + b);
            },
            Instruction::Sub => {
                let b = vm.stack.pop().expect("stack underflow");
                let a = vm.stack.pop().expect("stack underflow");
                vm.stack.push(a - b);
            },
            Instruction::Mul => {
                let b = vm.stack.pop().expect("stack underflow");
                let a = vm.stack.pop().expect("stack underflow");
                vm.stack.push(a * b);
            },
            Instruction::Div => {
                let b = vm.stack.pop().expect("stack underflow");
                let a = vm.stack.pop().expect("stack underflow");
                vm.stack.push(a / b);
            },
            Instruction::Mod => {
                let b = vm.stack.pop().expect("stack underflow");
                let a = vm.stack.pop().expect("stack underflow");
                let modulo = ((a % b) + b) % b;
                vm.stack.push(modulo);
            },

            Instruction::Store => {
                let value = vm.stack.pop().expect("Stack underflow");
                let address = vm.stack.pop().expect("stack underflow");
                vm.heap.insert(address, value);
            },
            Instruction::Load => {
                let address = vm.stack.pop().expect("stack underflow");
                let value = *vm.heap.get(&address).unwrap_or(&0);
                vm.stack.push(value);
            },

            Instruction::Label(_) => {
                // Labels do nothing during execution
            },
            Instruction::Call(n) => {
                let target = *vm.labels.get(&n).expect("unknown label");
                // Save return address
                vm.call_stack.push(vm.pc + 1);
                vm.pc = target;
                continue;
            },
            Instruction::Jump(n) => {
                let target = *vm.labels.get(&n).expect("unknown label");
                vm.pc = target;
            },
            Instruction::JumpZero(n) => {
                let value = vm.stack.pop().expect("stack underflow");

                if value == 0 {
                    let target = *vm.labels.get(&n).expect("unknown label");
                    vm.pc = target;
                    continue;
                }
            },
            Instruction::JumpNeg(n) => {
                let value = vm.stack.pop().expect("stack underflow");

                if value < 0 {
                    let target = *vm.labels.get(&n).expect("unknown label");
                    vm.pc = target;
                    continue;
                }

            },
            Instruction::Return => {
                vm.pc = vm.call_stack.pop().expect("call stack underflow");
                continue;
            },
            Instruction::End => { return; },

            Instruction::PrintChar => {
                let value = vm.stack.pop().expect("stack underflow");

                let c = char::from_u32(value as u32).expect("failed conversion to char");
                print!("{}", c);
            },
            Instruction::PrintNum => {
                let value = vm.stack.pop().expect("stack underflow");
                print!("{}", value);
            },
            Instruction::ReadChar => {
                let address = vm.stack.pop().expect("stack underflow");

                let mut buffer = [0u8; 1];
                flush_stdout();
                io::stdin()
                    .read_exact(&mut buffer)
                    .expect("failed to read char");

                vm.heap.insert(address, buffer[0] as i64);
            },
            Instruction::ReadNum => {
                let address = vm.stack.pop().expect("stack underflow");
                let mut input = String::new();
                flush_stdout();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read number");

                let number: i64 = input
                    .trim()
                    .parse()
                    .expect("invalid integer");

                vm.heap.insert(address, number);
            },

            // _ => todo!()
        }
        vm.pc += 1;
    }
}

fn flush_stdout() {
    io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let input = vec![Instruction::Push(2)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![2]);
    }

    #[test]
    fn test_dup() {
        let input = vec![Instruction::Push(5), Instruction::Dup];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![5, 5]);
    }

    #[test]
    fn test_swap() {
        let input = vec![Instruction::Push(2), Instruction::Push(1), Instruction::Swap];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![1, 2]);
    }

    #[test]
    fn test_drop() {
        let input = vec![Instruction::Push(123), Instruction::Push(321), Instruction::Drop];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![123]);
    }

    #[test]
    fn test_add() {
        let input = vec![Instruction::Push(1), Instruction::Push(2), Instruction::Add];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![3]);
    }

    #[test]
    fn test_sub() {
        let input = vec![Instruction::Push(0), Instruction::Push(10), Instruction::Sub];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-10]);
    }

    #[test]
    fn test_mul() {
        let input = vec![Instruction::Push(2), Instruction::Push(3), Instruction::Mul];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![6]);
    }

    #[test]
    fn test_div() {
        let input = vec![Instruction::Push(11), Instruction::Push(5), Instruction::Div];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![2]);
    }

    #[test]
    fn test_mod() {
        let input = vec![Instruction::Push(11), Instruction::Push(5), Instruction::Mod];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![1]);
    }

    #[test]
    fn test_store() {
        let input = vec![
            Instruction::Push(100),
            Instruction::Push(-10),
            Instruction::Store
        ];
        let vm = interpret(&input);
        assert_eq!(*vm.heap.get(&100).unwrap(), -10);
    }

    #[test]
    fn test_load() {
        let input = vec![
            Instruction::Push(100),
            Instruction::Push(-10),
            Instruction::Store,
            Instruction::Push(100),
            Instruction::Load
        ];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-10]);
    }

    #[test]
    fn test_label() {
        let input = vec![Instruction::Label(1), Instruction::Push(2), Instruction::Label(3)];
        let vm = interpret(&input);
        assert_eq!(*vm.labels.get(&1).unwrap(), 0);
        assert_eq!(*vm.labels.get(&3).unwrap(), 2);
        assert_eq!(vm.stack, vec![2]);
    }

    #[test]
    fn test_call() {
        let input = vec![Instruction::Call(1), Instruction::End, Instruction::Push(3), Instruction::Label(1), Instruction::Push(2), Instruction::Return];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![2]);
    }

    #[test]
    fn test_jump() {
        let input = vec![Instruction::Jump(2), Instruction::Push(2), Instruction::Label(2), Instruction::Push(1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![1]);
    }

    #[test]
    fn test_jump_zero_1() {
        let input = vec![Instruction::Push(0), Instruction::JumpZero(100), Instruction::Push(-10), Instruction::End, Instruction::Label(100), Instruction::Push(-1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-1]);
    }

    #[test]
    fn test_jump_zero_2() {
        let input = vec![Instruction::Push(1), Instruction::JumpZero(100), Instruction::Push(-10), Instruction::End, Instruction::Label(100), Instruction::Push(-1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-10]);
    }

    #[test]
    fn test_jump_zero_3() {
        let input = vec![Instruction::Push(-1), Instruction::JumpZero(100), Instruction::Push(-10), Instruction::End, Instruction::Label(100), Instruction::Push(-1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-10]);
    }

    #[test]
    fn test_jump_negative_1() {
        let input = vec![Instruction::Push(-2), Instruction::JumpNeg(100), Instruction::Push(-10), Instruction::End, Instruction::Label(100), Instruction::Push(-1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-1]);
    }
    
    #[test]
    fn test_jump_negative_2() {
        let input = vec![Instruction::Push(0), Instruction::JumpNeg(100), Instruction::Push(-10), Instruction::End, Instruction::Label(100), Instruction::Push(-1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-10]);
    }

    #[test]
    fn test_jump_negative_3() {
        let input = vec![Instruction::Push(1), Instruction::JumpNeg(100), Instruction::Push(-10), Instruction::End, Instruction::Label(100), Instruction::Push(-1)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![-10]);
    }

    #[test]
    fn test_return() {
        let input = vec![
            Instruction::Call(2),
            Instruction::Call(1),
            Instruction::End,
            Instruction::Label(1), Instruction::Add, Instruction::Return,
            Instruction::Label(2), Instruction::Push(2), Instruction::Push(3), Instruction::Return];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![5]);
    }

    #[test]
    fn test_end() {
        let input = vec![Instruction::End, Instruction::Push(1), Instruction::Push(2)];
        let vm = interpret(&input);
        assert_eq!(vm.stack, vec![]);
        assert_eq!(vm.pc, 0);
    }
}

