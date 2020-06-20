use super::Value;
use crate::context::Context;
use crate::error::{Error, ErrorKind, Result};
use crate::expression_evaluator::CallParams;
use crate::expression_evaluator::Evaluate;

use regex::Regex;

impl Value {
    pub fn abs(self) -> Result<Self> {
        match self {
            Value::Integer(number) => Ok(Value::Integer(number.abs())),
            Value::Double(number) => Ok(Value::Double(number.abs())),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn capitalize(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(format!(
                "{}{}",
                &s[0..1].to_string().to_uppercase(),
                &s[1..],
            ))),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn default_filter(self, params: &Option<CallParams>, context: Context) -> Result<Self> {
        match self {
            Value::Empty => match params {
                None => Ok(Value::String("".to_string())),
                Some(call_params) => {
                    if let Some(value) = call_params.kw_params.get("default_value") {
                        Ok(value.evaluate(context)?)
                    } else if let Some(value) = call_params.pos_params.first() {
                        Ok(value.evaluate(context)?)
                    } else {
                        Ok(Value::String("".to_string()))
                    }
                }
            },
            value => Ok(value),
        }
    }
    pub fn escape(self) -> Result<Self> {
        if let Value::String(s) = self {
            lazy_static! {
                static ref ESCAPED_REGEX: Regex = Regex::new("[<>&\"\']").unwrap();
            }
            let s_input = &s[..];
            if ESCAPED_REGEX.is_match(s_input) {
                let mut last_match = 0;
                let matches = ESCAPED_REGEX.find_iter(s_input);
                let mut output = String::with_capacity(s_input.len());
                for mat in matches {
                    output.push_str(&s_input[last_match..mat.start()]);
                    match &s_input[mat.range()] {
                        "<" => output.push_str("&lt;"),
                        ">" => output.push_str("&gt;"),
                        "&" => output.push_str("&amp;"),
                        "\"" => output.push_str("&#34;"),
                        "\'" => output.push_str("&#39;"),
                        _ => unreachable!(),
                    }
                    last_match = mat.end();
                }
                Ok(Value::String(output))
            } else {
                Ok(Value::String(s))
            }
        } else {
            Err(Error::from(ErrorKind::InvalidOperation))
        }
    }

    pub fn first(self) -> Result<Self> {
        if self.is_empty()? {
            Ok(Value::Empty)
        } else {
            match self {
                Value::String(s) => Ok(Value::String(s.chars().next().unwrap().to_string())),
                Value::ValuesList(values_list) => Ok(values_list.first().unwrap().clone()),
                Value::ValuesMap(values_map) => Ok(values_map.values().next().unwrap().clone()),
                _ => Err(Error::from(ErrorKind::InvalidOperation)),
            }
        }
    }
    pub fn float(self) -> Result<f64> {
        match self {
            Value::Integer(number) => Ok(number as f64),
            Value::Double(number) => Ok(number),
            Value::Boolean(true) => Ok(1_f64),
            Value::Boolean(false) => Ok(0_f64),

            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }

    pub fn int(self) -> Result<i64> {
        match self {
            Value::Integer(number) => Ok(number),
            Value::Double(number) => Ok(number as i64),
            Value::Boolean(true) => Ok(1_i64),
            Value::Boolean(false) => Ok(0_i64),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn is_empty(&self) -> Result<bool> {
        match self {
            Value::Empty => Ok(true),
            Value::String(s) => Ok(s.is_empty()),
            Value::ValuesMap(values_map) => Ok(values_map.is_empty()),
            Value::ValuesList(values_list) => Ok(values_list.is_empty()),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn max(self) -> Result<Self> {
        if self.is_empty()? {
            Ok(Value::Empty)
        } else {
            match self {
                Value::String(s) => Ok(Value::String(s.chars().max().unwrap().to_string())),
                Value::ValuesList(values_list) => Ok(values_list.iter().max().unwrap().clone()),
                Value::ValuesMap(values_map) => Ok(values_map.values().last().unwrap().clone()),
                _ => Err(Error::from(ErrorKind::InvalidOperation)),
            }
        }
    }

    pub fn min(self) -> Result<Self> {
        if self.is_empty()? {
            Ok(Value::Empty)
        } else {
            match self {
                Value::String(s) => Ok(Value::String(s.chars().min().unwrap().to_string())),
                Value::ValuesList(values_list) => Ok(values_list.iter().min().unwrap().clone()),
                Value::ValuesMap(values_map) => Ok(values_map.values().next().unwrap().clone()),
                _ => Err(Error::from(ErrorKind::InvalidOperation)),
            }
        }
    }

    pub fn last(self) -> Result<Self> {
        if self.is_empty()? {
            Ok(Value::Empty)
        } else {
            match self {
                Value::String(s) => Ok(Value::String(s.chars().last().unwrap().to_string())),
                Value::ValuesList(values_list) => Ok(values_list.last().unwrap().clone()),
                Value::ValuesMap(values_map) => Ok(values_map.values().last().unwrap().clone()),
                _ => Err(Error::from(ErrorKind::InvalidOperation)),
            }
        }
    }
    pub fn len(self) -> Result<usize> {
        match self {
            Value::String(s) => Ok(s.len()),
            Value::ValuesMap(values_map) => Ok(values_map.len()),
            Value::ValuesList(values_list) => Ok(values_list.len()),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn lower(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(s.to_lowercase())),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn sum(self) -> Result<Self> {
        if let Value::ValuesList(values_list) = self {
            let value: f64 = values_list
                .iter()
                .map(|value| value.clone().float().unwrap())
                .sum();
            Ok(Value::Double(value))
        } else {
            Err(Error::from(ErrorKind::InvalidOperation))
        }
    }
    pub fn upper(self) -> Result<Self> {
        match self {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => Err(Error::from(ErrorKind::InvalidOperation)),
        }
    }
    pub fn wordcount(self) -> Result<Self> {
        if let Value::String(s) = self {
            let mut is_delim = true;
            let mut count = 0;
            for c in s.chars() {
                if c.is_alphanumeric() & is_delim {
                    count += 1;
                }

                is_delim = !c.is_alphanumeric();
            }
            Ok(Value::Integer(count))
        } else {
            Err(Error::from(ErrorKind::InvalidOperation))
        }
    }
}
