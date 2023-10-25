use std::str::FromStr;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{AssemblyError, AssemblyResult, util::{Labels, U4, U12, instruction_parsing::{
    parse_immediate, parse_label, parse_register
}}};

// TODO: Idea: deserialize assembly, serialize into machine code

// TODO: look for a generic bitsize data path.

lazy_static!{
    static ref nop: Instruction = Instruction {
        name: "nop",
        instruction_type: InstructionType::B,
        opcode: U4::new(0x0).unwrap(),
    };

    static ref lda: Instruction = Instruction {
        name: "lda",
        instruction_type: InstructionType::R,
        opcode: U4::new(0x1).unwrap(),
    };

    static ref ldb: Instruction = Instruction {
        name: "ldb",
        instruction_type: InstructionType::R,
        opcode: U4::new(0x2).unwrap(),
    };

    static ref ldai: Instruction = Instruction {
        name: "ldai",
        instruction_type: InstructionType::I,
        opcode: U4::new(0x3).unwrap(),
    };

    static ref ldbi: Instruction = Instruction {
        name: "ldbi",
        instruction_type: InstructionType::I,
        opcode: U4::new(0x4).unwrap(),
    };

    static ref add: Instruction = Instruction {
        name: "add",
        instruction_type: InstructionType::R,
        opcode: U4::new(0x5).unwrap(),
    };

    static ref sub: Instruction = Instruction {
        name: "sub",
        instruction_type: InstructionType::R,
        opcode: U4::new(0x6).unwrap(),
    };

    static ref inv: Instruction = Instruction {
        name: "inv",
        instruction_type: InstructionType::R,
        opcode: U4::new(0x7).unwrap(),
    };

    static ref jmp: Instruction = Instruction {
        name: "jmp",
        instruction_type: InstructionType::J,
        opcode: U4::new(0xC).unwrap(),
    };

    static ref beq: Instruction = Instruction {
        name: "beq",
        instruction_type: InstructionType::J,
        opcode: U4::new(0xD).unwrap(),
    };

    static ref bne: Instruction = Instruction {
        name: "bne",
        instruction_type: InstructionType::J,
        opcode: U4::new(0xE).unwrap(),
    };

    static ref hlt: Instruction = Instruction {
        name: "hlt",
        instruction_type: InstructionType::B,
        opcode: U4::new(0xF).unwrap(),
    };
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum InstructionType {
    R,
    I,
    J,
    B,
}

#[derive(Debug, Clone)]
pub enum InstructionFlags {}

#[derive(Debug, Clone)]
pub struct Instruction {
    name: &'static str,
    instruction_type: InstructionType,
    opcode: U4
    // flags: InstructionFlags,
}

impl Instruction {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn instruction_type(&self) -> InstructionType {
        self.instruction_type
    }

    pub fn opcode(&self) -> U4 {
        self.opcode
    }

    // pub fn flags(&self) -> InstructionFlags {
    //     &self.instruction_flags
    // }

    pub fn gen_machine_code(&self, parameters: &str, labels: &Labels) -> AssemblyResult<U12> {
        let bits = self.validate_parameters(parameters, labels)?;

        let machinecode = (self.opcode.get() << 8) + (bits as usize);
        U12::new(machinecode)
    }

    fn validate_parameters(&self, parameters: &str, labels: &Labels) -> AssemblyResult<u8> {
        match self.instruction_type() {
            InstructionType::R => {
                Ok((parse_register(parameters)?.get() << 4) as u8)
            },
            InstructionType::I => {
                // TODO: Parameters for Jump arguments
                parse_immediate(parameters)
            },
            InstructionType::J => {
                parse_label(parameters, labels)
            }
            InstructionType::B => {
                match parameters {
                    "" => Ok(0),
                    other => Err(AssemblyError::InvalidParameter(self.name.into(), other.into()))
                }
            },
        }
    }
}

impl FromStr for &Instruction {
    type Err = AssemblyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(&nop),
            "lda" => Ok(&lda),
            "ldb" => Ok(&ldb),
            "ldai" => Ok(&ldai),
            "ldbi" => Ok(&ldbi),
            "add" => Ok(&add),
            "sub" => Ok(&sub),
            "inv" => Ok(&inv),
            "jmp" => Ok(&jmp),
            "beq" => Ok(&beq),
            "bne" => Ok(&bne),
            "hlt" => Ok(&hlt),
            other => Err(AssemblyError::UnknownInstruction(other.to_string()))
        }
    }
}