use super::Value;
use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            (Value::Double(left), Value::Integer(right)) => Value::Double(left + right as f64),
            (Value::Integer(left), Value::Double(right)) => Value::Double(left as f64 + right),
            (Value::Double(left), Value::Double(right)) => Value::Double(left + right),
            (Value::String(left), Value::String(right)) => {
                Value::String(format!("{}{}", left, right))
            }
            _ => todo!(),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left * right),
            (Value::Double(left), Value::Integer(right)) => Value::Double(left * right as f64),
            (Value::Integer(left), Value::Double(right)) => Value::Double(left as f64 * right),
            (Value::Double(left), Value::Double(right)) => Value::Double(left * right),
            (Value::String(left), Value::Integer(right)) => {
                Value::String(left.repeat(right as usize))
            }
            (Value::String(left), Value::Double(right)) => {
                Value::String(left.repeat(right as usize))
            }
            _ => todo!(),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => {
                Value::Double(left as f64 / right as f64)
            }
            (Value::Double(left), Value::Integer(right)) => Value::Double(left / right as f64),
            (Value::Integer(left), Value::Double(right)) => Value::Double(left as f64 / right),
            (Value::Double(left), Value::Double(right)) => Value::Double(left / right),
            _ => todo!(),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left - right),
            (Value::Double(left), Value::Integer(right)) => Value::Double(left - right as f64),
            (Value::Integer(left), Value::Double(right)) => Value::Double(left as f64 - right),
            (Value::Double(left), Value::Double(right)) => Value::Double(left - right),
            _ => todo!(),
        }
    }
}

impl Rem for Value {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left % right),
            (Value::Double(left), Value::Integer(right)) => Value::Double(left % right as f64),
            (Value::Integer(left), Value::Double(right)) => Value::Double(left as f64 % right),
            (Value::Double(left), Value::Double(right)) => Value::Double(left % right),
            _ => todo!(),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Integer(value) => Value::Integer(-value),
            Value::Double(value) => Value::Double(-value),
            _ => todo!(),
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Value::Boolean(boolean) => Value::Boolean(!boolean),
            _ => todo!(),
        }
    }
}
impl Value {
    pub fn pow(&self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left.pow(right as u32)),
            (Value::Double(left), Value::Integer(right)) => Value::Double(left.powf(right as f64)),
            (Value::Integer(left), Value::Double(right)) => {
                let left = *left as f64;
                Value::Double(left.powf(right))
            }
            (Value::Double(left), Value::Double(right)) => Value::Double(left.powf(right)),
            _ => todo!(),
        }
    }
}
