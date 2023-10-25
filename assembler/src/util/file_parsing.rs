use std::{collections::HashMap, str::FromStr, vec::Vec};

use crate::{
    instruction_set::Instruction,
    util::{Labels, U12},
    AssemblyError, AssemblyResult,
};

pub fn strip_comments(line: &str) -> &str {
    line.split_once('#').unwrap_or_else(|| (line, "")).0
}

pub fn find_labels(lines: &Vec<&str>) -> AssemblyResult<Labels> {
    let mut labels = HashMap::new();

    // Stores the instruction memory address of the next instruction
    let mut program_count = 0;
    for i in 0..lines.len() {
        let mut terms = lines[i].split_ascii_whitespace();

        let first_word = match terms.next() {
            Some(word) => word,
            None => continue,
        };

        match first_word.split_once(":") {
            None => {
                program_count += 1;
            }
            Some((label, rest)) => {
                if labels.insert(label.into(), program_count as u8).is_some() {
                    return Err(AssemblyError::RepeatedLabel(label.to_string()));
                }

                if !rest.is_empty() || !terms.remainder().is_none() {
                    return Err(AssemblyError::LabelNotAlone(label.to_string()));
                }
            }
        };
    }

    Ok(labels)
}

pub fn parse_lines(lines: &Vec<&str>, labels: &Labels) -> AssemblyResult<Vec<U12>> {
    let mut instructions = Vec::with_capacity(lines.len());
    for i in 0..lines.len() {
        let mut terms = lines[i].split_ascii_whitespace();

        let first_word = match terms.next() {
            Some(word) => word,
            None => continue,
        };

        // The first word can be just an instruction, just a label, or label:instr
        let instruction_str = if first_word.contains(":") {
            let remainder = first_word
                .split_once(":")
                .ok_or(AssemblyError::UnreachableParsing)?
                .1;
            if remainder == "" {
                continue;
            } else {
                remainder
            }
        } else {
            first_word
        };

        let instruction = <&Instruction>::from_str(instruction_str)?;
        instructions.push(
            instruction.gen_machine_code(
                &terms
                    .remainder()
                    .ok_or_else(|| AssemblyError::UnknownParsingErr)?,
                labels,
            )?,
        );
    }

    instructions.shrink_to_fit();
    Ok(instructions)
}
