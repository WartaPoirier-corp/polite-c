use std::path::PathBuf;

pub struct Span {
    file: PathBuf,
}

pub struct SpanLabel {
    span: Span,
}

pub enum DiagnosticErrorLevel {
    Warning,
    Error,
}

pub struct Diagnostic {}
