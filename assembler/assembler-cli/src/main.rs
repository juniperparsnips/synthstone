use std::{fs, vec::Vec};

use clap::Parser;

use assembler::{AssemblyResult, util::file_parsing};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(short, long)]
    input_file: String,
    #[clap(short, long)]
    output_file: String
}

fn main() -> AssemblyResult<()> {
    let args: CliArgs = Parser::parse();

    let the_assembly = String::from_utf8(fs::read(args.input_file)?)?;

    let lines = the_assembly.lines();
    let mut stripped_lines = Vec::new();

    for line in lines {
        stripped_lines.push(file_parsing::strip_comments(line));
    }

    let labels = file_parsing::find_labels(&stripped_lines)?;

    let machine_code = file_parsing::parse_lines(&stripped_lines, &labels)?;

    let mut machine_code_str = String::new();
    for instruction in machine_code {
        machine_code_str.push_str(&instruction.to_string());
        machine_code_str.push_str("\n");
    }

    // TODO: print machine code to file
    fs::write(args.output_file, machine_code_str)?;

    Ok(())
}