use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub const fn new(line: usize, column: usize) -> Self { Self { line, column } }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity { Error, Warning, Note }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: &'static str,
    pub severity: Severity,
    pub message: String,
    pub span: Option<Span>,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, severity: Severity::Error, message: message.into(), span: None, help: None }
    }

    pub fn at(mut self, line: usize, column: usize) -> Self {
        self.span = Some(Span::new(line, column));
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self.severity { Severity::Error => "error", Severity::Warning => "warning", Severity::Note => "note" };
        write!(f, "{severity}[{}]: {}", self.code, self.message)?;
        if let Some(span) = self.span { write!(f, " at {}:{}", span.line, span.column)?; }
        if let Some(help) = &self.help { write!(f, "\n  help: {help}")?; }
        Ok(())
    }
}

pub fn format_diagnostics(diagnostics: &[Diagnostic]) -> String {
    diagnostics.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
}
