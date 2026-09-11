use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub entry: PathBuf,
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockedDependency { pub name: String, pub version: String, pub source: String, pub checksum: Option<String> }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageLock { pub dependencies: Vec<LockedDependency> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError { Io(String), Invalid(String) }

impl PackageManifest {
    pub fn new(name: impl Into<String>, version: impl Into<String>, entry: impl Into<PathBuf>) -> Self {
        Self { name: name.into(), version: version.into(), entry: entry.into(), dependencies: BTreeMap::new() }
    }
    pub fn add_dependency(&mut self, name: impl Into<String>, requirement: impl Into<String>) { self.dependencies.insert(name.into(), requirement.into()); }
    pub fn validate(&self) -> Result<(), PackageError> {
        if self.name.trim().is_empty() { return Err(PackageError::Invalid("package name is empty".into())); }
        if self.version.trim().is_empty() { return Err(PackageError::Invalid("package version is empty".into())); }
        if self.entry.as_os_str().is_empty() { return Err(PackageError::Invalid("package entry is empty".into())); }
        if self.dependencies.keys().any(|k| k.trim().is_empty()) { return Err(PackageError::Invalid("dependency name is empty".into())); }
        Ok(())
    }
    pub fn from_simple_toml(path: impl AsRef<Path>) -> Result<Self, PackageError> {
        let text = fs::read_to_string(path.as_ref()).map_err(|e| PackageError::Io(e.to_string()))?;
        let mut name = None; let mut version = None; let mut entry = None; let mut dependencies = BTreeMap::new(); let mut in_deps = false;
        for raw in text.lines() {
            let line = raw.trim(); if line.is_empty() || line.starts_with('#') { continue; }
            if line == "[package]" { in_deps = false; continue; }
            if line == "[dependencies]" { in_deps = true; continue; }
            let Some((k,v)) = line.split_once('=') else { return Err(PackageError::Invalid(format!("invalid manifest line: {line}"))); };
            let key = k.trim(); let value = v.trim().trim_matches('"').to_string();
            if in_deps { dependencies.insert(key.to_string(), value); } else { match key { "name" => name=Some(value), "version"=>version=Some(value), "entry"=>entry=Some(PathBuf::from(value)), _=>{} } }
        }
        let m = Self { name:name.ok_or_else(||PackageError::Invalid("missing package.name".into()))?, version:version.ok_or_else(||PackageError::Invalid("missing package.version".into()))?, entry:entry.unwrap_or_else(||PathBuf::from("src/main.falcon")), dependencies };
        m.validate()?; Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn validates_manifest() { let mut m=PackageManifest::new("demo","1.0.0","main.falcon"); m.add_dependency("std","1"); assert!(m.validate().is_ok()); }
    #[test] fn parses_minimal_manifest() { let path=std::env::temp_dir().join(format!("falconcore-{}-manifest.toml",std::process::id())); fs::write(&path,"[package]\nname=\"demo\"\nversion=\"1.0.0\"\nentry=\"main.falcon\"\n[dependencies]\ncore=\"1\"\n").unwrap(); let m=PackageManifest::from_simple_toml(&path).unwrap(); fs::remove_file(path).ok(); assert_eq!(m.dependencies["core"],"1"); }
}