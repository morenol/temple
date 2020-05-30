use crate::renderer::Render;
use crate::value::visitors;
use crate::value::Value;
use std::io::Write;

pub trait Evaluate {
    fn evaluate(&self) -> Value;
}
#[derive(Debug)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Mul,
    Div,
    DivInteger,
    Modulo,
    Pow,
    LogicalOr,
    LogicalAnd,
    LogicalEq,
    LogicalNe,
    LogicalLt,
    LogicalGt,
    LogicalGe,
    LogicalLe,
    StringConcat,
}

#[derive(Debug)]
pub enum UnaryOperation {
    Plus,
    Minus,
    LogicalNot,
}

#[derive(Debug)]
pub enum Expression {
    Constant(Value),
    BinaryExpression(BinaryOperation, Box<Expression>, Box<Expression>),
    UnaryExpression(UnaryOperation, Box<Expression>),
}

impl Evaluate for Expression {
    fn evaluate(&self) -> Value {
        match &self {
            Expression::Constant(value) => value.clone(),
            Expression::BinaryExpression(op, left, right) => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                visitors::BinaryMathOperation::apply(op, left_val, right_val)
            }
            Expression::UnaryExpression(op, expr) => {
                let expression = expr.evaluate();
                match op {
                    UnaryOperation::Plus => expression,
                    UnaryOperation::Minus => -expression,
                    UnaryOperation::LogicalNot => !expression,
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct FullExpressionEvaluator {
    expression: Option<Expression>,
}

impl Render for FullExpressionEvaluator {
    fn render(&self, out: &mut dyn Write) {
        let value = self.evaluate();
        out.write(value.to_string().as_bytes());
    }
}

impl FullExpressionEvaluator {
    pub fn new() -> Self {
        Self { expression: None }
    }

    pub fn set_expression(&mut self, expression: Expression) {
        self.expression = Some(expression)
    }
}

impl Evaluate for FullExpressionEvaluator {
    fn evaluate(&self) -> Value {
        match &self.expression {
            Some(expression) => expression.evaluate(),
            None => Value::default(),
        }
    }
}
