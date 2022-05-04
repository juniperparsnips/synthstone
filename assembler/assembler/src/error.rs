use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssemblyError {
    #[error(transparent)]
    StrFromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("U4 out of range")]
    U4Range,
    #[error("U12 out of range")]
    U12Range,
    #[error("Error extracting first word of line {0}")]
    FirstWordError(usize),
    #[error("The label {0} is repeated")]
    RepeatedLabel(String),
    #[error("Labels must be on their own line ('{0}' is not)")]
    LabelNotAlone(String),
    #[error("Unreachable parsing error")]
    UnreachableParsing,
    #[error("Registers must take the format '$<id>'.")]
    RegisterDollarSign,
    #[error("Unknown instruction '{0}' (instructions are case-sensitive)")]
    UnknownInstruction(String),
    #[error("Invalid parameter '{1}' for instruction '{0}'")]
    InvalidParameter(String, String),
    #[error("Unknown label '{0}'")]
    UnknownLabel(String),
    #[error("Unknown assembly parsing error")]
    UnknownParsingErr,
}

pub type AssemblyResult<T> = Result<T, AssemblyError>;