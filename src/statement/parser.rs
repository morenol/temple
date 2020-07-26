use super::{
    ElseStatement, ForStatement, IfStatement, Statement, StatementInfo, StatementInfoList,
    StatementInfoType,
};
use crate::error::{Error, ErrorKind, Result, SourceLocation};
use crate::expression_parser::ExpressionParser;
use crate::lexer::Token;
use crate::renderer::ComposedRenderer;
use logos::{Lexer, Logos};
use std::iter::Peekable;
pub struct StatementParser;
use std::sync::Arc;

impl StatementParser {
    pub fn parse<'a>(
        text: &'a str,
        mut statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let lexer: Lexer<Token<'a>> = Token::lexer(text);
        let mut lexer: Peekable<Lexer<Token<'a>>> = lexer.peekable();
        let tok = lexer.next();

        match tok {
            Some(Token::If) => StatementParser::parse_if(&mut lexer, &mut statementinfo_list),
            Some(Token::Else) => StatementParser::parse_else(&mut statementinfo_list),
            Some(Token::EndIf) => StatementParser::parse_endif(&mut statementinfo_list),
            Some(Token::ElIf) => StatementParser::parse_elif(&mut lexer, &mut statementinfo_list),
            Some(Token::For) => StatementParser::parse_for(&mut lexer, &mut statementinfo_list),
            Some(Token::EndFor) => {
                StatementParser::parse_endfor(&mut lexer, &mut statementinfo_list)
            }

            _ => todo!(),
        }
    }
    fn parse_if<'a>(
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let value = ExpressionParser::full_expresion_parser(lexer)?;
        let composed_renderer = Arc::new(ComposedRenderer::new());
        let renderer = Statement::If(IfStatement::new(Box::new(value)));
        let mut statement_info =
            StatementInfo::new(StatementInfoType::IfStatement, Token::If, composed_renderer);
        statement_info.renderer = Some(renderer);

        statementinfo_list.push(statement_info);
        Ok(())
    }
    fn parse_elif<'a>(
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let value = ExpressionParser::full_expresion_parser(lexer)?;
        let composed_renderer = Arc::new(ComposedRenderer::new());
        let renderer = Statement::Else(ElseStatement::new(Some(Box::new(value))));
        let mut statement_info = StatementInfo::new(
            StatementInfoType::ElseIfStatement,
            Token::Else,
            composed_renderer,
        );
        statement_info.renderer = Some(renderer);
        statementinfo_list.push(statement_info);
        Ok(())
    }

    fn parse_else(statementinfo_list: &mut StatementInfoList) -> Result<()> {
        let composed_renderer = Arc::new(ComposedRenderer::new());
        let renderer = Statement::Else(ElseStatement::new(None));
        let mut statement_info = StatementInfo::new(
            StatementInfoType::ElseIfStatement,
            Token::Else,
            composed_renderer,
        );
        statement_info.renderer = Some(renderer);
        statementinfo_list.push(statement_info);
        Ok(())
    }
    fn parse_endif<'a>(statementinfo_list: &mut StatementInfoList<'a>) -> Result<()> {
        if statementinfo_list.len() <= 1 {
            return Err(Error::from(ErrorKind::UnexpectedStatement(
                SourceLocation::new(110, 220),
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
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let mut vars = vec![];
        loop {
            if let Some(Token::Identifier(identifier)) = lexer.next() {
                vars.push(identifier.to_string());
            } else {
                return Err(Error::from(ErrorKind::ExpectedIdentifier(
                    SourceLocation::new(1, 2),
                )));
            }
            if let Some(Token::Comma) = lexer.peek() {
                lexer.next();
            } else {
                break;
            }
        }
        if let Some(Token::In) = lexer.next() {
            let expression = ExpressionParser::full_expresion_parser(lexer)?;
            if lexer.next().is_some() {
                Err(Error::from(ErrorKind::ExpectedToken(SourceLocation::new(
                    1, 2,
                ))))
            } else {
                let composed_renderer = Arc::new(ComposedRenderer::new());
                let renderer = Statement::For(ForStatement::new(vars, Box::new(expression)));
                let mut statement_info = StatementInfo::new(
                    StatementInfoType::ForStatement,
                    Token::For,
                    composed_renderer,
                );
                statement_info.renderer = Some(renderer);
                statementinfo_list.push(statement_info);
                Ok(())
            }
        } else {
            Err(Error::from(ErrorKind::ExpectedToken(SourceLocation::new(
                1, 2,
            ))))
        }
    }
    fn parse_endfor<'a>(
        _lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        if statementinfo_list.len() <= 1 {
            return Err(Error::from(ErrorKind::UnexpectedStatement(
                SourceLocation::new(1, 2),
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
            Err(Error::from(ErrorKind::UnexpectedStatement(
                SourceLocation::new(1, 2),
            )))
        }
    }
}
