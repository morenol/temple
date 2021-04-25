use crate::context::Context;
use crate::error::{Error, Result};
use crate::expression_evaluator::FullExpressionEvaluator;
use std::borrow::Cow;
use std::fmt;
use std::io::Write;
use std::sync::RwLock;

pub struct ComposedRenderer<'a> {
    renderers: RwLock<Vec<Box<dyn Render + 'a>>>,
}

pub trait Render {
    fn render(&self, out: &mut dyn Write, params: Context<'_>) -> Result<()>;
}

impl<'a> ComposedRenderer<'a> {
    pub fn new() -> Self {
        let renderers = RwLock::new(vec![]);
        Self { renderers }
    }
    pub fn add_renderer(&self, renderer: Box<dyn Render + 'a>) {
        self.renderers.write().unwrap().push(renderer)
    }
}

impl<'a> Default for ComposedRenderer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Render for ComposedRenderer<'a> {
    fn render(&self, out: &mut dyn Write, params: Context<'_>) -> Result<()> {
        for r in self.renderers.read().unwrap().iter() {
            r.render(out, params.clone())?;
        }
        Ok(())
    }
}

impl<'a> fmt::Debug for ComposedRenderer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ComposedRenderer")
    }
}

#[derive(Debug)]
pub struct RawTextRenderer<'a> {
    content: Cow<'a, str>,
}

impl<'a> RawTextRenderer<'a> {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            content: content.into(),
        }
    }
}

impl<'a> Render for RawTextRenderer<'a> {
    fn render(&self, out: &mut dyn Write, _params: Context<'_>) -> Result<()> {
        if let Err(err) = out.write(self.content.as_bytes()) {
            Err(Error::Io(err))
        } else {
            Ok(())
        }
    }
}

pub struct ExpressionRenderer<'a> {
    expression: FullExpressionEvaluator<'a>,
}

impl<'a> Render for ExpressionRenderer<'a> {
    fn render(&self, out: &mut dyn Write, params: Context<'_>) -> Result<()> {
        self.expression.render(out, params)
    }
}

impl<'a> ExpressionRenderer<'a> {
    pub fn new(expression: FullExpressionEvaluator<'a>) -> Self {
        Self { expression }
    }
}
