use crate::source::SourceLocationInfo;
use std::io;

use thiserror::Error as ThisError;

#[non_exhaustive]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    ParseError(#[from] ParseError),
    #[error("{0}")]
    RenderError(#[from] RenderErrorKind),
}

#[derive(ThisError, Debug)]
#[error("{location} error: {kind}")]
pub struct ParseError {
    pub location: SourceLocationInfo,
    #[source]
    pub kind: ParseErrorKind,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, location: Option<SourceLocationInfo>) -> Self {
        Self {
            kind,
            location: location.unwrap_or_default(),
        }
    }
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
    #[error("{0} is not defined.")]
    UndefinedValue(String),
    #[error("Invalid name of template.")]
    InvalidTemplateName,
    #[error("String literal expected.")]
    ExpectedStringLiteral(SourceLocationInfo),
    #[error("Identifier expected")]
    ExpectedIdentifier(SourceLocationInfo),
    #[error("'{0}' expected")]
    ExpectedBracket(&'static str),
    #[error("Specific token expected ({0})")]
    ExpectedToken(&'static str),
    #[error("Expression expected")]
    ExpectedExpression,
    #[error("End of statement expected")]
    ExpectedEndOfStatement(SourceLocationInfo),
    #[error("{{% endraw %}} expected")]
    ExpectedRawEnd,
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Unexpected statement")]
    UnexpectedStatement(SourceLocationInfo),
    #[error("Unexpected comment block begin ('{{#')")]
    UnexpectedCommentBegin(SourceLocationInfo),
    #[error("Unexpected comment block end ('#}}')")]
    UnexpectedCommentEnd,
    #[error("Unexpected expression block begin ('{{{{}}")]
    UnexpectedExprBegin(SourceLocationInfo),
    #[error("Unexpected expression block end ('}}}}')")]
    UnexpectedExprEnd,
    #[error("Unexpected statement block begin ('{{%')")]
    UnexpectedStmtBegin(SourceLocationInfo),
    #[error("Unexpected statement block end ('%}}')")]
    UnexpectedStmtEnd,
    #[error("Unexpected raw block begin ('{{% raw %}}')")]
    UnexpectedRawBegin,
    #[error("Unexpected raw block end {{% endraw %}}")]
    UnexpectedRawEnd,
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
impl ParseError {
    pub fn set_location(&mut self, location: SourceLocationInfo) {
        self.location = location;
    }
}

impl From<ParseErrorKind> for ParseError {
    fn from(kind: ParseErrorKind) -> Self {
        Self {
            location: SourceLocationInfo::default(),
            kind,
        }
    }
}

impl From<ParseErrorKind> for Error {
    fn from(kind: ParseErrorKind) -> Self {
        Self::from(ParseError::from(kind))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
