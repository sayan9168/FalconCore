use crate::compiler::Opcode;
use crate::parser::Expr;

/// Human-readable bytecode inspection utilities for the FalconCore toolchain.
pub fn disassemble(constants: &[Expr], code: &[Opcode]) -> String {
    let mut out = String::new();
    out.push_str("== FalconCore Bytecode ==\n");
    out.push_str("Constants:\n");
    for (i, constant) in constants.iter().enumerate() {
        out.push_str(&format!("  [{i:03}] {constant:?}\n"));
    }
    out.push_str("Instructions:\n");
    for (ip, opcode) in code.iter().enumerate() {
        out.push_str(&format!("  {ip:04}  {opcode:?}\n"));
    }
    out
}
