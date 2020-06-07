use crate::error::Result;
use crate::value::{Value, ValuesMap};
pub enum Filter {
    Abs,
    Capitalize,
    Escape,
    First,
    Float,
    Int,
    Last,
    Length,
    Lower,
    Max,
    Min,
    String,
    Sum,
    Upper,
}
impl Filter {
    pub fn new(name: &str) -> Result<Self> {
        match name {
            "abs" => Ok(Filter::Abs),
            "capitalize" => Ok(Filter::Capitalize),
            "escape" => Ok(Filter::Escape),
            "first" => Ok(Filter::First),
            "float" => Ok(Filter::Float),
            "int" => Ok(Filter::Int),
            "last" => Ok(Filter::Last),
            "length" => Ok(Filter::Length),
            "lower" => Ok(Filter::Lower),
            "max" => Ok(Filter::Max),
            "min" => Ok(Filter::Min),
            "string" => Ok(Filter::String),
            "sum" => Ok(Filter::Sum),
            "upper" => Ok(Filter::Upper),
            _ => todo!(),
        }
    }
    pub fn filter(&self, base_value: Value, _context: &ValuesMap) -> Result<Value> {
        match &self {
            Filter::Abs => base_value.abs(),
            Filter::Capitalize => base_value.capitalize(),
            Filter::Escape => base_value.escape(),
            Filter::First => base_value.first(),
            Filter::Int => Ok(Value::Integer(base_value.int()?)),
            Filter::Float => Ok(Value::Double(base_value.float()?)),
            Filter::Last => base_value.last(),
            Filter::Length => Ok(Value::Integer(base_value.len()? as i64)),
            Filter::Lower => base_value.lower(),
            Filter::Max => base_value.max(),
            Filter::Min => base_value.min(),
            Filter::String => Ok(Value::String(base_value.to_string())),
            Filter::Sum => base_value.sum(),
            Filter::Upper => base_value.upper(),
        }
    }
}

pub struct FilterExpression {
    filter: Filter,
    parent: Option<Box<FilterExpression>>,
}

impl FilterExpression {
    pub fn new(identifier: &str) -> Result<Self> {
        let filter = Filter::new(identifier)?;
        Ok(Self {
            filter,
            parent: None,
        })
    }
    pub fn set_parent_filter(&mut self, parent: FilterExpression) {
        self.parent = Some(Box::new(parent));
    }

    pub fn filter(&self, base_value: Value, context: &ValuesMap) -> Result<Value> {
        if self.parent.is_some() {
            self.filter.filter(
                self.parent.as_ref().unwrap().filter(base_value, context)?,
                context,
            )
        } else {
            self.filter.filter(base_value, context)
        }
    }
}
