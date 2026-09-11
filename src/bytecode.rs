use crate::compiler::{FunctionInfo, Opcode};
use crate::parser::Expr;
use crate::lexer::TokenType;
use std::collections::HashMap;

pub const BYTECODE_MAGIC: &[u8; 4] = b"FCBC";
pub const BYTECODE_VERSION: u16 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BytecodeError {
    InvalidHeader,
    UnsupportedVersion(u16),
    UnexpectedEof,
    InvalidTag(u8),
    InvalidUtf8,
    InvalidOpcode(u8),
    InvalidOperator(u8),
    TrailingBytes,
}

impl std::fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeader => write!(f, "invalid FCBC header"),
            Self::UnsupportedVersion(v) => write!(f, "unsupported FCBC version {v}"),
            Self::UnexpectedEof => write!(f, "truncated FCBC artifact"),
            Self::InvalidTag(v) => write!(f, "invalid constant tag {v}"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 in FCBC artifact"),
            Self::InvalidOpcode(v) => write!(f, "invalid opcode tag {v}"),
            Self::InvalidOperator(v) => write!(f, "invalid operator tag {v}"),
            Self::TrailingBytes => write!(f, "trailing bytes after FCBC artifact"),
        }
    }
}

impl std::error::Error for BytecodeError {}

/// Serialize the compiler output into the versioned FCBC artifact format.
pub fn serialize(
    constants: &[Expr],
    code: &[Opcode],
    functions: &HashMap<String, FunctionInfo>,
) -> Result<Vec<u8>, BytecodeError> {
    let mut out = Vec::new();
    out.extend_from_slice(BYTECODE_MAGIC);
    out.extend_from_slice(&BYTECODE_VERSION.to_le_bytes());
    write_u32(&mut out, constants.len() as u32);
    for constant in constants {
        write_constant(&mut out, constant)?;
    }
    write_u32(&mut out, code.len() as u32);
    for op in code {
        write_opcode(&mut out, op)?;
    }
    write_u32(&mut out, functions.len() as u32);
    for (name, info) in functions {
        write_string(&mut out, name);
        write_u64(&mut out, info.entry as u64);
        write_u32(&mut out, info.params.len() as u32);
        for param in &info.params { write_string(&mut out, param); }
    }
    Ok(out)
}

pub fn deserialize(data: &[u8]) -> Result<(Vec<Expr>, Vec<Opcode>, HashMap<String, FunctionInfo>), BytecodeError> {
    let mut r = Reader { data, pos: 0 };
    if r.take(4)? != BYTECODE_MAGIC { return Err(BytecodeError::InvalidHeader); }
    let version = r.u16()?;
    if version != BYTECODE_VERSION { return Err(BytecodeError::UnsupportedVersion(version)); }
    let constant_count = r.u32()? as usize;
    let mut constants = Vec::with_capacity(constant_count);
    for _ in 0..constant_count { constants.push(r.constant()?); }
    let code_count = r.u32()? as usize;
    let mut code = Vec::with_capacity(code_count);
    for _ in 0..code_count { code.push(r.opcode()?); }
    let function_count = r.u32()? as usize;
    let mut functions = HashMap::with_capacity(function_count);
    for _ in 0..function_count {
        let name = r.string()?;
        let entry = r.u64()? as usize;
        let param_count = r.u32()? as usize;
        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count { params.push(r.string()?); }
        functions.insert(name, FunctionInfo { entry, params });
    }
    if r.pos != data.len() { return Err(BytecodeError::TrailingBytes); }
    Ok((constants, code, functions))
}

/// Human-readable bytecode inspection utilities.
pub fn disassemble(constants: &[Expr], code: &[Opcode]) -> String {
    let mut out = String::new();
    out.push_str("== FalconCore Bytecode ==\n");
    out.push_str(&format!("Format: FCBC v{}\n", BYTECODE_VERSION));
    out.push_str("Constants:\n");
    for (i, c) in constants.iter().enumerate() { out.push_str(&format!("  [{i:03}] {c:?}\n")); }
    out.push_str("Instructions:\n");
    for (ip, op) in code.iter().enumerate() { out.push_str(&format!("  {ip:04}  {op:?}\n")); }
    out
}

fn write_u32(out: &mut Vec<u8>, n: u32) { out.extend_from_slice(&n.to_le_bytes()); }
fn write_u64(out: &mut Vec<u8>, n: u64) { out.extend_from_slice(&n.to_le_bytes()); }
fn write_string(out: &mut Vec<u8>, s: &str) { write_u32(out, s.len() as u32); out.extend_from_slice(s.as_bytes()); }

