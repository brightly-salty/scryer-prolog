#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::too_many_lines)]

extern crate lexical;
#[cfg(feature = "num-rug-adapter")]
extern crate num_rug_adapter as rug;
extern crate ordered_float;
#[cfg(feature = "rug")]
extern crate rug;
extern crate unicode_reader;

#[macro_use]
pub mod tabled_rc;
#[macro_use]
pub mod ast;
#[macro_use]
pub mod macros;
pub mod parser;
pub mod put_back_n;

pub mod lexer;
