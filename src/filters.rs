use crate::error::Result;
use crate::value::{Value, ValuesMap};
pub enum Filter {
    Abs,
    First,
    Int,
    Last,
    Length,
    Lower,
    Upper,
}
impl Filter {
    pub fn new(name: &str) -> Result<Self> {
        match name {
            "abs" => Ok(Filter::Abs),
            "first" => Ok(Filter::First),
            "int" => Ok(Filter::Int),
            "last" => Ok(Filter::Last),
            "length" => Ok(Filter::Length),
            "lower" => Ok(Filter::Lower),
            "upper" => Ok(Filter::Upper),
            _ => todo!(),
        }
    }
    pub fn filter(&self, base_value: Value, _context: &ValuesMap) -> Result<Value> {
        match &self {
            Filter::Abs => base_value.abs(),
            Filter::First => base_value.first(),
            Filter::Int => Ok(Value::Integer(base_value.int()?)),
            Filter::Last => base_value.last(),
            Filter::Length => Ok(Value::Integer(base_value.len()? as i64)),
            Filter::Lower => base_value.lower(),
            Filter::Upper => base_value.upper(),
        }
    }
}
