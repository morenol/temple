use crate::context::Context;
use crate::error::Result;
use crate::expression_evaluator::CallParams;
use crate::value::Value;
pub enum Filter {
    Abs,
    Capitalize,
    Default,
    Escape,
    First,
    Float,
    Int,
    Last,
    Length,
    Lower,
    Max,
    Min,
    String,
    Sum,
    Truncate,
    Upper,
    WordCount,
}
impl Filter {
    pub fn new(name: &str) -> Result<Self> {
        match name {
            "abs" => Ok(Filter::Abs),
            "capitalize" => Ok(Filter::Capitalize),
            "default" | "d" => Ok(Filter::Default),
            "escape" | "e" => Ok(Filter::Escape),
            "first" => Ok(Filter::First),
            "float" => Ok(Filter::Float),
            "int" => Ok(Filter::Int),
            "last" => Ok(Filter::Last),
            "length" | "count" => Ok(Filter::Length),
            "lower" => Ok(Filter::Lower),
            "max" => Ok(Filter::Max),
            "min" => Ok(Filter::Min),
            "string" => Ok(Filter::String),
            "sum" => Ok(Filter::Sum),
            "truncate" => Ok(Filter::Truncate),
            "upper" => Ok(Filter::Upper),
            "wordcount" => Ok(Filter::WordCount),
            _ => todo!(),
        }
    }
    pub fn filter<'a>(
        &self,
        base_value: Value,
        params: &Option<CallParams<'a>>,
        context: Context,
    ) -> Result<Value> {
        match &self {
            Filter::Abs => base_value.abs(),
            Filter::Capitalize => base_value.capitalize(),
            Filter::Default => base_value.default_filter(params, context),
            Filter::Escape => base_value.escape(),
            Filter::First => base_value.first(),
            Filter::Int => Ok(Value::Integer(base_value.int()?)), // TODO change to accept parameters
            Filter::Float => Ok(Value::Double(base_value.float()?)), // TODO change to accept parameters
            Filter::Last => base_value.last(),
            Filter::Length => Ok(Value::Integer(base_value.len()? as i64)),
            Filter::Lower => base_value.lower(),
            Filter::Max => base_value.max(), // TODO Accept params
            Filter::Min => base_value.min(), // TODO Accept params
            Filter::String => Ok(Value::String(base_value.to_string())),
            Filter::Sum => base_value.sum(), // TODO: ACcept params
            Filter::Truncate => base_value.truncate(params, context),
            Filter::Upper => base_value.upper(),
            Filter::WordCount => base_value.wordcount(),
        }
    }
}

pub struct FilterExpression<'a> {
    filter: Filter,
    params: Option<CallParams<'a>>,
    parent: Option<Box<FilterExpression<'a>>>,
}

impl<'a> FilterExpression<'a> {
    pub fn new(identifier: &str, params: Option<CallParams<'a>>) -> Result<Self> {
        let filter = Filter::new(identifier)?;
        Ok(Self {
            filter,
            params,
            parent: None,
        })
    }
    pub fn set_parent_filter(&mut self, parent: FilterExpression<'a>) {
        self.parent = Some(Box::new(parent));
    }

    pub fn filter(&self, base_value: Value, context: Context) -> Result<Value> {
        if self.parent.is_some() {
            self.filter.filter(
                self.parent
                    .as_ref()
                    .unwrap()
                    .filter(base_value, context.clone())?,
                &self.params,
                context,
            )
        } else {
            self.filter.filter(base_value, &self.params, context)
        }
    }
}