fn write_constant(out: &mut Vec<u8>, e: &Expr) -> Result<(), BytecodeError> {
    match e {
        Expr::Number(n) => { out.push(0); out.extend_from_slice(&n.to_le_bytes()); }
        Expr::Float(n) => { out.push(1); out.extend_from_slice(&n.to_le_bytes()); }
        Expr::String(s) => { out.push(2); write_string(out, s); }
        Expr::Bool(v) => { out.push(3); out.push(*v as u8); }
        Expr::Null => out.push(4),
        _ => return Err(BytecodeError::InvalidTag(255)),
    }
    Ok(())
}

fn write_opcode(out: &mut Vec<u8>, op: &Opcode) -> Result<(), BytecodeError> {
    match op {
        Opcode::LoadConst(i) => { out.push(0); write_u64(out, *i as u64); }
        Opcode::LoadVar(s) => { out.push(1); write_string(out, s); }
        Opcode::StoreVar(s) => { out.push(2); write_string(out, s); }
        Opcode::Add => out.push(3), Opcode::Sub => out.push(4), Opcode::Mul => out.push(5), Opcode::Div => out.push(6),
        Opcode::Equal => out.push(7), Opcode::NotEqual => out.push(8), Opcode::Greater => out.push(9), Opcode::Less => out.push(10),
        Opcode::GreaterEqual => out.push(11), Opcode::LessEqual => out.push(12), Opcode::Print => out.push(13),
        Opcode::JumpIfFalse(i) => { out.push(14); write_u64(out, *i as u64); }
        Opcode::Jump(i) => { out.push(15); write_u64(out, *i as u64); }
        Opcode::RepeatStart => out.push(16),
        Opcode::RepeatEnd(i) => { out.push(17); write_u64(out, *i as u64); }
        Opcode::Call(s, n) => { out.push(18); write_string(out, s); write_u32(out, *n as u32); }
        Opcode::Return => out.push(19), Opcode::NetworkScan => out.push(20), Opcode::Halt => out.push(21),
    }
    Ok(())
}

struct Reader<'a> { data: &'a [u8], pos: usize }
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], BytecodeError> { if self.pos + n > self.data.len() { return Err(BytecodeError::UnexpectedEof); } let s=&self.data[self.pos..self.pos+n]; self.pos+=n; Ok(s) }
    fn u16(&mut self)->Result<u16,BytecodeError>{Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))}
    fn u32(&mut self)->Result<u32,BytecodeError>{Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))}
    fn u64(&mut self)->Result<u64,BytecodeError>{Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))}
    fn string(&mut self)->Result<String,BytecodeError>{let n=self.u32()? as usize;String::from_utf8(self.take(n)?.to_vec()).map_err(|_|BytecodeError::InvalidUtf8)}
    fn constant(&mut self)->Result<Expr,BytecodeError>{match self.take(1)?[0]{0=>Ok(Expr::Number(i64::from_le_bytes(self.take(8)?.try_into().unwrap()))),1=>Ok(Expr::Float(f64::from_le_bytes(self.take(8)?.try_into().unwrap()))),2=>Ok(Expr::String(self.string()?)),3=>Ok(Expr::Bool(self.take(1)?[0]!=0)),4=>Ok(Expr::Null),t=>Err(BytecodeError::InvalidTag(t))}}
    fn opcode(&mut self)->Result<Opcode,BytecodeError>{let t=self.take(1)?[0];Ok(match t{0=>Opcode::LoadConst(self.u64()? as usize),1=>Opcode::LoadVar(self.string()?),2=>Opcode::StoreVar(self.string()?),3=>Opcode::Add,4=>Opcode::Sub,5=>Opcode::Mul,6=>Opcode::Div,7=>Opcode::Equal,8=>Opcode::NotEqual,9=>Opcode::Greater,10=>Opcode::Less,11=>Opcode::GreaterEqual,12=>Opcode::LessEqual,13=>Opcode::Print,14=>Opcode::JumpIfFalse(self.u64()? as usize),15=>Opcode::Jump(self.u64()? as usize),16=>Opcode::RepeatStart,17=>Opcode::RepeatEnd(self.u64()? as usize),18=>Opcode::Call(self.string()?,self.u32()? as usize),19=>Opcode::Return,20=>Opcode::NetworkScan,21=>Opcode::Halt,t=>return Err(BytecodeError::InvalidOpcode(t))})}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn format_header_is_stable(){assert_eq!(BYTECODE_MAGIC,b"FCBC");assert_eq!(BYTECODE_VERSION,2);}
    #[test] fn round_trip_simple_artifact(){let c=vec![Expr::Number(42),Expr::Bool(true),Expr::String("ok".into())];let code=vec![Opcode::LoadConst(0),Opcode::Print,Opcode::Halt];let f=HashMap::new();let bytes=serialize(&c,&code,&f).unwrap();let decoded=deserialize(&bytes).unwrap();assert_eq!(decoded.0,c);assert_eq!(decoded.1,code);assert_eq!(decoded.2,f);}
    #[test] fn operators_are_import_free(){let _ = TokenType::Plus;}
}
