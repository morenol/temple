use super::Value;
use std::cmp::{Ordering, PartialEq, PartialOrd};
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
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => left.partial_cmp(&right),
            (Value::Integer(left), Value::Double(right)) => (*left as f64).partial_cmp(&right),
            (Value::Double(left), Value::Integer(right)) => left.partial_cmp(&(*right as f64)),
            (Value::Double(left), Value::Double(right)) => left.partial_cmp(&right),
            _ => todo!(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => left == right,
            (Value::Integer(left), Value::Double(right)) => (*left as f64) == *right,
            (Value::Double(left), Value::Integer(right)) => *left == (*right as f64),
            (Value::Double(left), Value::Double(right)) => left == right,
            (Value::Boolean(left), Value::Boolean(right)) => left == right,
            (Value::String(left), Value::String(right)) => left == right,
            _ => todo!(),
        }
    }
}

mod test {
    use super::Value;

    #[test]
    fn operations() {
        let two = Value::String("2".to_owned());
        let three = Value::Double(3.0);
        let four = Value::Double(4.0);
        let five = Value::Integer(5);

        assert_eq!(three.clone() * five.clone(), Value::Double(15.0));
        assert_eq!(five.clone() * four.clone(), Value::Double(20.0));
        assert_eq!(three.clone() * four.clone(), Value::Double(12.0));
        assert_eq!(three.clone() + four.clone(), Value::Double(7.0));
        assert_eq!(three.clone() / four.clone(), Value::Double(0.75));
        assert_eq!(five.clone() / four.clone(), Value::Double(1.25));
        assert_eq!(three.clone() / five.clone(), Value::Double(0.6));
        assert_eq!(two.clone() * three.clone(), Value::String("222".to_owned()));
        assert_eq!(five.clone() - three.clone(), Value::Double(2.0));
        assert_eq!(-four.clone(), Value::Double(-4.0));
        assert_eq!(!Value::Boolean(true), Value::Boolean(false));
        assert_eq!(five.clone() % three.clone(), Value::Double(2.0));
        assert_eq!(four.clone() % three.clone(), Value::Double(1.0));
        assert_eq!(five.pow(three.clone()), Value::Double(125.0));
        assert_eq!(four.pow(three.clone()), Value::Double(64.0));
    }
}
