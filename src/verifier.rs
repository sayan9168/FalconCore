use crate::{compiler::{FunctionInfo, Opcode}, parser::Expr};
use std::collections::HashMap;

/// Errors raised when an FCBC program violates runtime safety invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    EmptyCode,
    InvalidConstant { index: usize },
    InvalidJump { target: usize },
    InvalidRepeatTarget { target: usize },
    InvalidFunctionEntry { function: String, entry: usize },
    DuplicateFunction { function: String },
    InvalidCall { function: String, expected: usize, got: usize },
    FunctionEntryPointsIntoOperand { function: String, entry: usize },
    MissingHalt,
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyCode => write!(f, "bytecode contains no instructions"),
            Self::InvalidConstant { index } => write!(f, "invalid constant index {index}"),
            Self::InvalidJump { target } => write!(f, "invalid jump target {target}"),
            Self::InvalidRepeatTarget { target } => write!(f, "repeat target {target} is not RepeatStart"),
            Self::InvalidFunctionEntry { function, entry } => write!(f, "function {function} has invalid entry {entry}"),
            Self::DuplicateFunction { function } => write!(f, "duplicate function {function}"),
            Self::InvalidCall { function, expected, got } => write!(f, "call to {function} expects {expected} arguments, got {got}"),
            Self::FunctionEntryPointsIntoOperand { function, entry } => write!(f, "function {function} entry {entry} is not an instruction boundary"),
            Self::MissingHalt => write!(f, "bytecode has no Halt instruction"),
        }
    }
}

impl std::error::Error for VerificationError {}

/// Validate compiler-produced or externally loaded bytecode before execution.
/// The verifier deliberately performs structural checks only; type safety remains
/// the responsibility of the language type checker and runtime.
pub fn verify(
    constants: &[Expr],
    code: &[Opcode],
    functions: &HashMap<String, FunctionInfo>,
) -> Result<(), VerificationError> {
    if code.is_empty() { return Err(VerificationError::EmptyCode); }
    if !code.iter().any(|op| matches!(op, Opcode::Halt)) { return Err(VerificationError::MissingHalt); }

    let mut boundaries = vec![true; code.len()];
    for (ip, op) in code.iter().enumerate() {
        match op {
            Opcode::LoadConst(index) => {
                if *index >= constants.len() { return Err(VerificationError::InvalidConstant { index: *index }); }
            }
            Opcode::Jump(target) | Opcode::JumpIfFalse(target) => {
                if *target >= code.len() { return Err(VerificationError::InvalidJump { target: *target }); }
                boundaries[*target] = true;
            }
            Opcode::RepeatEnd(target) => {
                if *target >= code.len() || !matches!(code.get(*target), Some(Opcode::RepeatStart)) {
                    return Err(VerificationError::InvalidRepeatTarget { target: *target });
                }
            }
            Opcode::Call(name, argc) => {
                if let Some(info) = functions.get(name) {
                    if *argc != info.params.len() {
                        return Err(VerificationError::InvalidCall { function: name.clone(), expected: info.params.len(), got: *argc });
                    }
                } else {
                    return Err(VerificationError::InvalidCall { function: name.clone(), expected: 0, got: *argc });
                }
            }
            _ => {}
        }
        if matches!(op, Opcode::LoadConst(_) | Opcode::JumpIfFalse(_) | Opcode::Jump(_) | Opcode::RepeatEnd(_) | Opcode::Call(_, _)) {
            let _ = ip;
        }
    }

    let mut seen_entries = HashMap::<usize, String>::new();
    for (name, info) in functions {
        if info.entry >= code.len() { return Err(VerificationError::InvalidFunctionEntry { function: name.clone(), entry: info.entry }); }
        if let Some(previous) = seen_entries.insert(info.entry, name.clone()) {
            return Err(VerificationError::DuplicateFunction { function: format!("{previous}/{name}") });
        }
        if !boundaries[info.entry] { return Err(VerificationError::FunctionEntryPointsIntoOperand { function: name.clone(), entry: info.entry }); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_minimal_program() {
        verify(&[Expr::Number(1)], &[Opcode::LoadConst(0), Opcode::Halt], &HashMap::new()).unwrap();
    }
    #[test]
    fn rejects_bad_constant() {
        let err = verify(&[], &[Opcode::LoadConst(2), Opcode::Halt], &HashMap::new()).unwrap_err();
        assert!(matches!(err, VerificationError::InvalidConstant { index: 2 }));
    }
    #[test]
    fn rejects_missing_halt() {
        assert!(matches!(verify(&[], &[Opcode::Return], &HashMap::new()), Err(VerificationError::MissingHalt)));
    }
    #[test]
    fn rejects_bad_jump() {
        let err = verify(&[], &[Opcode::Jump(9), Opcode::Halt], &HashMap::new()).unwrap_err();
        assert!(matches!(err, VerificationError::InvalidJump { target: 9 }));
    }
}
