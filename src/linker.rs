use crate::{module::Module, parser::Expr};
use std::{collections::{BTreeMap,BTreeSet}, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkError { EmptyInput, DuplicateModule(String), InvalidModule(String), MissingQualifiedSymbol(String) }

/// Flattens a resolved module graph into one AST while preserving module boundaries
/// through qualified names such as `math::add`. Only exported declarations from
/// imported modules are exposed to the entry module.
pub fn link_modules(modules: &[Module], entry: impl AsRef<Path>) -> Result<Vec<Expr>, LinkError> {
    if modules.is_empty() { return Err(LinkError::EmptyInput); }
    let entry = std::fs::canonicalize(entry.as_ref()).map_err(|_| LinkError::InvalidModule(entry.as_ref().display().to_string()))?;
    let mut namespaces = BTreeMap::<String, BTreeSet<String>>::new();
    for module in modules {
        let ns = module.path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
        if ns.is_empty() { return Err(LinkError::InvalidModule(module.path.display().to_string())); }
        if namespaces.insert(ns.clone(), exported_names(&module.ast)).is_some() { return Err(LinkError::DuplicateModule(ns)); }
    }
    let mut out = Vec::new();
    for module in modules {
        let ns = module.path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
        let is_entry = module.path == entry;
        let mut aliases = BTreeMap::new();
        for expr in &module.ast {
            if let Expr::Import { path, alias } = expr {
                let target = Path::new(path).file_stem().and_then(|s| s.to_str()).unwrap_or(path).to_string();
                aliases.insert(alias.clone().unwrap_or_else(|| target.clone()), target);
            }
        }
        for expr in &module.ast {
            if matches!(expr, Expr::Import { .. }) { continue; }
            if !is_entry && !is_exported_expr(expr, &namespaces[&ns]) { continue; }
            out.push(transform(expr, &ns, &namespaces[&ns], &aliases, is_entry));
        }
    }
    Ok(out)
}

fn exported_names(ast: &[Expr]) -> BTreeSet<String> {
    ast.iter().filter_map(|e| match e { Expr::Export { item } => match item.as_ref() { Expr::Let{name,..}|Expr::FnDef{name,..} => Some(name.clone()), _ => None }, _ => None }).collect()
}
fn is_exported_expr(e: &Expr, exports: &BTreeSet<String>) -> bool { match e { Expr::Export{item} => matches!(item.as_ref(),Expr::Let{name,..}|Expr::FnDef{name,..} if exports.contains(name)), _ => false } }
fn transform(e:&Expr, ns:&str, exports:&BTreeSet<String>, aliases:&BTreeMap<String,String>, entry:bool)->Expr {
    match e {
        Expr::Export{item} => transform(item,ns,exports,aliases,entry),
        Expr::FnDef{name,params,param_types,return_type,body} => Expr::FnDef{name: if entry{name.clone()}else{format!("{ns}::{name}")},params:params.clone(),param_types:param_types.clone(),return_type:return_type.clone(),body:body.iter().map(|x| transform(x,ns,exports,aliases,entry)).collect()},
        Expr::Let{is_secure,is_const,name,value} => Expr::Let{is_secure:*is_secure,is_const:*is_const,name: if entry{name.clone()}else{format!("{ns}::{name}")},value:Box::new(transform(value,ns,exports,aliases,entry))},
        Expr::Identifier(name) => Expr::Identifier(resolve_ref(name,ns,exports,aliases)),
        Expr::Call{name,args} => Expr::Call{name:resolve_ref(name,ns,exports,aliases),args:args.iter().map(|x|transform(x,ns,exports,aliases,entry)).collect()},
        Expr::Binary{left,op,right} => Expr::Binary{left:Box::new(transform(left,ns,exports,aliases,entry)),op:op.clone(),right:Box::new(transform(right,ns,exports,aliases,entry))},
        Expr::Print{expr}=>Expr::Print{expr:Box::new(transform(expr,ns,exports,aliases,entry))},
        Expr::If{condition,then_branch,else_branch}=>Expr::If{condition:Box::new(transform(condition,ns,exports,aliases,entry)),then_branch:then_branch.iter().map(|x|transform(x,ns,exports,aliases,entry)).collect(),else_branch:else_branch.as_ref().map(|b|b.iter().map(|x|transform(x,ns,exports,aliases,entry)).collect())},
        Expr::Repeat{times,body}=>Expr::Repeat{times:Box::new(transform(times,ns,exports,aliases,entry)),body:body.iter().map(|x|transform(x,ns,exports,aliases,entry)).collect()},
        Expr::Return{value}=>Expr::Return{value:value.as_ref().map(|x|Box::new(transform(x,ns,exports,aliases,entry)))},
        Expr::NetworkScan{subnet}=>Expr::NetworkScan{subnet:Box::new(transform(subnet,ns,exports,aliases,entry))},
        _=>e.clone(),
    }
}
fn resolve_ref(name:&str,ns:&str,exports:&BTreeSet<String>,aliases:&BTreeMap<String,String>)->String {
    if name.contains("::") { let mut p=name.splitn(2,"::"); let a=p.next().unwrap(); let b=p.next().unwrap(); return format!("{}::{b}",aliases.get(a).unwrap_or(&a.to_string())); }
    if exports.contains(name) { format!("{ns}::{name}") } else { name.to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    fn parse(s:&str)->Vec<Expr>{Parser::new(Lexer::new(s)).parse()}
    #[test] fn qualified_calls_are_preserved_and_module_exports_are_namespaced(){
        let root=std::env::temp_dir().join(format!("falconcore-link-{}",std::process::id())); std::fs::create_dir_all(&root).unwrap();
        let main=root.join("main.falcon"); let math=root.join("math.falcon");
        std::fs::write(&main,"import \"math.falcon\" as math print math::add(2,3)").unwrap();
        std::fs::write(&math,"export fn add(a,b){ return a+b }").unwrap();
        let mut r=crate::module::ModuleResolver::new(); let modules=r.resolve(&main).unwrap(); let linked=link_modules(&modules,&main).unwrap();
        assert!(linked.iter().any(|e|matches!(e,Expr::FnDef{name,..} if name=="math::add"))); assert!(linked.iter().any(|e|matches!(e,Expr::Print{expr} if matches!(expr.as_ref(),Expr::Call{name,..} if name=="math::add"))));
        let _=std::fs::remove_dir_all(root);
    }
}
