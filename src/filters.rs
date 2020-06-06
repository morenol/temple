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
    pub fn filter(&self, base_value: Value, context: &ValuesMap) -> Result<Value> {
        match &self {
            Filter::Abs => base_value.abs(),
            Filter::First => base_value.first(),
            Filter::Int => base_value.int(),
            Filter::Last => base_value.last(),
            Filter::Length => base_value.len(),
            Filter::Lower => base_value.lower(),
            Filter::Upper => base_value.upper(),
        }
    }
}
