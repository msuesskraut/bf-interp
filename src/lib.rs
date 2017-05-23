//! bf-interp is a brainfuck interpreter.

// `pub` just to include them in the documentation.
pub mod ast;
pub mod loop_helper;
pub mod parser;
pub mod opt;
pub mod interp;

#[cfg(test)]
mod buffer;
#[cfg(test)]
mod buffer_tests;
#[cfg(test)]
mod parser_tests;
#[cfg(test)]
mod interp_tests;
#[cfg(test)]
mod opt_tests;
