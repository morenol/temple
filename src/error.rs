use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseRender(ErrorKind),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    Unspecified,
    YetUnsupported,
    FileNotFound,
    ExtensionDisabled,
    TemplateEnvAbsent,
    TemplateNotFound,
    TemplateNotParsed,
    InvalidValueType,
    InvalidTemplateName,
    InvalidOperation,
    UndefinedValue(SourceLocation),
    ExpectedStringLiteral(SourceLocation),
    ExpectedIdentifier(SourceLocation),
    ExpectedSquareBracket(SourceLocation),
    ExpectedRoundBracket(SourceLocation),
    ExpectedCurlyBracket(SourceLocation),
    ExpectedToken(SourceLocation),
    ExpectedExpression(SourceLocation),
    ExpectedEndOfStatement(SourceLocation),
    ExpectedRawEnd(SourceLocation),
    UnexpectedToken(SourceLocation),
    UnexpectedStatement(SourceLocation),
    UnexpectedCommentBegin(SourceLocation),
    UnexpectedCommentEnd(SourceLocation),
    UnexpectedExprBegin(SourceLocation),
    UnexpectedExprEnd(SourceLocation),
    UnexpectedStmtBegin(SourceLocation),
    UnexpectedStmtEnd(SourceLocation),
    UnexpectedRawBegin(SourceLocation),
    UnexpectedRawEnd(SourceLocation),
}

impl ErrorKind {
    fn as_str(&self) -> &str {
        match &*self {
            ErrorKind::Unspecified => "Unspecified error",
            ErrorKind::YetUnsupported => "Jinja feature not yet supported ",
            ErrorKind::FileNotFound => "File not found",
            ErrorKind::ExtensionDisabled => "Extension disabled ",
            ErrorKind::TemplateEnvAbsent => "Expected template environment",
            ErrorKind::TemplateNotFound => "Template not found",
            ErrorKind::TemplateNotParsed => "Template not parsed",
            ErrorKind::InvalidValueType => "Invalid type of the value in the particular context",
            ErrorKind::InvalidTemplateName => "Invalid name of the template",
            ErrorKind::InvalidOperation => "Invalid operation",
            //           ErrorKind::MetadataParseError => "Metadata Parse Error ", // TODO: Solve in jinja2cpp
            ErrorKind::UndefinedValue(_location) => "Value is not defined",
            ErrorKind::ExpectedStringLiteral(_location) => "String literal expected",
            ErrorKind::ExpectedIdentifier(_location) => "Identifier expected",
            ErrorKind::ExpectedSquareBracket(_location) => "']' expected",
            ErrorKind::ExpectedRoundBracket(_location) => "')' expected",
            ErrorKind::ExpectedCurlyBracket(_location) => "'}}' expected",
            ErrorKind::ExpectedToken(_location) => "Specific token(s) expected",
            ErrorKind::ExpectedExpression(_location) => "Expression expected",
            ErrorKind::ExpectedEndOfStatement(_location) => "End of statement expected",
            ErrorKind::ExpectedRawEnd(_location) => "{{% endraw %}} expected",
            ErrorKind::UnexpectedToken(_location) => "Unexpected token",
            ErrorKind::UnexpectedStatement(_location) => "Unexpected statement",
            ErrorKind::UnexpectedCommentBegin(_location) => {
                "Unexpected comment block begin ('{{#')"
            }
            ErrorKind::UnexpectedCommentEnd(_location) => "Unexpected comment block end ('#}}')",
            ErrorKind::UnexpectedExprBegin(_location) => {
                "Unexpected expression block begin ('{{{{}}')"
            }
            ErrorKind::UnexpectedExprEnd(_location) => "Unexpected expression block end ('}}}}')",
            ErrorKind::UnexpectedStmtBegin(_location) => "Unexpected statement block begin ('{{%')",
            ErrorKind::UnexpectedStmtEnd(_location) => "Unexpected statement block end ('%}}')",
            ErrorKind::UnexpectedRawBegin(_location) => "Unexpected raw block begin {{% raw %}}",
            ErrorKind::UnexpectedRawEnd(_location) => "Unexpected raw block end {{% endraw %}}",
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceLocation {
    /// Line number (1-based)
    line: usize,
    /// Column number (1-based)
    col: usize,
}

impl SourceLocation {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::ParseRender(ref err) => err.fmt(f),
        }
    }
}
impl error::Error for ErrorKind {}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => err.source(),
            Error::ParseRender(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ErrorKind> for Error {
    fn from(err: ErrorKind) -> Error {
        Error::ParseRender(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
