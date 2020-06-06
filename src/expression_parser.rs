use crate::error::{Error, ErrorKind, Result, SourceLocation};
use crate::expression_evaluator::{
    BinaryOperation, DictionaryExpression, Expression, FilterExpression, FullExpressionEvaluator,
    SubscriptExpression, TupleExpression, UnaryOperation, ValueRefExpression,
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

        Ok(Expression::BinaryExpression(
            binary_op,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn parse_unary_plus_min<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
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

        let mut result = match unary_op {
            Some(op) => Expression::UnaryExpression(op, Box::new(sub_expr)),
            None => sub_expr,
        };

        if let Some(Token::Pipe) = lexer.peek() {
            lexer.next();
            let filter_expression = ExpressionParser::parse_filter_expression(&mut lexer)?;
            if let Expression::Filter(mut filter) = filter_expression {
                filter.set_parent_filter(result);
                result = Expression::Filter(filter);
            } else {
                todo!()
            }
        }
        Ok(result)
    }
    fn parse_filter_expression<'a>(
        lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
    ) -> Result<Expression<'a>> {
        let mut result: Option<Expression<'a>> = None;
        loop {
            match lexer.next() {
                Some(token) => {
                    if let Token::Identifier(identifier) = token {
                        let mut filter = FilterExpression::new(&identifier)?;
                        if let Some(expression) = result.take() {
                            filter.set_parent_filter(expression);
                        }
                        result = Some(Expression::Filter(filter));
                    } else {
                        return Err(Error::from(ErrorKind::ExpectedIdentifier(
                            SourceLocation::new(1, 2),
                        )));
                    }
                    if let Some(Token::Pipe) = lexer.peek() {
                        lexer.next();
                    } else {
                        break;
                    }
                }
                None => {
                    return Err(Error::from(ErrorKind::ExpectedIdentifier(
                        SourceLocation::new(1, 2),
                    )));
                }
            }
        }
        Ok(result.unwrap())
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
                Token::Identifier(identifier) => {
                    Expression::ValueRef(ValueRefExpression::new(identifier.to_string()))
                }
                Token::LSqBracket => ExpressionParser::parse_tuple(&mut lexer)?,
                Token::LCrlBracket => ExpressionParser::parse_dict(&mut lexer)?,

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

        Ok(value)
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
                    if !exprs.is_empty() {
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
            let mut tuple = TupleExpression::new();
            for expr in exprs {
                tuple.push(Box::new(expr));
            }
            Ok(Expression::Tuple(tuple))
        } else {
            Ok(exprs.remove(0))
        }
    }
    fn parse_subscript<'a>(
        mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>,
        expression: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut subscript = SubscriptExpression::new(Box::new(expression));
        while let Some(token) = lexer.peek() {
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
        }

        Ok(Expression::SubscriptExpression(subscript))
    }
    fn parse_tuple<'a>(mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>) -> Result<Expression<'a>> {
        let mut tuple = TupleExpression::new();
        if let Some(Token::RSqBracket) = lexer.peek() {
            lexer.next();
            return Ok(Expression::Tuple(tuple));
        }

        loop {
            let expr = ExpressionParser::full_expresion_parser(&mut lexer)?;
            tuple.push(Box::new(expr));
            if let Some(Token::Comma) = lexer.peek() {
                lexer.next();
            } else {
                break;
            }
        }
        if let Some(Token::RSqBracket) = lexer.peek() {
            lexer.next();
            Ok(Expression::Tuple(tuple))
        } else {
            Err(Error::from(ErrorKind::ExpectedSquareBracket(
                SourceLocation::new(1, 2),
            )))
        }
    }
    fn parse_dict<'a>(mut lexer: &mut Peekable<Lexer<'a, Token<'a>>>) -> Result<Expression<'a>> {
        let mut dict = DictionaryExpression::new();
        if let Some(Token::RCrlBracket) = lexer.peek() {
            lexer.next();
            return Ok(Expression::Dict(dict));
        }
        loop {
            let key = lexer.next();
            if let Some(Token::String(key_string)) = key {
                if let Some(Token::Colon) = lexer.next() {
                    let expr = ExpressionParser::full_expresion_parser(&mut lexer)?;
                    dict.push(key_string.to_string(), Box::new(expr));
                    if let Some(Token::Comma) = lexer.peek() {
                        lexer.next();
                        continue;
                    } else {
                        break;
                    }
                } else {
                    return Err(Error::from(ErrorKind::ExpectedToken(SourceLocation::new(
                        1, 2,
                    ))));
                }
            } else {
                return Err(Error::from(ErrorKind::ExpectedStringLiteral(
                    SourceLocation::new(1, 2),
                )));
            }
        }
        if let Some(Token::RCrlBracket) = lexer.next() {
            Ok(Expression::Dict(dict))
        } else {
            Err(Error::from(ErrorKind::ExpectedCurlyBracket(
                SourceLocation::new(1, 2),
            )))
        }
    }
}
