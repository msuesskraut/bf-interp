# bf-interp

[![Build Status](https://travis-ci.org/msuesskraut/bf-interp.svg?branch=master)](https://travis-ci.org/msuesskraut/bf-interp)
 [![codecov.io](https://codecov.io/gh/msuesskraut/bf-interp/coverage.svg?branch=master)](https://codecov.io/gh/msuesskraut/bf-interp?branch=master)
 [![license](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/msuesskraut/bf-interp)

A brainfuck interpreter in Rust.
Its main purpose is to learn some Rust.

Start it with `cargo run`.
It will execute the `mandelbrot.bf` program from the `examples` directory.
The program prints the Mandelbrot picture in ASCII on the screen.
The `main.rs` can be easily hacked to executed arbitrary brainfuck programs.

The implementation is inspired by blog post from Eli Bendersky:
* [Adventures in JIT compilation: Part 1 - an interpreter](http://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-1-an-interpreter/)
* [Adventures in JIT compilation: Part 2 - an x64 JIT](http://eli.thegreenplace.net/2017/adventures-in-jit-compilation-part-2-an-x64-jit/)

License: MIT

## Architecture

The module `ast.rs` defines the in-memory representation for a brainfuck program.
A `Program` is a list (`vec`) of `Instruction`s.
`Instruction` is a rust enum, which is a perfect fit. 
This representation takes already optimizations into account.

The implementation is split into a classical 3-layer interpreter architecture:
* `parser.rs` translates a string into an ast-representation
  * because of the simplicity of the language it includes the lexer step as well
  * the parser uses `loop_helper.rs` speed-up the loop parsing
* `opt.rs` implements simple optimizations by generting the an optimized `Program` from an unoptimized `Program`
  * the optimizations are described below
* `interp.rs` implements the interpreter itself
  * can interpret both optimized and unoptimized `Program`s

## Optimizations

`opt.rs` implements one general optimization.
It merges successif `+++` into an `Inc(3)` and so on.
It works also `-`, `<` and `>`.
This optimzations reduces the number of iterations over the program ast at runtime.

Furthermore, the `parser.rs` already looks up loop-exits and loop-entrys while parsing
to remove these look-up steps from runtime.
The loop-up is implemented in `loop_helper.rs`.
The optimizations also use `loop_helper.rs` to re-calculate these loop-ups
when reducing the total number of instructions.

Further optimizations will follow.