use crate::parser::Expr;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind { Variable, Constant, Function }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol { pub name: String, pub kind: SymbolKind, pub exported: bool }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSymbols { pub module: String, pub symbols: BTreeMap<String, Symbol>, pub imports: BTreeSet<String> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolError { Duplicate(String), MissingImport(String), InvalidExport(String), InvalidName(String), Ambiguous(String) }

impl Symbol {
    pub fn qualified_name(&self, module: &str) -> String { format!("{module}::{}", self.name) }
}

impl ModuleSymbols {
    pub fn qualified(&self, name: &str) -> Option<&Symbol> { self.symbols.get(name) }
    pub fn exported_names(&self) -> Vec<&str> { self.symbols.values().filter(|s| s.exported).map(|s| s.name.as_str()).collect() }
}

pub fn analyze_module(module: impl Into<String>, ast: &[Expr]) -> Result<ModuleSymbols, Vec<SymbolError>> {
    let module = module.into();
    let mut symbols = BTreeMap::new();
    let mut imports = BTreeSet::new();
    let mut errors = Vec::new();
    if module.trim().is_empty() { errors.push(SymbolError::InvalidName("module name is empty".into())); }
    collect(ast, false, &mut symbols, &mut imports, &mut errors);
    if errors.is_empty() { Ok(ModuleSymbols { module, symbols, imports }) } else { Err(errors) }
}

fn collect(ast: &[Expr], exported: bool, symbols: &mut BTreeMap<String, Symbol>, imports: &mut BTreeSet<String>, errors: &mut Vec<SymbolError>) {
    for expr in ast {
        match expr {
            Expr::Import { path, alias } => { imports.insert(alias.clone().unwrap_or_else(|| path.clone())); }
            Expr::Export { item } => match item.as_ref() {
                Expr::Let { name, is_const, .. } => insert(symbols, name, if *is_const { SymbolKind::Constant } else { SymbolKind::Variable }, true, errors),
                Expr::FnDef { name, .. } => insert(symbols, name, SymbolKind::Function, true, errors),
                Expr::Export { .. } => errors.push(SymbolError::InvalidExport("nested export".into())),
                _ => errors.push(SymbolError::InvalidExport("only declarations can be exported".into())),
            },
            Expr::Let { name, is_const, .. } => insert(symbols, name, if *is_const { SymbolKind::Constant } else { SymbolKind::Variable }, exported, errors),
            Expr::FnDef { name, .. } => insert(symbols, name, SymbolKind::Function, exported, errors),
            _ => {}
        }
    }
}

fn insert(symbols: &mut BTreeMap<String, Symbol>, name: &str, kind: SymbolKind, exported: bool, errors: &mut Vec<SymbolError>) {
    if name.trim().is_empty() { errors.push(SymbolError::InvalidName("empty symbol".into())); return; }
    if let Some(existing) = symbols.get_mut(name) {
        if existing.exported || exported { existing.exported = existing.exported || exported; }
        errors.push(SymbolError::Duplicate(name.into()));
        return;
    }
    symbols.insert(name.into(), Symbol { name: name.into(), kind, exported });
}

pub fn resolve_export<'a>(symbols: &'a ModuleSymbols, name: &str) -> Result<&'a Symbol, SymbolError> {
    match symbols.symbols.get(name) {
        Some(symbol) if symbol.exported => Ok(symbol),
        _ => Err(SymbolError::MissingImport(name.into())),
    }
}

pub fn resolve_qualified<'a>(modules: &'a BTreeMap<String, ModuleSymbols>, reference: &str) -> Result<&'a Symbol, SymbolError> {
    let (module, name) = reference.split_once("::").ok_or_else(|| SymbolError::InvalidName(reference.into()))?;
    if module.is_empty() || name.is_empty() { return Err(SymbolError::InvalidName(reference.into())); }
    let table = modules.get(module).ok_or_else(|| SymbolError::MissingImport(module.into()))?;
    resolve_export(table, name)
}

pub fn build_namespace<'a>(modules: impl IntoIterator<Item = &'a ModuleSymbols>) -> BTreeMap<String, ModuleSymbols> {
    modules.into_iter().map(|m| (m.module.clone(), m.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    fn parse(s: &str) -> Vec<Expr> { Parser::new(Lexer::new(s)).parse() }
    #[test] fn exports_functions_and_constants() {
        let ast = parse("export fn add(a,b){ return a+b } export secure const answer = 42");
        let m = analyze_module("math", &ast).unwrap();
        assert!(matches!(m.symbols["add"].kind, SymbolKind::Function));
        assert!(m.symbols["answer"].exported);
        assert!(resolve_export(&m, "answer").is_ok());
        assert_eq!(m.symbols["answer"].qualified_name("math"), "math::answer");
    }
    #[test] fn rejects_duplicate_symbols() {
        let ast = parse("secure let x = 1 secure let x = 2");
        assert!(matches!(analyze_module("m", &ast), Err(e) if matches!(&e[0], SymbolError::Duplicate(n) if n == "x")));
    }
    #[test] fn resolves_qualified_exports() {
        let ast = parse("export secure const answer = 42");
        let m = analyze_module("math", &ast).unwrap();
        let mut ns = BTreeMap::new(); ns.insert("math".into(), m);
        assert!(resolve_qualified(&ns, "math::answer").is_ok());
        assert!(resolve_qualified(&ns, "math::missing").is_err());
    }
}