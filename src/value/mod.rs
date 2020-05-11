#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Empty,
    Boolean(bool),
    String(String),
    Integer(i64),
    Double(f64),
    ValuesList(ValuesList),
    ValuesMap(ValuesMap),
}

pub type ValuesMap = std::collections::HashMap<String, Value>;

pub type ValuesList = Vec<Value>;

impl Default for Value {
    fn default() -> Value {
        Value::Empty
    }
}

mod from;
