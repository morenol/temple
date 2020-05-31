use crate::error::{Error, ErrorKind, Result, SourceLocation};
use crate::expression_evaluator::{
    BinaryOperation, Expression, FullExpressionEvaluator, SubscriptExpression, UnaryOperation,
};
use crate::lexer::Token;
use crate::value::Value;

use crate::renderer::ExpressionRenderer;
use logos::{Lexer, Logos};
use std::iter::Peekable;

pub struct ExpressionParser {}

impl ExpressionParser {
    pub fn parse<'a>(text: &'a str) -> Result<ExpressionRenderer> {
        let lexer: Lexer<Token<'a>> = Token::lexer(text);
        let mut lexer: Peekable<Lexer<Token<'a>>> = lexer.peekable();

        let evaluator = ExpressionParser::full_expresion_parser(&mut lexer)?;

        if let Some(_tok) = lexer.next() {
            todo!()
        }
        Ok(ExpressionRenderer::new(evaluator))
    }

    pub fn full_expresion_parser<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<FullExpressionEvaluator<'a>> {
        let mut evaluator = FullExpressionEvaluator::new();

        let value = ExpressionParser::parse_logical_or(&mut lexer);
        match value {
            Ok(expression) => evaluator.set_expression(expression),
            Err(err) => return Err(err),
        }

        Ok(evaluator)
    }

    fn parse_logical_or<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_logical_and(&mut lexer)?;
        if let Some(Token::LogicalOr) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_logical_or(lexer)?;
            return Ok(Expression::BinaryExpression(
                BinaryOperation::LogicalOr,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_logical_and<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_logical_compare(&mut lexer)?;
        if let Some(Token::LogicalAnd) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_logical_and(lexer)?;
            return Ok(Expression::BinaryExpression(
                BinaryOperation::LogicalAnd,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_logical_compare<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_string_concat(&mut lexer)?;

        let binary_op = match lexer.peek() {
            Some(Token::Equal) => BinaryOperation::LogicalEq,
            Some(Token::NotEqual) => BinaryOperation::LogicalNe,
            Some(Token::Lt) => BinaryOperation::LogicalLt,
            Some(Token::Gt) => BinaryOperation::LogicalGt,
            Some(Token::LessEqual) => BinaryOperation::LogicalLe,
            Some(Token::GreaterEqual) => BinaryOperation::LogicalGe,
            _ => return Ok(left),
        };

        lexer.next();
        let right = ExpressionParser::parse_string_concat(&mut lexer)?;

        Ok(Expression::BinaryExpression(
            binary_op,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn parse_string_concat<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_math_pow(&mut lexer)?;
        if let Some(Token::Tilde) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_logical_and(&mut lexer)?;
            return Ok(Expression::BinaryExpression(
                BinaryOperation::StringConcat,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_math_pow<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_math_plus_minus(&mut lexer)?;
        if let Some(Token::MulMul) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_math_pow(&mut lexer)?;
            return Ok(Expression::BinaryExpression(
                BinaryOperation::Pow,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_math_plus_minus<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_math_mul_div(&mut lexer)?;
        let binary_op = match lexer.peek() {
            Some(Token::Plus) => BinaryOperation::Plus,
            Some(Token::Minus) => BinaryOperation::Minus,
            _ => return Ok(left),
        };
        lexer.next();
        let right = ExpressionParser::parse_math_plus_minus(&mut lexer)?;
        Ok(Expression::BinaryExpression(
            binary_op,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn parse_math_mul_div<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_unary_plus_min(&mut lexer)?;
        let binary_op = match lexer.peek() {
            Some(Token::Mul) => BinaryOperation::Mul,
            Some(Token::Div) => BinaryOperation::Div,
            Some(Token::DivDiv) => BinaryOperation::DivInteger,
            Some(Token::Percent) => BinaryOperation::Modulo,
            _ => return Ok(left),
        };
        lexer.next();
        let right = ExpressionParser::parse_math_mul_div(&mut lexer)?;

        return Ok(Expression::BinaryExpression(
            binary_op,
            Box::new(left),
            Box::new(right),
        ));
    }

    fn parse_unary_plus_min<'a>(
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let unary_op = match lexer.peek() {
            Some(Token::Plus) => Some(UnaryOperation::Plus),
            Some(Token::Minus) => Some(UnaryOperation::Minus),
            Some(Token::LogicalNot) => Some(UnaryOperation::LogicalNot),
            _ => None,
        };
        if unary_op.is_some() {
            lexer.next();
        }

        let sub_expr = ExpressionParser::parse_value_expression(lexer)?;

        let result = match unary_op {
            Some(op) => Expression::UnaryExpression(op, Box::new(sub_expr)),
            None => sub_expr,
        };

        if let Some(Token::Pipe) = lexer.peek() {
            todo!()
        }
        Ok(result)
    }

    fn parse_value_expression<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let token = lexer.next();

        let value = if let Some(tok) = token {
            match tok {
                Token::IntegerNum(num) => Expression::Constant(Value::from(num)),
                Token::True => Expression::Constant(Value::from(true)),
                Token::False => Expression::Constant(Value::from(false)),
                Token::FloatNum(num) => Expression::Constant(Value::from(num)),
                Token::String(string) => Expression::Constant(Value::from(string.to_string())),
                Token::LBracket => ExpressionParser::parse_braced_expression_or_tuple(&mut lexer)?,
                _ => {
                    return Err(Error::from(ErrorKind::ExpectedExpression(
                        SourceLocation::new(1, 2), // TODO: Use actual source locations
                    )));
                }
            }
        } else {
            return Err(Error::from(ErrorKind::ExpectedExpression(
                SourceLocation::new(1, 2), // TODO: Use actual source locations
            )));
        };

        let token = lexer.peek();

        let value = match token {
            Some(Token::LSqBracket) | Some(Token::Point) => {
                ExpressionParser::parse_subscript(&mut lexer, value)?
            }
            Some(Token::LBracket) => todo!(),
            _ => value,
        };

        return Ok(value);
    }

    fn parse_braced_expression_or_tuple<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let mut is_tuple: bool = false;
        let mut exprs = vec![];
        loop {
            if let Some(Token::RBracket) = lexer.peek() {
                lexer.next();
                break;
            }
            let expr = ExpressionParser::parse_logical_or(&mut lexer);
            match expr {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    if exprs.len() > 0 {
                        return Err(Error::from(ErrorKind::ExpectedRoundBracket(
                            SourceLocation::new(1, 2),
                        )));
                    } else {
                        return Err(err);
                    }
                }
            }
            if let Some(Token::Comma) = lexer.peek() {
                lexer.next();
                is_tuple = true;
            }
        }
        if is_tuple {
            todo!()
        } else {
            return Ok(exprs.remove(0));
        }
    }
    fn parse_subscript<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        expression: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut subscript = SubscriptExpression::new(Box::new(expression));
        loop {
            if let Some(token) = lexer.peek() {
                match token {
                    Token::LSqBracket => {
                        lexer.next();
                        let expr = ExpressionParser::full_expresion_parser(&mut lexer)?;
                        if let Some(Token::RSqBracket) = lexer.next() {
                            subscript.add_index(Box::new(expr));
                        } else {
                            return Err(Error::from(ErrorKind::ExpectedSquareBracket(
                                SourceLocation::new(1, 2),
                            )));
                        }
                    }
                    Token::Point => {
                        lexer.next();
                        let token = lexer.next();
                        if let Some(Token::Identifier(identifier)) = token {
                            subscript.add_index(Box::new(Expression::Constant(Value::String(
                                identifier.to_string(),
                            ))));
                        } else {
                            return Err(Error::from(ErrorKind::ExpectedIdentifier(
                                SourceLocation::new(1, 2),
                            )));
                        }
                    }
                    _ => break,
                };
            } else {
                break;
            }
        }
        Ok(Expression::SubscriptExpression(subscript))
    }
}
