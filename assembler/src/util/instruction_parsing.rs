use crate::{AssemblyError, AssemblyResult, util::{U4, Labels}};

/// Turns a parameter string for an immediate value into its representative 
/// unsigned integer.
/// Possible immediate values:
/// 0 -> 0000_0000 -> 0
/// -5 -> 1111_1011 -> 251
/// 251 -> 1111_1011 -> 251
/// 0xFB -> 1111_1011 -> 251
/// 0b1111_10111 -> 251
/// That is valid inputs are:
/// Signed decimal [-128, 127]
/// Unsigned decimal [0, 255]
/// Unsigned hex [0x00, 0xFF]
/// Unsigned binary [0b0000_0000, 0b1111_1111]
/// 
/// Inputs can have underscores to help with human readability
pub fn parse_immediate(val: &str) -> AssemblyResult<u8> {
    let (mut digits, radix) = if val.starts_with("0x") {
        (val.split_at(2).1.to_string(), 16)
    } else if val.starts_with("0b") {
        (val.split_at(2).1.to_string(), 2)
    } else {
        (val.to_string(), 10)
    };

    digits.remove_matches("_");

    if digits.starts_with("-") && radix == 10 {
        let x = i8::from_str_radix(&digits, 10)?;
        u8::try_from(256 + x as i16).map_err(|e| e.into())
    } else {
        u8::from_str_radix(&digits, radix).map_err(|e| e.into())
    }  
}

/// Must be formatted as $0 .. $15 
/// TODO: allow key words for the registers
pub fn parse_register(reg: &str) -> AssemblyResult<U4> {
    match reg.split_once("$") {
        Some(("", id)) => U4::new(id.parse()?),
        _ => Err(AssemblyError::RegisterDollarSign)
    }
}

/// Labels can be in the form of a hexadeximal number (0x0) directly referencing a PC address,
/// or a text label previously defined in the file.
pub fn parse_label(label: &str, labels: &Labels) -> AssemblyResult<u8> {
    if label.starts_with("0x") {
        return u8::from_str_radix(label.split_at(2).1, 16).map_err(|e| e.into());
    }

    labels.get(label)
        .map(|x| x.to_owned())
        .ok_or_else(|| AssemblyError::UnknownLabel(label.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_parse_immediate() {
        assert_eq!(parse_immediate("0").unwrap(), 0);
        assert_eq!(parse_immediate("-128").unwrap(), 128);
        assert_eq!(parse_immediate("-5").unwrap(), 251);
        assert_eq!(parse_immediate("251").unwrap(), 251);
        assert_eq!(parse_immediate("0xFB").unwrap(), 251);
        assert_eq!(parse_immediate("0b1111_1011").unwrap(), 251);
        assert_eq!(parse_immediate("0b0_1111_1011").unwrap(), 251);
    }

    #[test]
    fn test_parse_immediate_errs() {
        assert!(parse_immediate("256").is_err());
        assert!(parse_immediate("0xabc").is_err());
        assert!(parse_immediate("-129").is_err());
        assert!(parse_immediate("0b1_0000_0000").is_err());

        assert!(parse_immediate("abc").is_err());
        assert!(parse_immediate("10 0").is_err());

        assert!(parse_immediate("").is_err());
    }

    #[test]
    fn test_parse_register() {
        assert_eq!(parse_register("$0").unwrap(), U4::new(0).unwrap());
        assert_eq!(parse_register("$15").unwrap(), U4::new(15).unwrap());
        assert!(parse_register("$16").is_err());
        assert!(parse_register("$a").is_err());
        assert!(parse_register("5").is_err());
    }

    #[test]
    fn test_parse_label() {
        let mut labels: Labels = HashMap::new();
        labels.insert("main".to_string(), 0);
        labels.insert("cond1".to_string(), 5);

        assert_eq!(parse_label("main", &labels).unwrap(), 0);
        assert_eq!(parse_label("cond1", &labels).unwrap(), 5);
        assert_eq!(parse_label("0xA", &labels).unwrap(), 10);
        assert_eq!(parse_label("0xFF", &labels).unwrap(), 255);
 
        assert!(parse_label("0xFFF", &labels).is_err());
        assert!(parse_label("not_a_label", &labels).is_err());
    }
}