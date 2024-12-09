use crate::error::{Error, ParseError, ParseErrorKind, Result};
use crate::expression_evaluator::{
    BinaryOperation, CallParams, DictionaryExpression, Expression, FilteredExpression,
    FullExpressionEvaluator, SubscriptExpression, TupleExpression, UnaryOperation,
    ValueRefExpression,
};
use crate::filters::FilterExpression;
use crate::lexer::{PeekableLexer, Token};
use crate::source::SourceLocationInfo;
use crate::value::Value;

use crate::renderer::ExpressionRenderer;
use logos::{Lexer, Logos};

pub struct ExpressionParser {}

impl ExpressionParser {
    pub fn parse<'a>(text: &'a str) -> Result<ExpressionRenderer<'a>> {
        let lexer: Lexer<'_, Token<'a>> = Token::lexer(text);
        let mut lexer = PeekableLexer::new(lexer);

        let evaluator = ExpressionParser::full_expresion_parser(&mut lexer)?;

        if let Some(_tok) = lexer.next() {
            todo!()
        }
        Ok(ExpressionRenderer::new(evaluator))
    }

    pub fn full_expresion_parser<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<FullExpressionEvaluator<'a>> {
        let mut evaluator = FullExpressionEvaluator::default();

        let value = ExpressionParser::parse_logical_or(lexer);
        match value {
            Ok(expression) => evaluator.set_expression(expression),
            Err(err) => return Err(err),
        }

        Ok(evaluator)
    }

    fn parse_logical_or<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_logical_and(lexer)?;
        if let Some(Ok(Token::LogicalOr)) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_logical_or(lexer)?;
            return Ok(Expression::Binary(
                BinaryOperation::LogicalOr,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_logical_and<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_logical_compare(lexer)?;
        if let Some(Ok(Token::LogicalAnd)) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_logical_and(lexer)?;
            return Ok(Expression::Binary(
                BinaryOperation::LogicalAnd,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_logical_compare<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_string_concat(lexer)?;

        let binary_op = match lexer.peek() {
            Some(Ok(Token::Equal)) => BinaryOperation::LogicalEq,
            Some(Ok(Token::NotEqual)) => BinaryOperation::LogicalNe,
            Some(Ok(Token::Lt)) => BinaryOperation::LogicalLt,
            Some(Ok(Token::Gt)) => BinaryOperation::LogicalGt,
            Some(Ok(Token::LessEqual)) => BinaryOperation::LogicalLe,
            Some(Ok(Token::GreaterEqual)) => BinaryOperation::LogicalGe,
            _ => return Ok(left),
        };

        lexer.next();
        let right = ExpressionParser::parse_string_concat(lexer)?;

        Ok(Expression::Binary(
            binary_op,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn parse_string_concat<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_math_pow(lexer)?;
        if let Some(Ok(Token::Tilde)) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_logical_and(lexer)?;
            return Ok(Expression::Binary(
                BinaryOperation::StringConcat,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_math_pow<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_math_plus_minus(lexer)?;
        if let Some(Ok(Token::MulMul)) = lexer.peek() {
            lexer.next();
            let right = ExpressionParser::parse_math_pow(lexer)?;
            return Ok(Expression::Binary(
                BinaryOperation::Pow,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_math_plus_minus<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_math_mul_div(lexer)?;
        let binary_op = match lexer.peek() {
            Some(Ok(Token::Plus)) => BinaryOperation::Plus,
            Some(Ok(Token::Minus)) => BinaryOperation::Minus,
            _ => return Ok(left),
        };
        lexer.next();
        let right = ExpressionParser::parse_math_plus_minus(lexer)?;
        Ok(Expression::Binary(
            binary_op,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn parse_math_mul_div<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let left = ExpressionParser::parse_unary_plus_min(lexer)?;
        let binary_op = match lexer.peek() {
            Some(Ok(Token::Mul)) => BinaryOperation::Mul,
            Some(Ok(Token::Div)) => BinaryOperation::Div,
            Some(Ok(Token::DivDiv)) => BinaryOperation::DivInteger,
            Some(Ok(Token::Percent)) => BinaryOperation::Modulo,
            _ => return Ok(left),
        };
        lexer.next();
        let right = ExpressionParser::parse_math_mul_div(lexer)?;

        Ok(Expression::Binary(
            binary_op,
            Box::new(left),
            Box::new(right),
        ))
    }

    fn parse_unary_plus_min<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<Expression<'a>> {
        let unary_op = match lexer.peek() {
            Some(Ok(Token::Plus)) => Some(UnaryOperation::Plus),
            Some(Ok(Token::Minus)) => Some(UnaryOperation::Minus),
            Some(Ok(Token::LogicalNot)) => Some(UnaryOperation::LogicalNot),
            _ => None,
        };
        if unary_op.is_some() {
            lexer.next();
        }

        let sub_expr = ExpressionParser::parse_value_expression(lexer)?;

        let result = match unary_op {
            Some(op) => Expression::Unary(op, Box::new(sub_expr)),
            None => sub_expr,
        };

        if let Some(Ok(Token::Pipe)) = lexer.peek() {
            lexer.next();
            let filter_expression = ExpressionParser::parse_filter_expression(lexer)?;
            Ok(Expression::Filtered(FilteredExpression::new(
                Box::new(result),
                filter_expression,
            )))
        } else {
            Ok(result)
        }
    }
    fn parse_filter_expression<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<FilterExpression<'a>> {
        let mut result: Option<FilterExpression<'_>> = None;
        loop {
            match lexer.next() {
                Some(token) => {
                    if let Ok(Token::Identifier(identifier)) = token {
                        let params = if let Some(Ok(Token::LBracket)) = lexer.peek() {
                            lexer.next();
                            ExpressionParser::parse_call_params(lexer)?
                        } else {
                            None
                        };

                        let mut filter = FilterExpression::new(&identifier, params)?;
                        if let Some(expression) = result.take() {
                            filter.set_parent_filter(expression);
                        }
                        result = Some(filter);
                    } else {
                        let range = lexer.span();
                        return Err(Error::from(ParseErrorKind::ExpectedIdentifier(
                            SourceLocationInfo::new(range.start, range.end),
                        )));
                    }
                    if let Some(Ok(Token::Pipe)) = lexer.peek() {
                        lexer.next();
                    } else {
                        break;
                    }
                }
                None => {
                    let range = lexer.span();
                    return Err(Error::from(ParseErrorKind::ExpectedIdentifier(
                        SourceLocationInfo::new(range.start, range.end),
                    )));
                }
            }
        }
        Ok(result.unwrap())
    }
    fn parse_call_params<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<Option<CallParams<'a>>> {
        let mut params = CallParams::default();
        if let Some(Ok(Token::RBracket)) = lexer.peek() {
            lexer.next();
            return Ok(None);
        }

        loop {
            let mut params_name: Option<String> = None;
            if let Some(Ok(Token::Identifier(keyword))) = lexer.peek() {
                params_name = Some(keyword.to_string());
                lexer.next();
                if let Some(Ok(Token::Assign)) = lexer.peek() {
                    lexer.next();
                }
            }
            let value = ExpressionParser::full_expresion_parser(lexer)?;
            if let Some(keyword) = params_name {
                params.kw_params.insert(keyword, value);
            } else {
                params.pos_params.push(value);
            }
            if let Some(Ok(Token::Comma)) = lexer.peek() {
                lexer.next();
            } else {
                break;
            }
        }
        if let Some(Ok(Token::RBracket)) = lexer.next() {
            Ok(Some(params))
        } else {
            let range = lexer.span();
            Err(Error::from(ParseError::new(
                ParseErrorKind::ExpectedBracket("}"),
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )))
        }
    }
    fn parse_value_expression<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<Expression<'a>> {
        let token = lexer.next();

        let value = if let Some(tok) = token {
            match tok {
                Ok(Token::IntegerNum(num)) => Expression::Constant(Value::from(num)),
                Ok(Token::True) => Expression::Constant(Value::from(true)),
                Ok(Token::False) => Expression::Constant(Value::from(false)),
                Ok(Token::FloatNum(num)) => Expression::Constant(Value::from(num)),
                Ok(Token::String(string)) => Expression::Constant(Value::from(string.to_string())),
                Ok(Token::LBracket) => ExpressionParser::parse_braced_expression_or_tuple(lexer)?,
                Ok(Token::Identifier(identifier)) => {
                    Expression::ValueRef(ValueRefExpression::new(identifier.to_string()))
                }
                Ok(Token::LSqBracket) => ExpressionParser::parse_tuple(lexer)?,
                Ok(Token::LCrlBracket) => ExpressionParser::parse_dict(lexer)?,

                _ => {
                    let range = lexer.span();
                    return Err(Error::from(ParseError::new(
                        ParseErrorKind::ExpectedExpression,
                        Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                    )));
                }
            }
        } else {
            let range = lexer.span();
            return Err(Error::from(ParseError::new(
                ParseErrorKind::ExpectedExpression,
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )));
        };

        let token = lexer.peek();

        let value = match token {
            Some(Ok(Token::LSqBracket)) | Some(Ok(Token::Point)) => {
                ExpressionParser::parse_subscript(lexer, value)?
            }
            Some(Ok(Token::LBracket)) => todo!(),
            _ => value,
        };

        Ok(value)
    }

    fn parse_braced_expression_or_tuple<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
    ) -> Result<Expression<'a>> {
        let mut is_tuple: bool = false;
        let mut exprs = vec![];
        loop {
            if let Some(Ok(Token::RBracket)) = lexer.peek() {
                lexer.next();
                break;
            }
            let expr = ExpressionParser::parse_logical_or(lexer);
            match expr {
                Ok(expr) => exprs.push(expr),
                Err(err) => {
                    if !exprs.is_empty() {
                        return Err(Error::from(ParseError::new(
                            ParseErrorKind::ExpectedBracket(")"),
                            Some(SourceLocationInfo::new_at_the_end()),
                        )));
                    } else {
                        return Err(err);
                    }
                }
            }
            if let Some(Ok(Token::Comma)) = lexer.peek() {
                lexer.next();
                is_tuple = true;
            }
        }
        if is_tuple {
            let mut tuple = TupleExpression::default();
            for expr in exprs {
                tuple.push(Box::new(expr));
            }
            Ok(Expression::Tuple(tuple))
        } else {
            Ok(exprs.remove(0))
        }
    }
    fn parse_subscript<'a>(
        lexer: &mut PeekableLexer<'a, Token<'a>>,
        expression: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut subscript = SubscriptExpression::new(Box::new(expression));
        while let Some(token) = lexer.peek() {
            match token {
                Ok(Token::LSqBracket) => {
                    lexer.next();
                    let expr = ExpressionParser::full_expresion_parser(lexer)?;
                    if let Some(Ok(Token::RSqBracket)) = lexer.next() {
                        subscript.add_index(Box::new(expr));
                    } else {
                        let range = lexer.span();

                        return Err(Error::from(ParseError::new(
                            ParseErrorKind::ExpectedBracket("]"),
                            Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                        )));
                    }
                }
                Ok(Token::Point) => {
                    lexer.next();
                    let token = lexer.next();
                    if let Some(Ok(Token::Identifier(identifier))) = token {
                        subscript.add_index(Box::new(Expression::Constant(Value::String(
                            identifier.to_string(),
                        ))));
                    } else {
                        let range = lexer.span();
                        return Err(Error::from(ParseErrorKind::ExpectedIdentifier(
                            SourceLocationInfo::new_with_range(range.start, range.end),
                        )));
                    }
                }
                _ => break,
            };
        }

        Ok(Expression::Subscript(subscript))
    }
    fn parse_tuple<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let mut tuple = TupleExpression::default();
        if let Some(Ok(Token::RSqBracket)) = lexer.peek() {
            lexer.next();
            return Ok(Expression::Tuple(tuple));
        }

        loop {
            let expr = ExpressionParser::full_expresion_parser(lexer)?;
            tuple.push(Box::new(expr));
            if let Some(Ok(Token::Comma)) = lexer.peek() {
                lexer.next();
            } else {
                break;
            }
        }
        if let Some(Ok(Token::RSqBracket)) = lexer.peek() {
            lexer.next();
            Ok(Expression::Tuple(tuple))
        } else {
            let range = lexer.span();

            Err(Error::from(ParseError::new(
                ParseErrorKind::ExpectedBracket("]"),
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )))
        }
    }
    fn parse_dict<'a>(lexer: &mut PeekableLexer<'a, Token<'a>>) -> Result<Expression<'a>> {
        let mut dict = DictionaryExpression::default();
        if let Some(Ok(Token::RCrlBracket)) = lexer.peek() {
            lexer.next();
            return Ok(Expression::Dict(dict));
        }
        loop {
            let key = lexer.next();
            if let Some(Ok(Token::String(key_string))) = key {
                if let Some(Ok(Token::Colon)) = lexer.next() {
                    let expr = ExpressionParser::full_expresion_parser(lexer)?;
                    dict.push(key_string.to_string(), Box::new(expr));
                    if let Some(Ok(Token::Comma)) = lexer.peek() {
                        lexer.next();
                        continue;
                    } else {
                        break;
                    }
                } else {
                    let range = lexer.span();

                    return Err(Error::from(ParseError::new(
                        ParseErrorKind::ExpectedToken(":"),
                        Some(SourceLocationInfo::new_with_range(range.start, range.end)),
                    )));
                }
            } else {
                let range = lexer.span();
                return Err(Error::from(ParseErrorKind::ExpectedStringLiteral(
                    SourceLocationInfo::new_with_range(range.start, range.end),
                )));
            }
        }
        if let Some(Ok(Token::RCrlBracket)) = lexer.next() {
            Ok(Expression::Dict(dict))
        } else {
            let range = lexer.span();

            Err(Error::from(ParseError::new(
                ParseErrorKind::ExpectedBracket("}"),
                Some(SourceLocationInfo::new_with_range(range.start, range.end)),
            )))
        }
    }
}
