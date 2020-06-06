use crate::error::Result;
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
impl Value {
    pub fn abs(self) -> Result<Self> {
        match self {
            Value::Integer(number) => Ok(Value::Integer(number.abs())),
            Value::Double(number) => Ok(Value::Double(number.abs())),
            _ => todo!(),
        }
    }
    pub fn first(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(s.chars().next().unwrap().to_string())),
            Value::ValuesMap(values_map) => todo!(),
            Value::ValuesList(values_list) => Ok(values_list.first().unwrap().clone()),
            _ => todo!(),
        }
    }
    pub fn int(self) -> Result<Self> {
        match self {
            Value::Integer(number) => Ok(Value::Integer(number)),
            Value::Double(number) => Ok(Value::Integer(number as i64)),
            _ => todo!(),
        }
    }
    pub fn len(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::Integer(s.len() as i64)),
            Value::ValuesMap(values_map) => Ok(Value::Integer(values_map.len() as i64)),
            Value::ValuesList(values_list) => Ok(Value::Integer(values_list.len() as i64)),
            _ => todo!(),
        }
    }
    pub fn last(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(s.chars().last().unwrap().to_string())),
            Value::ValuesMap(values_map) => todo!(),
            Value::ValuesList(values_list) => Ok(values_list.last().unwrap().clone()),
            _ => todo!(),
        }
    }
    pub fn lower(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            _ => todo!(),
        }
    }
    pub fn upper(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => todo!(),
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
