use enumn::N;

macro_rules! define_enum_regex {
    ($Name:ident { $($Variant:ident => $Regex:expr),+ $(,)*}, $Const_name:ident) => {


        #[derive(Debug, N)]
        #[repr(usize)]
        pub enum $Name {
            $($Variant),*,
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
