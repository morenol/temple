use super::{
    ElseStatement, IfStatement, Statement, StatementInfo, StatementInfoList, StatementInfoType,
};
use crate::error::{Error, ErrorKind, Result, SourceLocation};
use crate::expression_parser::ExpressionParser;
use crate::lexer::Token;
use crate::renderer::ComposedRenderer;
use logos::{Lexer, Logos};
use std::iter::Peekable;
pub struct StatementParser;
use std::rc::Rc;

impl StatementParser {
    pub fn parse<'a, 'b>(
        text: &'a str,
        mut statementinfo_list: &mut StatementInfoList<'a>,
    ) -> Result<()> {
        let lexer: Lexer<Token<'a>> = Token::lexer(text);
        let mut lexer: Peekable<Lexer<Token<'a>>> = lexer.peekable();
        let tok = lexer.next();

        let result = match tok {
            Some(Token::If) => StatementParser::parse_if(&mut lexer, &mut statementinfo_list),
            Some(Token::Else) => StatementParser::parse_else(&mut statementinfo_list),
            Some(Token::EndIf) => StatementParser::parse_endif(&mut statementinfo_list),
            Some(Token::ElIf) => StatementParser::parse_elif(&mut lexer, &mut statementinfo_list),
            _ => todo!(),
        };

        result
    }
    fn parse_if<'a>(
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        statementinfo_list: &mut StatementInfoList,
    ) -> Result<()> {
        let value = ExpressionParser::full_expresion_parser(lexer)?;
        let composed_renderer = Rc::new(ComposedRenderer::new());
        let renderer = Statement::If(IfStatement::new(Box::new(value)));
        let mut statement_info =
            StatementInfo::new(StatementInfoType::IfStatement, Token::If, composed_renderer);
        statement_info.renderer = Some(renderer);

        statementinfo_list.push(statement_info);
        Ok(())
    }
    fn parse_elif<'a>(
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        statementinfo_list: &mut StatementInfoList,
    ) -> Result<()> {
        let value = ExpressionParser::full_expresion_parser(lexer)?;
        let composed_renderer = Rc::new(ComposedRenderer::new());
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

    fn parse_else<'a>(statementinfo_list: &mut StatementInfoList) -> Result<()> {
        let composed_renderer = Rc::new(ComposedRenderer::new());
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
}
