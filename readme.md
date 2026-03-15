# Whitespace Compiler (Rust + LLVM)

A **compiler for the Whitespace programming language** written in **Rust**, using **LLVM** through the **Inkwell** bindings.

The project started as a **Whitespace interpreter** and is being extended to generate **LLVM IR**, allowing programs to be compiled and optimized using LLVM.

---

## About Whitespace

Whitespace is an esoteric programming language where **only spaces, tabs, and linefeeds are meaningful**. All other characters are ignored, which allows Whitespace programs to hide inside seemingly normal text files.

Core features of the language include:

* Stack-based execution
* Arithmetic operations
* Heap access
* Flow control (labels, jumps, calls)
* Input and output

More information about the language can be found here:
https://esolangs.org/wiki/Whitespace

---

## Project Goals

This project aims to:

* Parse Whitespace programs into an internal instruction representation
* Execute them using an **interpreter**
* Compile them to **LLVM IR**
* Eventually produce **native binaries** via LLVM

---

## Current Features

* Whitespace **parser**
* **Interpreter / VM** for executing programs
* LLVM **IR generation** using Inkwell
* Basic stack operations implemented in LLVM

Example instructions implemented in the compiler backend:

* `Push`
* `Dup`
* `Swap`
* `Drop`
* `Add`
* `Sub`
* `PrintChar`
* `End`

---

## Project Structure

```
src/
├── compiler/
│   └── llvm.rs        # LLVM IR code generation
├── parser.rs          # Whitespace parser
├── instruction.rs     # Instruction enum
├── vm/                # Interpreter implementation
└── main.rs            # Entry point
```

---

## Dependencies

* Rust
* LLVM 11
* Inkwell 0.8

Example Cargo dependency:

```toml
inkwell = { version = "0.8", features = ["llvm11-0"] }
```

Make sure LLVM 11 is installed on your system.

---

## Building

Clone the repository:

```
git clone https://github.com/zackeua/whitespace
cd whitespace
```

Build the project:

```
cargo build
```

Run the project (currently only supports a minimal subset of the language):

```
cargo run code/hello.ws
./program
```

---

## Running tests

To run the tests:
```
cargo test
```

## Debugging LLVM IR

The compiler currently prints the generated LLVM IR to stderr for debugging:

```
module.print_to_stderr();
```

This allows you to inspect the generated instructions before adding JIT or native compilation.

---

## Future Work

Planned improvements include:

* More complete instruction coverage
* Heap support
* Control flow instructions
* JIT execution using LLVM
* Native binary output
* Optimizations

---

## License

MIT License

---

## Author

Created by **Zackeus** as a learning project exploring:

* Rust
* LLVM
* Compiler design
* Esoteric programming languages
