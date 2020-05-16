use enumn::N;
use std::fmt;

macro_rules! define_enum_regex {
    ($Name:ident { $($Variant:ident => $Regex:expr),+ $(,)*}, $Const_name:ident) => {


        #[derive(Debug, N)]
        #[repr(usize)]
        pub enum $Name {
            $($Variant),*,
        }

        impl $Name{
            fn as_str(&self) -> &str {
                match *self {
                    $($Name::$Variant => $Regex),
                    *,
                 }
            }
        }

        impl fmt::Display for $Name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }

        pub const $Const_name: &'static str = concat!($("(", $Regex, ")|"),*);
    }
}

define_enum_regex!(
RegexEnum {
    ExprBegin => r"\{\{",
    ExprEnd => r"\}\}",
    RawBegin => r"\{%[\+\-]?\s+raw\s+[\+\-]?%\}",
    RawEnd => r"\{%[\+\-]?\s+endraw\s+[\+\-]?%\}",
    StmtBegin  => r"\{%",
    StmtEnd => r"%\}",
    CommentBegin => r"\{#",
    CommentEnd => r"#\}",
    NewLine => r"\n",
}, ROUGH_TOKENIZER
);

define_enum_regex!(
Keyword {
    LogicalOr => "or",
    LogicalAnd => "and",
    LogicalNot => "!",
    True => "true",
    False => "false",
    None => "None",
    In => "in",
    Is => "is",
    For => "fpr",
    Endfor => "endfor",
    If => "if",
    Else => "else",
    ElIf => "elif",
    EndIf => "endif",
    Block => "block",
    EndBlock => "endblock",
    Extends => "extends",
    Macro => "macro",
    EndMacro => "endmacro",
    Call => "call",
    EndCall => "endcall",
    Filter => "filter",
    EndFilter => "endfilter",
    Set => "set",
    EndSet => "endset",
    Include => "include",
    Import => "import",
    Recursive => "recursive",
    Scoped => "scoped",
    With => "with",
    EndWith => "endwith",
    Without => "without",
    Ignore => "ignore",
    Missing => "missing",
    Context => "context",
    From => "from",
    As => "as",
    Do => "do",
}, KEYWORDS

);
