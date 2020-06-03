use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    Empty,
    Boolean(bool),
    String(String),
    Integer(i64),
    Double(f64),
    ValuesList(ValuesList),
    ValuesMap(ValuesMap),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Empty => write!(f, ""),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
            Value::Integer(integer) => write!(f, "{}", integer),
            Value::Double(float) => write!(f, "{}", float),
            Value::String(string) => write!(f, "{}", string),
            Value::ValuesList(tuple) => {
                write!(f, "[")?;
                for (idx, value) in tuple.into_iter().enumerate() {
                    if idx == 0 {
                        write!(f, "{}", value)?;
                    } else {
                        write!(f, " {}", value)?;
                    }
                    if idx < tuple.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
            Value::ValuesMap(dict) => {
                write!(f, "{{")?;
                for (idx, (key, value)) in dict.iter().enumerate() {
                    if idx == 0 {
                        write!(f, "\"{}\": {}", key, value)?;
                    } else {
                        write!(f, " \"{}\": {}", key, value)?;
                    }
                    if idx < dict.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

pub type ValuesMap = std::collections::HashMap<String, Value>;

pub type ValuesList = Vec<Value>;

impl Default for Value {
    fn default() -> Value {
        Value::Empty
    }
}

mod from;
mod ops;

pub mod visitors;
