use logos::{Lexer, Logos};
use std::{
    borrow::Cow,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(_: ParseFloatError) -> Self {
        ParseError
    }
}

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(
    error = ParseError,
)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    // One-symbol operators
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("%")]
    Percent,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("(")]
    LBracket,
    #[token(")")]
    RBracket,
    #[token("[")]
    LSqBracket,
    #[token("]")]
    RSqBracket,
    #[token("{")]
    LCrlBracket,
    #[token("}")]
    RCrlBracket,
    #[token("=")]
    Assign,
    #[token(".")]
    Point,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("|")]
    Pipe,
    #[token("~")]
    Tilde,

    // General
    #[regex("[A-Za-z_]+[A-Za-z_0-9]*", |lex| Cow::Borrowed(lex.slice())) ]
    Identifier(Cow<'a, str>),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    IntegerNum(i64),
    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    FloatNum(f64),
    #[regex(r#""(?:[^"]|\\")*""#, |lex| Cow::Borrowed(&lex.slice()[1..lex.slice().len()-1]))]
    #[regex(r#"'(?:[^']|\\')*'"#, |lex| Cow::Borrowed(&lex.slice()[1..lex.slice().len()-1]))]
    String(Cow<'a, str>),

    // Operators
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("++")]
    PlusPlus,
    #[token("--")]
    DashDash,
    #[token("**")]
    MulMul,
    #[token("//")]
    DivDiv,
    #[regex("[Tt]rue")]
    True,
    #[regex("[Ff]alse")]
    False,
    #[token("None")]
    None,

    // Keywords
    #[token("or")]
    LogicalOr,
    #[token("and")]
    LogicalAnd,
    #[token("not")]
    LogicalNot,
    #[token("in")]
    In,
    #[token("is")]
    Is,
    #[token("for")]
    For,
    #[token("endfor")]
    EndFor,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("elif")]
    ElIf,
    #[token("endif")]
    EndIf,
    #[token("block")]
    Block,
    #[token("endblock")]
    EndBlock,
    #[token("extends")]
    Extends,
    #[token("macro")]
    Macro,
    #[token("endmacro")]
    EndMacro,
    #[token("call")]
    Call,
    #[token("endcall")]
    EndCall,
    #[token("filter")]
    Filter,
    #[token("endfilter")]
    EndFilter,
    #[token("set")]
    Set,
    #[token("endset")]
    EndSet,
    #[token("include")]
    Include,
    #[token("import")]
    Import,
    #[token("recursive")]
    Recursive,
    #[token("scoped")]
    Scoped,
    #[token("with")]
    With,
    #[token("endwith")]
    EndWith,
    #[token("without")]
    Without,
    #[token("ignore")]
    Ignore,
    #[token("missing")]
    Missing,
    #[token("context")]
    Context,
    #[token("from")]
    From,
    #[token("as")]
    As,
    #[token("do")]
    Do,

    // Template control
    #[token("{#")]
    CommentBegin,
    #[token("#}")]
    CommentEnd,
    #[token("{{% raw %}}")]
    RawBegin,
    #[token("{{% endraw %}}")]
    RawEnd,
    #[token("{%")]
    StmtBegin,
    #[token("%}")]
    StmtEnd,
    #[token("{{")]
    ExprBegin,
    #[token("}}")]
    ExprEnd,
}

pub struct PeekableLexer<'source, T: Logos<'source>> {
    lexer: Lexer<'source, T>,
    peeked: Option<Option<Result<T, T::Error>>>,
}

impl<'source, T> PeekableLexer<'source, T>
where
    T: Logos<'source>,
{
    pub fn new(lexer: Lexer<'source, T>) -> Self {
        Self {
            lexer,
            peeked: None,
        }
    }
    #[inline]
    pub fn peek(&mut self) -> Option<&Result<T, T::Error>> {
        let lexer = &mut self.lexer;
        self.peeked.get_or_insert_with(|| lexer.next()).as_ref()
    }

    #[inline]
    pub fn span(&self) -> core::ops::Range<usize> {
        self.lexer.span()
    }
}

impl<'source, T> Iterator for PeekableLexer<'source, T>
where
    T: Logos<'source>,
{
    type Item = Result<T, T::Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(token) => token,
            None => self.lexer.next(),
        }
    }
}

#[test]
fn lex_numbers() {
    let tokens: Vec<_> = Token::lexer("1 42 -100 3.18 -77.77").collect();
    assert_eq!(
        tokens,
        &[
            Ok(Token::IntegerNum(1)),
            Ok(Token::IntegerNum(42)),
            Ok(Token::Minus),
            Ok(Token::IntegerNum(100)),
            Ok(Token::FloatNum(3.18)),
            Ok(Token::Minus),
            Ok(Token::FloatNum(77.77)),
        ]
    );
}

#[test]
fn lex_strings() {
    let tokens: Vec<_> = Token::lexer("\"some string\" \"\"").collect();
    assert_eq!(
        tokens,
        &[
            Ok(Token::String(std::borrow::Cow::Borrowed("some string"))),
            Ok(Token::String(std::borrow::Cow::Borrowed(""))),
        ]
    );
}

#[test]
fn lex_math() {
    let tokens: Vec<_> = Token::lexer("(2 + 3 * (5 - 1) + 2 ** 3 / 16) % 5").collect();
    assert_eq!(
        tokens,
        &[
            Ok(Token::LBracket),
            Ok(Token::IntegerNum(2)),
            Ok(Token::Plus),
            Ok(Token::IntegerNum(3)),
            Ok(Token::Mul),
            Ok(Token::LBracket),
            Ok(Token::IntegerNum(5)),
            Ok(Token::Minus),
            Ok(Token::IntegerNum(1)),
            Ok(Token::RBracket),
            Ok(Token::Plus),
            Ok(Token::IntegerNum(2)),
            Ok(Token::MulMul),
            Ok(Token::IntegerNum(3)),
            Ok(Token::Div),
            Ok(Token::IntegerNum(16)),
            Ok(Token::RBracket),
            Ok(Token::Percent),
            Ok(Token::IntegerNum(5)),
        ]
    );
}
