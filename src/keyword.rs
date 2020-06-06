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
