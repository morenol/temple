use crate::source::SourceLocationInfo;
use std::borrow::Cow;
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
    TemplateNotFound(String),
    TemplateNotParsed,
    InvalidValueType,
    InvalidTemplateName,
    InvalidOperation,
    UndefinedValue(String, SourceLocationInfo),
    ExpectedStringLiteral(SourceLocationInfo),
    ExpectedIdentifier(SourceLocationInfo),
    ExpectedSquareBracket(SourceLocationInfo),
    ExpectedRoundBracket(SourceLocationInfo),
    ExpectedCurlyBracket(SourceLocationInfo),
    ExpectedToken(&'static str, SourceLocationInfo),
    ExpectedExpression(SourceLocationInfo),
    ExpectedEndOfStatement(SourceLocationInfo),
    ExpectedRawEnd(SourceLocationInfo),
    UnexpectedToken(SourceLocationInfo),
    UnexpectedStatement(SourceLocationInfo),
    UnexpectedCommentBegin(SourceLocationInfo),
    UnexpectedCommentEnd(SourceLocationInfo),
    UnexpectedExprBegin(SourceLocationInfo),
    UnexpectedExprEnd(SourceLocationInfo),
    UnexpectedStmtBegin(SourceLocationInfo),
    UnexpectedStmtEnd(SourceLocationInfo),
    UnexpectedRawBegin(SourceLocationInfo),
    UnexpectedRawEnd(SourceLocationInfo),
}

impl ErrorKind {
    fn as_cow_str(&self) -> Cow<'_, str> {
        match &*self {
            ErrorKind::Unspecified => "Unspecified error".into(),
            ErrorKind::YetUnsupported => "Jinja feature not yet supported".into(),
            ErrorKind::FileNotFound => "File not found".into(),
            ErrorKind::ExtensionDisabled => "Extension disabled".into(),
            ErrorKind::TemplateEnvAbsent => "Expected template environment".into(),
            ErrorKind::TemplateNotFound(tmp) => format!("Template {} not found", tmp).into(),
            ErrorKind::TemplateNotParsed => "Template not parsed".into(),
            ErrorKind::InvalidValueType => {
                "Invalid type of the value in the particular context".into()
            }
            ErrorKind::InvalidTemplateName => "Invalid name of the template".into(),
            ErrorKind::InvalidOperation => "Invalid operation".into(),
            //           ErrorKind::MetadataParseError => "Metadata Parse Error ", // TODO: Solve in jinja2cpp
            ErrorKind::UndefinedValue(value, location) => format!(
                "{} error: {} is not defined",
                location.position_log(),
                value
            )
            .into(),
            ErrorKind::ExpectedStringLiteral(_location) => "String literal expected".into(),
            ErrorKind::ExpectedIdentifier(_location) => "Identifier expected".into(),
            ErrorKind::ExpectedSquareBracket(location) => {
                format!("{} error: ']' expected", location.position_log()).into()
            }
            ErrorKind::ExpectedRoundBracket(location) => {
                format!("{} error: ')' expected", location.position_log()).into()
            }
            ErrorKind::ExpectedCurlyBracket(_location) => "'}}' expected".into(),
            ErrorKind::ExpectedToken(s, location) => format!(
                "{} error: Specific token expected ({})",
                location.position_log(),
                s
            )
            .into(),
            ErrorKind::ExpectedExpression(location) => {
                format!("{} error: Expression expected", location.position_log()).into()
            }
            ErrorKind::ExpectedEndOfStatement(_location) => "End of statement expected".into(),
            ErrorKind::ExpectedRawEnd(location) => {
                format!("{} error: {{% endraw %}} expected", location.position_log()).into()
            }
            ErrorKind::UnexpectedToken(location) => {
                format!("{} error: Unexpected token", location.position_log()).into()
            }
            ErrorKind::UnexpectedStatement(_location) => "Unexpected statement".into(),
            ErrorKind::UnexpectedCommentBegin(_location) => {
                "Unexpected comment block begin ('{{#')".into()
            }
            ErrorKind::UnexpectedCommentEnd(location) => format!(
                "{} error: Unexpected comment block end ('#}}')",
                location.position_log()
            )
            .into(),
            ErrorKind::UnexpectedExprBegin(_location) => {
                "Unexpected expression block begin ('{{{{}}')".into()
            }
            ErrorKind::UnexpectedExprEnd(location) => format!(
                "{} error: Unexpected expression block end ('}}}}')",
                location.position_log()
            )
            .into(),
            ErrorKind::UnexpectedStmtBegin(_location) => {
                "Unexpected statement block begin ('{{%')".into()
            }
            ErrorKind::UnexpectedStmtEnd(location) => format!(
                "{} error: Unexpected statement block end ('%}}')",
                location.position_log()
            )
            .into(),
            ErrorKind::UnexpectedRawBegin(location) => format!(
                "{} error: Unexpected raw block begin ('{{% raw %}}')",
                location.position_log()
            )
            .into(),
            ErrorKind::UnexpectedRawEnd(location) => format!(
                "{} error: Unexpected raw block end {{% endraw %}}",
                location.position_log()
            )
            .into(),
        }
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
        write!(f, "{}", self.as_cow_str())
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
