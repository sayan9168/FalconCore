use crate::{lexer::Lexer,parser::{Expr,Parser}};
use std::{collections::HashSet,fs,path::{Path,PathBuf}};
#[derive(Debug,Clone,PartialEq)]
pub struct Module{pub path:PathBuf,pub imports:Vec<PathBuf>,pub ast:Vec<Expr>}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum ModuleError{Io(String),Parse(String),Missing(PathBuf),Cycle(PathBuf)}
pub struct ModuleResolver{visited:HashSet<PathBuf>,stack:Vec<PathBuf>}
impl ModuleResolver{pub fn new()->Self{Self{visited:HashSet::new(),stack:Vec::new()}}pub fn resolve(&mut self,entry:impl AsRef<Path>)->Result<Vec<Module>,ModuleError>{let path=canonical(entry.as_ref())?;let mut modules=Vec::new();self.visit(path,&mut modules)?;Ok(modules)}fn visit(&mut self,path:PathBuf,modules:&mut Vec<Module>)->Result<(),ModuleError>{if self.stack.contains(&path){return Err(ModuleError::Cycle(path))}if self.visited.contains(&path){return Ok(())}let source=fs::read_to_string(&path).map_err(|e|ModuleError::Io(format!("{}: {e}",path.display())))?;let mut parser=Parser::new(Lexer::new(&source));let ast=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||parser.parse())).map_err(|_|ModuleError::Parse(path.display().to_string()))?;self.stack.push(path.clone());let mut imports=Vec::new();for expr in &ast{if let Expr::Import{path:import,..}=expr{let resolved=resolve_import(&path,import);if !resolved.exists(){return Err(ModuleError::Missing(resolved))}imports.push(resolved.clone());self.visit(resolved,modules)?}}self.stack.pop();self.visited.insert(path.clone());modules.push(Module{path,imports,ast});Ok(())}}
fn canonical(path:&Path)->Result<PathBuf,ModuleError>{if !path.exists(){return Err(ModuleError::Missing(path.to_path_buf()))}fs::canonicalize(path).map_err(|e|ModuleError::Io(e.to_string()))}
fn resolve_import(from:&Path,import:&str)->PathBuf{let mut p=from.parent().unwrap_or_else(||Path::new(".")).join(import);if p.extension().is_none(){p.set_extension("falcon")}p}
#[cfg(test)]mod tests{use super::*;#[test]fn resolves_relative_falcon_extension(){let p=resolve_import(Path::new("/tmp/main.falcon"),"lib/math");assert_eq!(p,PathBuf::from("/tmp/lib/math.falcon"));}}
