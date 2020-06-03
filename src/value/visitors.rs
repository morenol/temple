use super::Value;
use crate::expression_evaluator::BinaryOperation;
pub struct BinaryMathOperation;

impl BinaryMathOperation {
    pub fn apply(op: &BinaryOperation, left: Value, right: Value) -> Value {
        match op {
            BinaryOperation::Plus => left + right,
            BinaryOperation::Minus => left - right,
            BinaryOperation::Mul => left * right,
            BinaryOperation::Div => left / right,
            BinaryOperation::Modulo => left % right,
            BinaryOperation::DivInteger => {
                let value = left / right;
                if let Value::Double(num) = value {
                    Value::Integer(num as i64)
                } else {
                    todo!()
                }
            }
            BinaryOperation::Pow => left.pow(right),
            BinaryOperation::LogicalEq => Value::Boolean(left == right),
            BinaryOperation::LogicalLe => Value::Boolean(left <= right),
            BinaryOperation::LogicalLt => Value::Boolean(left < right),
            BinaryOperation::LogicalGe => Value::Boolean(left >= right),
            BinaryOperation::LogicalGt => Value::Boolean(left > right),
            BinaryOperation::LogicalNe => Value::Boolean(left != right),
            BinaryOperation::LogicalOr => left | right,
            BinaryOperation::LogicalAnd => left & right,
            BinaryOperation::StringConcat => Value::String(left.to_string() + &right.to_string()),
        }
    }
}

pub struct Subscription;
impl Subscription {
    pub fn apply(value: Value, subscript: Value) -> Value {
        // TODO: Change to Result<Value>
        match (value, subscript) {
            (Value::String(st), Value::Integer(idx)) => {
                Value::String(st.chars().nth(idx as usize).unwrap().to_string())
            }
            (Value::ValuesList(tuple), Value::Integer(idx)) => (&tuple[idx as usize]).clone(),
            (Value::ValuesMap(dict), Value::String(key)) => dict.get(&key).unwrap().clone(),
            _ => todo!(),
        }
    }
}
