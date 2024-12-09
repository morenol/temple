use crate::context::Context;
use crate::error::{Error, Result};
use crate::filters::FilterExpression;
use crate::renderer::Render;
use crate::value::visitors;
use crate::value::{Value, ValuesList, ValuesMap};
use std::collections::HashMap;
use std::io::Write;

pub trait Evaluate {
    fn evaluate(&self, values: Context<'_>) -> Result<Value>;
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
pub struct SubscriptExpression<'a> {
    expression: Box<Expression<'a>>,
    subscript_expression: Vec<Box<dyn Evaluate + 'a>>,
}
#[derive(Default)]
pub struct TupleExpression<'a> {
    pub expressions: Vec<Box<dyn Evaluate + 'a>>,
}
impl<'a> TupleExpression<'a> {
    pub fn push(&mut self, expression: Box<dyn Evaluate + 'a>) {
        self.expressions.push(expression)
    }
}
impl Evaluate for TupleExpression<'_> {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        let tuple: ValuesList = self
            .expressions
            .iter()
            .map(|expr| expr.evaluate(values.clone()).unwrap())
            .collect();
        Ok(Value::ValuesList(tuple))
    }
}

pub struct ValueRefExpression {
    identifier: String,
}
#[derive(Default)]
pub struct DictionaryExpression<'a> {
    elems: std::collections::HashMap<String, Box<dyn Evaluate + 'a>>,
}
impl<'a> DictionaryExpression<'a> {
    pub fn push(&mut self, key: String, value: Box<dyn Evaluate + 'a>) {
        self.elems.insert(key, value);
    }
}
impl Evaluate for DictionaryExpression<'_> {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        let mut dict = ValuesMap::new();
        for (key, expression) in self.elems.iter() {
            dict.insert(key.to_string(), expression.evaluate(values.clone())?);
        }
        Ok(Value::ValuesMap(dict))
    }
}

pub struct FilteredExpression<'a> {
    expression: Box<dyn Evaluate + 'a>,
    filter: FilterExpression<'a>,
}
impl<'a> FilteredExpression<'a> {
    pub fn new(expression: Box<dyn Evaluate + 'a>, filter: FilterExpression<'a>) -> Self {
        Self { expression, filter }
    }
}

impl Evaluate for FilteredExpression<'_> {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        let result = self.expression.evaluate(values.clone());
        let base_value = match result {
            Ok(value) => value,
            Err(_err) => Value::Empty,
        };
        self.filter.filter(base_value, values)
    }
}

pub enum Expression<'a> {
    Constant(Value),
    Binary(BinaryOperation, Box<Expression<'a>>, Box<Expression<'a>>),
    Unary(UnaryOperation, Box<Expression<'a>>),
    Subscript(SubscriptExpression<'a>),
    ValueRef(ValueRefExpression),
    Filtered(FilteredExpression<'a>),
    Tuple(TupleExpression<'a>),
    Dict(DictionaryExpression<'a>),
}
impl ValueRefExpression {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }
}
impl Evaluate for ValueRefExpression {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        values.find(&self.identifier)
    }
}

impl<'a> SubscriptExpression<'a> {
    pub fn new(expression: Box<Expression<'a>>) -> Self {
        let subscript_expression = vec![];
        Self {
            expression,
            subscript_expression,
        }
    }
    pub fn add_index(&mut self, subscript: Box<dyn Evaluate + 'a>) {
        self.subscript_expression.push(subscript);
    }
}
impl Evaluate for SubscriptExpression<'_> {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        let mut cur = self.expression.evaluate(values.clone())?;
        for idx in &self.subscript_expression {
            let subscript = idx.evaluate(values.clone())?;
            cur = visitors::Subscription::apply(cur, subscript);
        }

        Ok(cur)
    }
}
impl Evaluate for Expression<'_> {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        let result = match &self {
            Expression::Constant(value) => value.clone(),
            Expression::Binary(op, left, right) => {
                let left_val = left.evaluate(values.clone())?;
                let right_val = right.evaluate(values)?;
                visitors::BinaryMathOperation::apply(op, left_val, right_val)
            }
            Expression::Unary(op, expr) => {
                let expression = expr.evaluate(values)?;
                match op {
                    UnaryOperation::Plus => expression,
                    UnaryOperation::Minus => -expression,
                    UnaryOperation::LogicalNot => !expression,
                }
            }
            Expression::Subscript(sub) => sub.evaluate(values)?,
            Expression::ValueRef(identifier) => identifier.evaluate(values)?,
            Expression::Tuple(tuple) => tuple.evaluate(values)?,
            Expression::Dict(dict) => dict.evaluate(values)?,
            Expression::Filtered(filter) => filter.evaluate(values)?,
        };
        Ok(result)
    }
}

#[derive(Default)]
pub struct FullExpressionEvaluator<'a> {
    expression: Option<Expression<'a>>,
}

impl Render for FullExpressionEvaluator<'_> {
    fn render(&self, out: &mut dyn Write, params: Context<'_>) -> Result<()> {
        let value = self.evaluate(params)?;
        if let Err(err) = out.write(value.to_string().as_bytes()) {
            Err(Error::Io(err))
        } else {
            Ok(())
        }
    }
}

impl<'a> FullExpressionEvaluator<'a> {
    pub fn set_expression(&mut self, expression: Expression<'a>) {
        self.expression = Some(expression)
    }
}

impl Evaluate for FullExpressionEvaluator<'_> {
    fn evaluate(&self, values: Context<'_>) -> Result<Value> {
        let result = match &self.expression {
            Some(expression) => expression.evaluate(values)?,
            None => Value::default(),
        };
        Ok(result)
    }
}

#[derive(Default)]
pub struct CallParams<'a> {
    pub kw_params: HashMap<String, FullExpressionEvaluator<'a>>,
    pub pos_params: Vec<FullExpressionEvaluator<'a>>,
}

impl CallParams<'_> {
    pub fn parse<'b>(
        &self,
        param_names: Vec<&'b str>,
        context: Context<'_>,
    ) -> Result<HashMap<&'b str, Value>> {
        let mut parameters = HashMap::default();
        let mut idx = 0;
        for name in param_names {
            match self.kw_params.get(name) {
                Some(expression) => {
                    let value = expression.evaluate(context.clone())?;
                    parameters.insert(name, value);
                }
                None => {
                    if let Some(expression) = self.pos_params.get(idx) {
                        idx += 1;
                        let value = expression.evaluate(context.clone())?;
                        parameters.insert(name, value);
                    }
                }
            }
        }
        Ok(parameters)
    }
}
