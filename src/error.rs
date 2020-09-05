use crate::source::SourceLocationInfo;
use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    ParseError(ParseErrorKind),
    RenderError(RenderErrorKind),
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseErrorKind {
    Unspecified,
    YetUnsupported,
    ExtensionDisabled,
    TemplateEnvAbsent,
    TemplateNotFound(String),
    InvalidTemplateName,
    UndefinedValue(String, SourceLocationInfo),
    ExpectedStringLiteral(SourceLocationInfo),
    ExpectedIdentifier(SourceLocationInfo),
    ExpectedBracket(&'static str, SourceLocationInfo),
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

impl ParseErrorKind {
    fn as_cow_str(&self) -> Cow<'_, str> {
        match &*self {
            ParseErrorKind::Unspecified => "Unspecified error".into(),
            ParseErrorKind::YetUnsupported => "Jinja feature not yet supported".into(),
            ParseErrorKind::ExtensionDisabled => "Extension disabled".into(),
            ParseErrorKind::TemplateEnvAbsent => "Expected template environment".into(),
            ParseErrorKind::TemplateNotFound(tmp) => format!("Template {} not found", tmp).into(),
            ParseErrorKind::InvalidTemplateName => "Invalid name of the template".into(),
            //           ParseErrorKind::MetadataParseError => "Metadata Parse Error ", // TODO: Solve in jinja2cpp
            ParseErrorKind::UndefinedValue(value, location) => format!(
                "{} error: {} is not defined",
                location.position_log(),
                value
            )
            .into(),
            ParseErrorKind::ExpectedStringLiteral(_location) => "String literal expected".into(),
            ParseErrorKind::ExpectedIdentifier(_location) => "Identifier expected".into(),
            ParseErrorKind::ExpectedBracket(bracket, location) => {
                format!("{} error: '{}' expected", location.position_log(), bracket).into()
            }
            ParseErrorKind::ExpectedToken(s, location) => format!(
                "{} error: Specific token expected ({})",
                location.position_log(),
                s
            )
            .into(),
            ParseErrorKind::ExpectedExpression(location) => {
                format!("{} error: Expression expected", location.position_log()).into()
            }
            ParseErrorKind::ExpectedEndOfStatement(_location) => "End of statement expected".into(),
            ParseErrorKind::ExpectedRawEnd(location) => {
                format!("{} error: {{% endraw %}} expected", location.position_log()).into()
            }
            ParseErrorKind::UnexpectedToken(location) => {
                format!("{} error: Unexpected token", location.position_log()).into()
            }
            ParseErrorKind::UnexpectedStatement(_location) => "Unexpected statement".into(),
            ParseErrorKind::UnexpectedCommentBegin(_location) => {
                "Unexpected comment block begin ('{{#')".into()
            }
            ParseErrorKind::UnexpectedCommentEnd(location) => format!(
                "{} error: Unexpected comment block end ('#}}')",
                location.position_log()
            )
            .into(),
            ParseErrorKind::UnexpectedExprBegin(_location) => {
                "Unexpected expression block begin ('{{{{}}')".into()
            }
            ParseErrorKind::UnexpectedExprEnd(location) => format!(
                "{} error: Unexpected expression block end ('}}}}')",
                location.position_log()
            )
            .into(),
            ParseErrorKind::UnexpectedStmtBegin(_location) => {
                "Unexpected statement block begin ('{{%')".into()
            }
            ParseErrorKind::UnexpectedStmtEnd(location) => format!(
                "{} error: Unexpected statement block end ('%}}')",
                location.position_log()
            )
            .into(),
            ParseErrorKind::UnexpectedRawBegin(location) => format!(
                "{} error: Unexpected raw block begin ('{{% raw %}}')",
                location.position_log()
            )
            .into(),
            ParseErrorKind::UnexpectedRawEnd(location) => format!(
                "{} error: Unexpected raw block end {{% endraw %}}",
                location.position_log()
            )
            .into(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RenderErrorKind {
    FileNotFound,
    TemplateNotParsed,
    InvalidOperation,
    InvalidValueType,
}

impl RenderErrorKind {
    fn as_cow_str(&self) -> Cow<'_, str> {
        match &*self {
            RenderErrorKind::FileNotFound => "File not found".into(),
            RenderErrorKind::TemplateNotParsed => "Template not parsed".into(),
            RenderErrorKind::InvalidOperation => "Invalid operation".into(),
            RenderErrorKind::InvalidValueType => {
                "Invalid type of the value in the particular context".into()
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::ParseError(ref err) => err.fmt(f),
            Error::RenderError(ref err) => err.fmt(f),
        }
    }
}
impl error::Error for ParseErrorKind {}
impl error::Error for RenderErrorKind {}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => err.source(),
            Error::ParseError(ref err) => Some(err),
            Error::RenderError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_cow_str())
    }
}
impl fmt::Display for RenderErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_cow_str())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ParseErrorKind> for Error {
    fn from(err: ParseErrorKind) -> Error {
        Error::ParseError(err)
    }
}

impl From<RenderErrorKind> for Error {
    fn from(err: RenderErrorKind) -> Error {
        Error::RenderError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
