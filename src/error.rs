use crate::source::SourceLocationInfo;
use std::io;

use thiserror::Error as ThisError;

#[non_exhaustive]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    ParseError(#[from] ParseErrorKind),
    #[error("{0}")]
    RenderError(#[from] RenderErrorKind),
}

#[derive(ThisError, Debug)]
#[error("{location} error: {kind}")]
struct ParseError {
    location: SourceLocationInfo,
    #[source]
    kind: ParseErrorKind,
}

#[non_exhaustive]
#[derive(Debug, ThisError)]
pub enum ParseErrorKind {
    #[error("Unspecified error.")]
    Unspecified,
    #[error("Jinja feature not yet supported.")]
    YetUnsupported,
    #[error("Extension disabled.")]
    ExtensionDisabled,
    #[error("Expected template environment")]
    TemplateEnvAbsent,
    #[error("Template {0} not found.")]
    TemplateNotFound(String),
    #[error("Invalid name of template.")]
    InvalidTemplateName,
    #[error("{1} error: {0} is not defined.")]
    UndefinedValue(String, SourceLocationInfo),
    #[error("String literal expected.")]
    ExpectedStringLiteral(SourceLocationInfo),
    #[error("Identifier expected")]
    ExpectedIdentifier(SourceLocationInfo),
    #[error("{1} error: '{0}' expected")]
    ExpectedBracket(&'static str, SourceLocationInfo),
    #[error("{1} error: Specific token expected ({0})")]
    ExpectedToken(&'static str, SourceLocationInfo),
    #[error("{0} error: Expression expected")]
    ExpectedExpression(SourceLocationInfo),
    #[error("End of statement expected")]
    ExpectedEndOfStatement(SourceLocationInfo),
    #[error("{0} error: {{% endraw %}} expected")]
    ExpectedRawEnd(SourceLocationInfo),
    #[error("{0} error: Unexpected token")]
    UnexpectedToken(SourceLocationInfo),
    #[error("Unexpected statement")]
    UnexpectedStatement(SourceLocationInfo),
    #[error("Unexpected comment block begin ('{{#')")]
    UnexpectedCommentBegin(SourceLocationInfo),
    #[error("{0} error: Unexpected comment block end ('#}}')")]
    UnexpectedCommentEnd(SourceLocationInfo),
    #[error("Unexpected expression block begin ('{{{{}}")]
    UnexpectedExprBegin(SourceLocationInfo),
    #[error("{0} error: Unexpected expression block end ('}}}}')")]
    UnexpectedExprEnd(SourceLocationInfo),
    #[error("Unexpected statement block begin ('{{%')")]
    UnexpectedStmtBegin(SourceLocationInfo),
    #[error("{0} error: Unexpected statement block end ('%}}')")]
    UnexpectedStmtEnd(SourceLocationInfo),
    #[error("{0} error: Unexpected raw block begin ('{{% raw %}}')")]
    UnexpectedRawBegin(SourceLocationInfo),
    #[error("{0} error: Unexpected raw block end {{% endraw %}}")]
    UnexpectedRawEnd(SourceLocationInfo),
}

#[non_exhaustive]
#[derive(Debug, ThisError)]
pub enum RenderErrorKind {
    #[error("File not found")]
    FileNotFound,
    #[error("Template not parsed")]
    TemplateNotParsed,
    #[error("Invalid operation")]
    InvalidOperation,
    #[error("Invalid type of the value in the particular context")]
    InvalidValueType,
}

pub type Result<T> = std::result::Result<T, Error>;
