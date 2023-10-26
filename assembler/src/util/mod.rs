use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

use crate::{AssemblyError, AssemblyResult};

pub mod file_parsing;
pub mod instruction_parsing;

pub type Labels = HashMap<String, u8>;

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct U4 {
    val: usize,
}

impl U4 {
    pub fn new(val: usize) -> AssemblyResult<Self> {
        if val < 16 {
            Ok(Self { val })
        } else {
            Err(AssemblyError::U4Range)
        }
    }

    pub fn get(&self) -> usize {
        self.val
    }
}

impl Display for U4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#04b}", self.val)
    }
}

impl TryFrom<u8> for U4 {
    type Error = AssemblyError;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        U4::new(val.into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct U12 {
    val: usize,
}

impl U12 {
    pub fn new(val: usize) -> AssemblyResult<Self> {
        if val < 4096 {
            Ok(Self { val })
        } else {
            Err(AssemblyError::U12Range)
        }
    }

    pub fn get(&self) -> usize {
        self.val
    }
}

impl Display for U12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:012b}", self.val)
    }
}

impl TryFrom<u16> for U12 {
    type Error = AssemblyError;
    fn try_from(val: u16) -> Result<Self, Self::Error> {
        U12::new(val.into())
    }
}
