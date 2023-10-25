#![feature(str_split_whitespace_remainder)]
#![feature(string_remove_matches)]

// #![deny(missing_docs)]
// #![deny(warnings)]

pub use error::{AssemblyError, AssemblyResult};

pub mod error;
pub mod instruction_set;
pub mod util;
