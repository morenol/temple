use super::{Value, ValuesList, ValuesMap};

impl From<isize> for Value {
    fn from(f: isize) -> Self {
        From::from(f as i64)
    }
}
impl From<usize> for Value {
    fn from(f: usize) -> Self {
        From::from(f as i64)
    }
}

impl From<i8> for Value {
    fn from(f: i8) -> Self {
        From::from(f as i64)
    }
}
impl From<u8> for Value {
    fn from(f: u8) -> Self {
        From::from(f as i64)
    }
}

impl From<i16> for Value {
    fn from(f: i16) -> Self {
        From::from(f as i64)
    }
}
impl From<u16> for Value {
    fn from(f: u16) -> Self {
        From::from(f as i64)
    }
}

impl From<u32> for Value {
    fn from(f: u32) -> Self {
        From::from(f as i64)
    }
}
impl From<i32> for Value {
    fn from(f: i32) -> Self {
        From::from(f as i64)
    }
}

impl From<u64> for Value {
    fn from(f: u64) -> Self {
        From::from(f as i64)
    }
}

impl From<i64> for Value {
    fn from(f: i64) -> Self {
        Value::Integer(f)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        From::from(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Double(f)
    }
}

impl From<bool> for Value {
    fn from(f: bool) -> Self {
        Value::Boolean(f)
    }
}

impl From<String> for Value {
    fn from(f: String) -> Self {
        Value::String(f)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(f: &str) -> Self {
        Value::String(f.to_string())
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Value::Empty
    }
}

impl From<ValuesMap> for Value {
    fn from(f: ValuesMap) -> Self {
        Value::ValuesMap(f)
    }
}

impl From<ValuesList> for Value {
    fn from(f: ValuesList) -> Self {
        Value::ValuesList(f)
    }
}

impl From<Value> for ValuesList {
    fn from(f: Value) -> Self {
        match f {
            Value::ValuesList(value_list) => value_list,
            Value::ValuesMap(value_map) => value_map
                .keys()
                .map(|key| Value::String(key.to_string()))
                .collect(),
            Value::String(value_string) => value_string
                .chars()
                .map(|ch| Value::String(ch.to_string()))
                .collect(),
            _ => vec![],
        }
    }
}
