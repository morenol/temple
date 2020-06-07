use super::Value;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub};

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            (Value::Double(left), Value::Integer(right)) => Value::Double(left + right as f64),
            (Value::Integer(left), Value::Double(right)) => Value::Double(left as f64 + right),
            (Value::Double(left), Value::Double(right)) => Value::Double(left + right),
            (Value::Integer(left), Value::Boolean(true)) => Value::Integer(left + 1),
            (Value::Integer(left), Value::Boolean(false)) => Value::Integer(left),
            (Value::Double(left), Value::Boolean(true)) => Value::Double(left + 1_f64),
            (Value::Double(left), Value::Boolean(false)) => Value::Double(left),
            (Value::Boolean(true), Value::Integer(right)) => Value::Integer(right + 1),
            (Value::Boolean(false), Value::Integer(right)) => Value::Integer(right),
            (Value::Boolean(true), Value::Double(right)) => Value::Double(right + 1_f64),
            (Value::Boolean(false), Value::Double(right)) => Value::Double(right),
            (Value::String(left), Value::String(right)) => {
                Value::String(format!("{}{}", left, right))
            }
            _ => Value::Error,
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
            (Value::Integer(left), Value::Boolean(true)) => Value::Integer(left),
            (Value::Integer(_), Value::Boolean(false)) => Value::Integer(0),
            (Value::Double(left), Value::Boolean(true)) => Value::Double(left),
            (Value::Double(_), Value::Boolean(false)) => Value::Double(0.0),
            (Value::Boolean(true), Value::Integer(right)) => Value::Integer(right),
            (Value::Boolean(false), Value::Integer(_)) => Value::Integer(0),
            (Value::Boolean(true), Value::Double(right)) => Value::Double(right),
            (Value::Boolean(false), Value::Double(_)) => Value::Double(0.0),
            (Value::String(left), Value::Integer(right)) => {
                Value::String(left.repeat(right as usize))
            }
            (Value::String(left), Value::Double(right)) => {
                Value::String(left.repeat(right as usize))
            }
            (Value::String(left), Value::Boolean(true)) => Value::String(left),
            (Value::String(_), Value::Boolean(false)) => Value::String("".to_string()),
            _ => Value::Error,
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
            _ => Value::Error,
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
            _ => Value::Error,
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
            _ => Value::Error,
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Integer(value) => Value::Integer(-value),
            Value::Double(value) => Value::Double(-value),
            _ => Value::Error,
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Value::Boolean(boolean) => Value::Boolean(!boolean),
            _ => Value::Error,
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
            _ => Value::Error,
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

impl Ord for Value {
    // Currently implemented using python2 order
    // Python3 does not allow the ordering between
    // certain kind of types.
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
            (Value::Integer(left), Value::Double(right)) => {
                (*left as f64).partial_cmp(&right).unwrap()
            }
            (Value::Double(left), Value::Integer(right)) => {
                left.partial_cmp(&(*right as f64)).unwrap()
            }
            (Value::Double(left), Value::Double(right)) => left.partial_cmp(right).unwrap(),
            (Value::Integer(left), Value::Boolean(true)) => left.cmp(&(1_i64)),
            (Value::Integer(left), Value::Boolean(false)) => left.cmp(&(1_i64)),
            (Value::Double(left), Value::Boolean(true)) => left.partial_cmp(&(1_f64)).unwrap(),
            (Value::Double(left), Value::Boolean(false)) => left.partial_cmp(&(0_f64)).unwrap(),
            (Value::Boolean(left), Value::Boolean(right)) => left.cmp(right),
            (Value::Boolean(true), Value::Integer(right)) => 1_i64.cmp(right),
            (Value::Boolean(false), Value::Integer(right)) => 0i64.cmp(right),
            (Value::Boolean(true), Value::Double(right)) => 1_f64.partial_cmp(right).unwrap(),
            (Value::Boolean(false), Value::Double(right)) => 0_f64.partial_cmp(right).unwrap(),
            (Value::ValuesList(left), Value::ValuesList(right)) => left.cmp(right),
            (Value::ValuesMap(left), Value::ValuesMap(right)) => left.cmp(right),
            (Value::String(left), Value::String(right)) => left.cmp(right),
            (Value::String(_), _) => Ordering::Greater,
            (_, Value::String(_)) => Ordering::Less,
            (Value::ValuesList(_), _) => Ordering::Greater,
            (_, Value::ValuesList(_)) => Ordering::Less,
            (_, Value::Empty) => Ordering::Greater,
            (_, Value::Error) => Ordering::Greater,
            (Value::Empty, _) => Ordering::Less,
            (Value::Error, _) => Ordering::Less,
            (Value::ValuesMap(_), _) => Ordering::Greater,
            (_, Value::ValuesMap(_)) => Ordering::Less,
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
            _ => false,
        }
    }
}
impl Eq for Value {}

impl BitAnd for Value {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left & right),
            _ => Value::Error,
        }
    }
}

impl BitOr for Value {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Boolean(left), Value::Boolean(right)) => Value::Boolean(left || right),
            _ => Value::Error,
        }
    }
}

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
