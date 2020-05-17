use super::Value;
use std::ops::{Add, Div, Mul, Sub};

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            _ => todo!(),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        todo!()
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        todo!()
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        todo!()
    }
}
