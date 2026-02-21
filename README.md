# Assignment 1: Adder Compiler

CSCI 282 - Compilers

## Overview
This project implements a compiler for the Adder language. The Adder language supports 32-bit integers and three operations: add1, sub1, and negate. The compiler translates Adder expressions into x86-64 assembly code.

## Project Structure

```
starter-code/
├── src/main.rs          # Compiler implementation
├── runtime/start.rs     # Runtime
├── test/                # Test files
├── Cargo.toml
├── Makefile
├── transcript.txt
└── README.md
```

## The Adder Language

### Syntax
```
<expr> :=
  | <number>
  | (add1 <expr>)
  | (sub1 <expr>)
  | (negate <expr>)
```

### Semantics
- Numbers evaluate to themselves
- add1(e) evaluates e and adds 1
- sub1(e) evaluates e and subtracts 1
- negate(e) evaluates e and multiplies by -1

### Examples

Example 1:
```scheme
(add1 (sub1 5))
```
Result: 5

Example 2:
```scheme
4
```
Result: 4

Example 3:
```scheme
(negate (add1 3))
```
Result: -4

## Implementation

The compiler has three main parts:

1. **Parser** - Converts S-expressions to an AST
2. **Code Generator** - Converts AST to x86-64 assembly
3. **Main** - Reads input file and writes assembly output

The AST is defined as:
```rust
enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}
```

Assembly generation:
- Numbers: `mov rax, n`
- add1: compile subexpr, then `add rax, 1`
- sub1: compile subexpr, then `sub rax, 1`
- negate: compile subexpr, then `imul rax, -1`

## Setup

### Required Tools
1. Rust and Cargo - https://www.rust-lang.org/tools/install
2. NASM - `brew install nasm` (macOS) or `sudo apt-get install nasm` (Linux)
3. GCC/Clang for linking

### Building and Running

Build the compiler:
```bash
cargo build
```

Compile a test file:
```bash
cargo run -- test/num.snek test/num.s
cat test/num.s
```

Build and run:
```bash
make test/num.run
./test/num.run
```

## Test Files

I created 13 test files covering:
- Simple numbers (positive, negative, zero)
- All three operations
- Nested expressions
- Mixed operations

See transcript.txt for test output.

## Deliverables

1. src/main.rs - Complete compiler implementation
2. runtime/start.rs - Runtime entry point
3. Makefile - Build system (configured for macOS)
4. test/ - 13 test files
5. transcript.txt - Shows compiler working on test cases

## Notes

- The Makefile uses `rustc --target x86_64-apple-darwin` for ARM Mac compatibility
- Build artifacts (target/, *.s, *.run, etc.) are not included in submission
