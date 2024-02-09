use std::rc::Rc;

use logos::{Lexer, Logos};

use crate::error::{Error, ParseError, ParseErrorKind, Result};
use crate::expression_parser::ExpressionParser;
use crate::lexer::{PeekableLexer, Token};
use crate::renderer::ComposedRenderer;
use crate::source::SourceLocationInfo;
use crate::statement::Evaluate;

use super::{
    ElseStatement, ForStatement, IfStatement, IncludeStatement, Statement, StatementInfo,
    StatementInfoList, StatementInfoType, WithStatement,
};

pub struct StatementParser;

impl StatementParser {
    pub fn parse<'a>(text: &'a str, statementinfo_list: &mut StatementInfoList<'a>) -> Result<()> {
        let lexer: Lexer<'_, Token<'a>> = Token::lexer(text);
        let mut lexer = PeekableLexer::new(lexer);
        let tok = lexer.next();

        match tok {
            Some(Ok(Token::If)) => StatementParser::parse_if(&mut lexer, statementinfo_list),
            Some(Ok(Token::Else)) => {
                StatementParser::parse_else(statementinfo_list);
                Ok(())
            }
            Some(Ok(Token::EndIf)) => StatementParser::parse_endif(&mut lexer, statementinfo_list),
            Some(Ok(Token::ElIf)) => StatementParser::parse_elif(&mut lexer, statementinfo_list),
            Some(Ok(Token::For)) => StatementParser::parse_for(&mut lexer, statementinfo_list),
            Some(Ok(Token::EndFor)) => {
                StatementParser::parse_endfor(&mut lexer, statementinfo_list)
            }
            Some(Ok(Token::With)) => StatementParser::parse_with(&mut lexer, statementinfo_list),
            Some(Ok(Token::EndWith)) => {
                StatementParser::parse_endwith(&mut lexer, statementinfo_list)
            }
            Some(Ok(Token::Include)) => {
                StatementParser::parse_include(&mut lexer, statementinfo_list)
            }
            Some(_) => {
                let range = lexer.span();
                Err(Error::from(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                )))
            }
            _ => todo!(),
        }
    }
    fn parse_if<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let value = ExpressionParser::full_expresion_parser(lexer)?;
        let composed_renderer = Rc::new(ComposedRenderer::new());
        let renderer = Statement::If(IfStatement::new(Box::new(value)));
        let mut statement_info = StatementInfo::new(
            StatementInfoType::IfStatement,
            Some(Token::If),
            composed_renderer,
        );
        statement_info.renderer = Some(renderer);

        statementinfo_list.push(statement_info);
        Ok(())
    }
    fn parse_elif<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let value = ExpressionParser::full_expresion_parser(lexer)?;
        let composed_renderer = Rc::new(ComposedRenderer::new());
        let renderer = Statement::Else(ElseStatement::new(Some(Box::new(value))));
        let mut statement_info = StatementInfo::new(
            StatementInfoType::ElseIfStatement,
            Some(Token::Else),
            composed_renderer,
        );
        statement_info.renderer = Some(renderer);
        statementinfo_list.push(statement_info);
        Ok(())
    }

    fn parse_else(statementinfo_list: &mut StatementInfoList<'_>) {
        let composed_renderer = Rc::new(ComposedRenderer::new());
        let renderer = Statement::Else(ElseStatement::new(None));
        let mut statement_info = StatementInfo::new(
            StatementInfoType::ElseIfStatement,
            Some(Token::Else),
            composed_renderer,
        );
        statement_info.renderer = Some(renderer);
        statementinfo_list.push(statement_info);
    }
    fn parse_endif<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,

        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        if statementinfo_list.len() <= 1 {
            let range = lexer.span();
            return Err(Error::from(ParseErrorKind::UnexpectedStatement(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )));
        }
        let mut info;
        let mut else_branches = vec![];
        loop {
            info = statementinfo_list.pop().unwrap();
            match info.mode {
                StatementInfoType::IfStatement => {
                    break;
                }
                StatementInfoType::ElseIfStatement => {
                    let mut renderer = info.renderer.unwrap();
                    renderer.set_main_body(info.compositions.remove(0));
                    else_branches.push(renderer);
                }
                _ => todo!(),
            }
        }
        let mut renderer = info.renderer.unwrap();
        let body = info.compositions.remove(0);
        renderer.set_main_body(body);
        for else_branch in else_branches.into_iter().rev() {
            renderer.add_else_branch(else_branch);
        }
        statementinfo_list
            .last_mut()
            .unwrap()
            .current_composition
            .add_renderer(Box::new(renderer));
        Ok(())
    }
    fn parse_for<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let mut vars = vec![];
        loop {
            if let Some(Ok(Token::Identifier(identifier))) = lexer.next() {
                vars.push(identifier.to_string());
            } else {
                let range = lexer.span();
                return Err(Error::from(ParseErrorKind::ExpectedIdentifier(
                    SourceLocationInfo::new_with_range(range.start, range.end),
                )));
            }
            if let Some(Ok(Token::Comma)) = lexer.peek() {
                lexer.next();
            } else {
                break;
            }
        }
        if let Some(Ok(Token::In)) = lexer.next() {
            let expression = ExpressionParser::full_expresion_parser(lexer)?;
            if lexer.next().is_some() {
                let range = lexer.span();
                Err(Error::from(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                )))
            } else {
                let composed_renderer = Rc::new(ComposedRenderer::new());
                let renderer = Statement::For(ForStatement::new(vars, Box::new(expression)));
                let mut statement_info = StatementInfo::new(
                    StatementInfoType::ForStatement,
                    Some(Token::For),
                    composed_renderer,
                );
                statement_info.renderer = Some(renderer);
                statementinfo_list.push(statement_info);
                Ok(())
            }
        } else {
            let range = lexer.span();
            Err(Error::from(ParseError::new(
                ParseErrorKind::ExpectedToken("in"),
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )))
        }
    }
    fn parse_endfor<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        if statementinfo_list.len() <= 1 {
            let range = lexer.span();
            return Err(Error::from(ParseErrorKind::UnexpectedStatement(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )));
        }
        let mut info = statementinfo_list.pop().unwrap();
        if let StatementInfoType::ForStatement = info.mode {
            let mut renderer = info.renderer.unwrap();
            let body = info.compositions.remove(0);
            renderer.set_main_body(body);
            statementinfo_list
                .last_mut()
                .unwrap()
                .current_composition
                .add_renderer(Box::new(renderer));
            Ok(())
        } else {
            let range = lexer.span();
            Err(Error::from(ParseErrorKind::UnexpectedStatement(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )))
        }
    }
    fn parse_with<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let mut vars: Vec<(String, Box<dyn Evaluate + 'a>)> = vec![];
        while let Some(Ok(Token::Identifier(identifier))) = lexer.next() {
            let value = if let Some(Ok(Token::Assign)) = lexer.peek() {
                lexer.next();
                ExpressionParser::full_expresion_parser(lexer)?
            } else {
                let range = lexer.span();
                return Err(Error::from(ParseError::new(
                    ParseErrorKind::ExpectedToken("="),
                    Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                )));
            };
            vars.push((identifier.to_string(), Box::new(value)));
            if let Some(Ok(Token::Comma)) = lexer.peek() {
                lexer.next();
            } else {
                break;
            }
        }
        if vars.is_empty() {
            let range = lexer.span();
            return Err(Error::from(ParseErrorKind::ExpectedIdentifier(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )));
        }
        if lexer.peek().is_some() {
            let range = lexer.span();
            return Err(Error::from(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )));
        }
        let composed_renderer = Rc::new(ComposedRenderer::new());
        let renderer = Statement::With(WithStatement::new(vars));
        let mut statement_info = StatementInfo::new(
            StatementInfoType::WithStatement,
            Some(Token::With),
            composed_renderer,
        );
        statement_info.renderer = Some(renderer);
        statementinfo_list.push(statement_info);
        Ok(())
    }
    fn parse_endwith<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        if statementinfo_list.len() <= 1 {
            let range = lexer.span();
            return Err(Error::from(ParseErrorKind::UnexpectedStatement(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )));
        }
        let mut info = statementinfo_list.pop().unwrap();
        if let StatementInfoType::WithStatement = info.mode {
            let mut renderer = info.renderer.unwrap();
            let body = info.compositions.remove(0);
            renderer.set_main_body(body);
            statementinfo_list
                .last_mut()
                .unwrap()
                .current_composition
                .add_renderer(Box::new(renderer));
            Ok(())
        } else {
            let range = lexer.span();
            Err(Error::from(ParseErrorKind::UnexpectedStatement(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )))
        }
    }
    fn parse_include<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        if statementinfo_list.is_empty() {
            let range = lexer.span();
            return Err(Error::from(ParseErrorKind::UnexpectedStatement(
                SourceLocationInfo::new_with_range(range.start, range.end),
            )));
        }
        let expr = ExpressionParser::full_expresion_parser(lexer)?;
        let mut is_ignore_missing = false;
        let mut is_with_context = true;

        if let Some(Ok(Token::Ignore)) = lexer.peek() {
            lexer.next();
            if let Some(Ok(Token::Missing)) = lexer.peek() {
                is_ignore_missing = true;
            } else {
                let range = lexer.span();
                return Err(Error::from(ParseError::new(
                    ParseErrorKind::ExpectedToken("missing"),
                    Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                )));
            }
            lexer.next();
        }

        match lexer.next() {
            Some(Ok(Token::With)) => {
                if let Some(Ok(Token::Context)) = lexer.peek() {
                    lexer.next();
                } else {
                    let range = lexer.span();
                    return Err(Error::from(ParseError::new(
                        ParseErrorKind::ExpectedToken("context"),
                        Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                    )));
                }
            }
            Some(Ok(Token::Without)) => {
                is_with_context = false;
                if let Some(Ok(Token::Context)) = lexer.peek() {
                    lexer.next();
                } else {
                    let range = lexer.span();
                    return Err(Error::from(ParseError::new(
                        ParseErrorKind::ExpectedToken("context"),
                        Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                    )));
                }
            }
            None => {}
            _ => {
                let range = lexer.span();
                return Err(Error::from(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                )));
            }
        }
        if lexer.next().is_some() {
            let range = lexer.span();
            return Err(Error::from(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )));
        }
        let renderer = Statement::Include(IncludeStatement::new(
            is_ignore_missing,
            is_with_context,
            Box::new(expr),
        ));
        statementinfo_list
            .last_mut()
            .unwrap()
            .current_composition
            .add_renderer(Box::new(renderer));

        Ok(())
    }
}
