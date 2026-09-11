use crate::compiler::Opcode;
use crate::parser::Expr;
pub const BYTECODE_MAGIC:&[u8;4]=b"FCBC";
pub const BYTECODE_VERSION:u16=1;
/// Human-readable bytecode inspection utilities for the FalconCore toolchain.
pub fn disassemble(constants:&[Expr],code:&[Opcode])->String{let mut out=String::new();out.push_str("== FalconCore Bytecode ==\n");out.push_str(&format!("Format: FCBC v{}\n",BYTECODE_VERSION));out.push_str("Constants:\n");for(i,c)in constants.iter().enumerate(){out.push_str(&format!("  [{i:03}] {c:?}\n"));}out.push_str("Instructions:\n");for(ip,op)in code.iter().enumerate(){out.push_str(&format!("  {ip:04}  {op:?}\n"));}out}
#[cfg(test)]mod tests{use super::*;#[test]fn format_header_is_stable(){assert_eq!(BYTECODE_MAGIC,b"FCBC");assert_eq!(BYTECODE_VERSION,1);}}
