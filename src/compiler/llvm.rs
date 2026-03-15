use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::values::PointerValue;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target, TargetMachine};
use inkwell::targets::RelocMode;
use inkwell::targets::CodeModel;
use inkwell::targets::FileType;
use std::path::Path;
use std::process::Command;


use crate::instruction::Instruction;

pub struct LLVMCompiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub stack: Option<PointerValue<'ctx>>,
    pub sp: Option<PointerValue<'ctx>>,
}

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module = context.create_module("whitespace");
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            stack: None, // Safe to initialize as None, will be set up in compile()
            sp: None,
        }
    }

    pub fn push(&self, value: i64) {
        let i64_type = self.context.i64_type();
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp").unwrap().into_int_value();

        let stack_ptr = unsafe {
            self.builder.build_gep(
                self.stack.unwrap(),
                &[
                    i64_type.const_int(0, false),
                    sp_val,
                ],
                "stack_ptr",
            ).unwrap()
        };

        self.builder
            .build_store(stack_ptr, i64_type.const_int(value as u64, true))
            .unwrap();
        let new_sp = self.builder
            .build_int_add(sp_val, i64_type.const_int(1, false), "sp_int")
            .unwrap();

        let _ = self.builder.build_store(self.sp.unwrap(), new_sp).unwrap();
    }

    fn dup(&self) {
        let i64_type = self.context.i64_type();

        // Load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val").unwrap().into_int_value();

        // 1. Compute top index: sp - 1
        let top_index = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "top_index").unwrap();

        // 2. Get pointer to top element
        let top_ptr = unsafe { 
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), top_index], "top_ptr")
        }.unwrap();

        // 3. Load top value
        let top_val = self.builder.build_load(top_ptr, "top_val").unwrap();

        // 4. Get pointer to next free slot (current sp)
        let next_ptr = unsafe { 
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), sp_val], "next_ptr") 
        }.unwrap();

        // 5. Store the top value into the next slot
        let _ = self.builder.build_store(next_ptr, top_val);

        // 6. Increment stack pointer
        let sp_inc = self.builder.build_int_add(sp_val, i64_type.const_int(1, false), "sp_inc").unwrap();
        let _ = self.builder.build_store(self.sp.unwrap(), sp_inc);
    }

    fn swap(&self) {
        let i64_type = self.context.i64_type();

        // load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val")
            .expect("load sp failed")
            .into_int_value();

        // Compute indices of the top two elements: sp - 1 and sp - 2
        let idx_top = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "idx_top").unwrap();
        let idx_second = self.builder.build_int_sub(sp_val, i64_type.const_int(2, false), "idx_second").unwrap();

        // Get pointers to top two stack elements
        let ptr_top = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_top], "ptr_top")
                .expect("gep failed")
        };
        let ptr_second = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_second], "ptr_second")
                .expect("gep failed")
        };

        // Load values
        let val_top = self.builder.build_load(ptr_top, "val_top").expect("load top failed");
        let val_second = self.builder.build_load(ptr_second, "val_second").expect("load second failed");

        // Swap: store top in second, second in top
        let _ = self.builder.build_store(ptr_top, val_second);
        let _ = self.builder.build_store(ptr_second, val_top);
    }

    fn drop(&self) {
        let i64_type = self.context.i64_type();

        // Load current sp
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val")
            .expect("load sp failed")
            .into_int_value();

        // Decrement sp by 1
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();

        // Store new sp
        self.builder.build_store(self.sp.unwrap(), new_sp)
            .expect("store sp failed");

        let stack_ptr = unsafe { 
        self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), new_sp], "drop_ptr")
            .expect("gep failed")
        };
        self.builder.build_store(stack_ptr, i64_type.const_int(0, false)).expect("clear failed");
    }
    

    fn add(&self) {
        let i64_type = self.context.i64_type();
        
        // load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val").expect("load sp failed").into_int_value();

        // Compute indices of the top two elements: sp - 1 and sp - 2
        let idx_top = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "idx_top").unwrap();
        let idx_second = self.builder.build_int_sub(sp_val, i64_type.const_int(2, false), "idx_second").unwrap();

        // Get pointers to top two stack elements
        let ptr_top = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_top], "ptr_top")
                .expect("gep failed")
        };
        let ptr_second = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_second], "ptr_second")
                .expect("gep failed")
        };

        // Load values
        let val_top = self.builder.build_load(ptr_top, "val_top").expect("load top failed");
        let val_second = self.builder.build_load(ptr_second, "val_second").expect("load second failed");

        // Add
        let result = self.builder.build_int_add(val_second.into_int_value(), val_top.into_int_value(), "add_result").unwrap();

        // Store result back in second-to-top slot
        let _ = self.builder.build_store(ptr_second, result);
        // Decrement sp by 1 to pop the top element
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();
        self.builder.build_store(self.sp.unwrap(), new_sp).expect("store sp failed");
    }

    fn sub(&self) {
        let i64_type = self.context.i64_type();
        
        // load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val").expect("load sp failed").into_int_value();

        // Compute indices of the top two elements: sp - 1 and sp - 2
        let idx_top = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "idx_top").unwrap();
        let idx_second = self.builder.build_int_sub(sp_val, i64_type.const_int(2, false), "idx_second").unwrap();

        // Get pointers to top two stack elements
        let ptr_top = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_top], "ptr_top")
                .expect("gep failed")
        };
        let ptr_second = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_second], "ptr_second")
                .expect("gep failed")
        };

        // Load values
        let val_top = self.builder.build_load(ptr_top, "val_top").expect("load top failed");
        let val_second = self.builder.build_load(ptr_second, "val_second").expect("load second failed");

        // Subtract
        let result = self.builder.build_int_sub(val_second.into_int_value(), val_top.into_int_value(), "sub_result").unwrap();

        // Store result back in second-to-top slot
        let _ = self.builder.build_store(ptr_second, result);
        // Decrement sp by 1 to pop the top element
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();
        self.builder.build_store(self.sp.unwrap(), new_sp).expect("store sp failed");
    }

    fn mul(&self) {
        let i64_type = self.context.i64_type();
        
        // load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val").expect("load sp failed").into_int_value();

        // Compute indices of the top two elements: sp - 1 and sp - 2
        let idx_top = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "idx_top").unwrap();
        let idx_second = self.builder.build_int_sub(sp_val, i64_type.const_int(2, false), "idx_second").unwrap();

        // Get pointers to top two stack elements
        let ptr_top = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_top], "ptr_top")
                .expect("gep failed")
        };
        let ptr_second = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_second], "ptr_second")
                .expect("gep failed")
        };

        // Load values
        let val_top = self.builder.build_load(ptr_top, "val_top").expect("load top failed");
        let val_second = self.builder.build_load(ptr_second, "val_second").expect("load second failed");

        // Multiply
        let result = self.builder.build_int_mul(val_second.into_int_value(), val_top.into_int_value(), "mul_result").unwrap();

        // Store result back in second-to-top slot
        let _ = self.builder.build_store(ptr_second, result);
        // Decrement sp by 1 to pop the top element
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();
        self.builder.build_store(self.sp.unwrap(), new_sp).expect("store sp failed");
    }

    fn div(&self) {
        let i64_type = self.context.i64_type();
        
        // load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val").expect("load sp failed").into_int_value();

        // Compute indices of the top two elements: sp - 1 and sp - 2
        let idx_top = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "idx_top").unwrap();
        let idx_second = self.builder.build_int_sub(sp_val, i64_type.const_int(2, false), "idx_second").unwrap();

        // Get pointers to top two stack elements
        let ptr_top = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_top], "ptr_top")
                .expect("gep failed")
        };
        let ptr_second = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_second], "ptr_second")
                .expect("gep failed")
        };

        // Load values
        let val_top = self.builder.build_load(ptr_top, "val_top").expect("load top failed");
        let val_second = self.builder.build_load(ptr_second, "val_second").expect("load second failed");

        // Divide
        let result = self.builder.build_int_signed_div(val_second.into_int_value(), val_top.into_int_value(), "div_result").unwrap();

        // Store result back in second-to-top slot
        let _ = self.builder.build_store(ptr_second, result);
        // Decrement sp by 1 to pop the top element
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();
        self.builder.build_store(self.sp.unwrap(), new_sp).expect("store sp failed");
    }

    fn modulo(&self) {
        let i64_type = self.context.i64_type();
        
        // load current stack pointer
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val").expect("load sp failed").into_int_value();

        // Compute indices of the top two elements: sp - 1 and sp - 2
        let idx_top = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "idx_top").unwrap();
        let idx_second = self.builder.build_int_sub(sp_val, i64_type.const_int(2, false), "idx_second").unwrap();

        // Get pointers to top two stack elements
        let ptr_top = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_top], "ptr_top")
                .expect("gep failed")
        };
        let ptr_second = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), idx_second], "ptr_second")
                .expect("gep failed")
        };

        // Load values
        let val_top = self.builder.build_load(ptr_top, "val_top").expect("load top failed");
        let val_second = self.builder.build_load(ptr_second, "val_second").expect("load second failed");

        // Modulo
        let result = self.builder.build_int_signed_rem(val_second.into_int_value(), val_top.into_int_value(), "mod_result").unwrap();

        // Store result back in second-to-top slot
        let _ = self.builder.build_store(ptr_second, result);
        // Decrement sp by 1 to pop the top element
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();
        self.builder.build_store(self.sp.unwrap(), new_sp).expect("store sp failed");
    }


    fn end(&self) {
        let _ = self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
    }


    fn impl_print_char(&self) {
        let i64_type = self.context.i64_type();
        let i32_type = self.context.i32_type();

        // Declare putchar if it doesn't exist yet
        let putchar_fn = match self.module.get_function("putchar") {
            Some(f) => f,
            None => {
                let fn_type = i32_type.fn_type(&[i32_type.into()], false);
                self.module.add_function("putchar", fn_type, None)
            }
        };

        // Load sp
        let sp_val = self.builder.build_load(self.sp.unwrap(), "sp_val")
            .expect("load sp failed")
            .into_int_value();

        // Compute index of top element: sp - 1
        let top_idx = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "top_idx").unwrap();

        // Get pointer to top element
        let top_ptr = unsafe {
            self.builder.build_in_bounds_gep(self.stack.unwrap(), &[i64_type.const_int(0, false), top_idx], "top_ptr")
                .expect("gep failed")
        };

        // Load value
        let val = self.builder.build_load(top_ptr, "top_val").expect("load top failed").into_int_value();

        // Convert i64 -> i32 for putchar
        let char_val = self.builder.build_int_cast(val, i32_type, "char_val").unwrap();

        // Call putchar
        let _ = self.builder.build_call(putchar_fn, &[char_val.into()], "putchar_call");

        // Decrement sp
        let new_sp = self.builder.build_int_sub(sp_val, i64_type.const_int(1, false), "sp_dec").unwrap();
        self.builder.build_store(self.sp.unwrap(), new_sp).expect("store sp failed");
    }

}



pub fn compile(program: &Vec<Instruction>) {
    
    // 1. Create LLVM context
    let context = Context::create();
    let mut compiler = LLVMCompiler::new(&context); // creates module, builder, stack, sp

    
    // 2. Create main function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = compiler.module.add_function("main", fn_type, None);

    let entry = context.append_basic_block(function, "entry");
    compiler.builder.position_at_end(entry);

    let i64_type = context.i64_type();

    let stack_type = i64_type.array_type(1024);
    let stack = compiler.builder.build_alloca(stack_type, "stack").unwrap();

    let sp = compiler.builder.build_alloca(i64_type, "sp").unwrap();
    compiler.builder.build_store(sp, i64_type.const_int(0, false)).unwrap();
    compiler.stack = Some(stack);
    compiler.sp = Some(sp);

    // 3. Compile instructions
    for instruction in program {
        match instruction {
            Instruction::Push(n) => compiler.push(*n),
            Instruction::Dup => compiler.dup(),
            Instruction::Swap => compiler.swap(),
            Instruction::Drop => compiler.drop(),

            Instruction::Add => compiler.add(),
            Instruction::Sub => compiler.sub(),
            Instruction::Mul => compiler.mul(),
            Instruction::Div => compiler.div(),
            Instruction::Mod => compiler.modulo(),





            Instruction::End => compiler.end(),
            Instruction::PrintChar => compiler.impl_print_char(),
            _ => {}
        }
    }

    // 4. Print LLVM IR (for debugging)
    let _ = compiler.module.print_to_stderr();

    // 5. Optimize and generate machine code
    Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).expect("Failed to get target from triple");
    let target_machine = target.create_target_machine(
        &triple,
        "generic",
        "",
        OptimizationLevel::Default,
        RelocMode::Default,
        CodeModel::Default,
    ).expect("Failed to create target machine");

    target_machine
        .write_to_file(&compiler.module, FileType::Object, Path::new("output.o")).expect("Failed to write object file");

    build_executable(&compiler.module, "program");
    
}


pub fn build_executable(module: &inkwell::module::Module, output: &str) {
    // Initialize LLVM targets
    Target::initialize_all(&InitializationConfig::default());

    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).expect("Failed to get target");

    let target_machine = target
        .create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .expect("Failed to create target machine");

    // Emit object file
    let obj_file = format!("{}.o", output);

    target_machine
        .write_to_file(module, FileType::Object, Path::new(&obj_file))
        .expect("Failed to write object file");

    // Link with clang
    Command::new("clang")
        .args([&obj_file, "-o", output])
        .status()
        .expect("Failed to run clang");

    println!("Built executable: {}", output);
}
