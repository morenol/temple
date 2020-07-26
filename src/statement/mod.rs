use crate::context::Context;
use crate::error::Result;
use crate::expression_evaluator::Evaluate;
use crate::lexer::Token;
use crate::renderer::ComposedRenderer;
use crate::renderer::Render;
use crate::value::{Value, ValuesList, ValuesMap};
use std::io::Write;
use std::sync::Arc;
pub mod parser;
pub struct IfStatement<'a> {
    expression: Box<dyn Evaluate + 'a>,
    body: Option<Arc<ComposedRenderer<'a>>>,
    else_branches: Vec<Statement<'a>>,
}
impl<'a> IfStatement<'a> {
    pub fn new(expression: Box<dyn Evaluate + 'a>) -> Self {
        Self {
            expression,
            body: None,
            else_branches: vec![],
        }
    }
    fn set_main_body(&mut self, body: Arc<ComposedRenderer<'a>>) {
        let if_body = body.clone();
        self.body = Some(if_body);
    }
    pub fn add_else_branch(&mut self, branch: Statement<'a>) {
        self.else_branches.push(branch);
    }
}
impl<'a> Render for IfStatement<'a> {
    fn render(&self, out: &mut dyn Write, params: Context) -> Result<()> {
        let value = self.expression.evaluate(params.clone())?;
        if let Value::Boolean(true) = value {
            self.body.as_ref().unwrap().render(out, params)?
        } else {
            for branch in &self.else_branches {
                if let Statement::Else(else_branch) = branch {
                    if else_branch.should_render(params.clone()) {
                        branch.render(out, params)?;
                        break;
                    }
                } else {
                    todo!()
                }
            }
        };
        Ok(())
    }
}

pub struct ElseStatement<'a> {
    expression: Option<Box<dyn Evaluate + 'a>>,
    body: Option<Arc<ComposedRenderer<'a>>>,
}

impl<'a> ElseStatement<'a> {
    pub fn new(expression: Option<Box<dyn Evaluate + 'a>>) -> Self {
        Self {
            expression,
            body: None,
        }
    }
    fn set_main_body(&mut self, body: Arc<ComposedRenderer<'a>>) {
        let else_body = body.clone();
        self.body = Some(else_body);
    }

    fn should_render(&self, values: Context) -> bool {
        self.expression.is_none()
            || match self.expression.as_ref().unwrap().evaluate(values) {
                Ok(Value::Boolean(boolean)) => boolean,
                _ => todo!(),
            }
    }
}
impl<'a> Render for ElseStatement<'a> {
    fn render(&self, out: &mut dyn Write, params: Context) -> Result<()> {
        self.body.as_ref().unwrap().render(out, params)
    }
}
pub struct WithStatement<'a> {
    scope_vars: Vec<(String, Box<dyn Evaluate + 'a>)>,
    body: Option<Arc<ComposedRenderer<'a>>>,
}
impl<'a> WithStatement<'a> {
    pub fn new(scope_vars: Vec<(String, Box<dyn Evaluate + 'a>)>) -> Self {
        Self {
            scope_vars,
            body: None,
        }
    }
    fn set_main_body(&mut self, body: Arc<ComposedRenderer<'a>>) {
        let with_body = body.clone();
        self.body = Some(with_body);
    }
}
impl<'a> Render for WithStatement<'a> {
    fn render(&self, out: &mut dyn Write, params: Context) -> Result<()> {
        let mut inner_values = params.clone();
        let scope = inner_values.enter_scope();
        for (name, value) in &self.scope_vars {
            let mut scope = scope.write().unwrap();
            scope.insert(name.to_string(), value.evaluate(params.clone())?);
        }
        self.body.as_ref().unwrap().render(out, inner_values)
    }
}

pub struct ForStatement<'a> {
    vars: Vec<String>,
    value: Box<dyn Evaluate + 'a>,
    body: Option<Arc<ComposedRenderer<'a>>>,
}

impl<'a> ForStatement<'a> {
    pub fn new(vars: Vec<String>, value: Box<dyn Evaluate + 'a>) -> Self {
        Self {
            vars,
            value,
            body: None,
        }
    }
    fn set_main_body(&mut self, body: Arc<ComposedRenderer<'a>>) {
        let for_body = body.clone();
        self.body = Some(for_body);
    }
    fn render_loop(
        &self,
        loop_value: Value,
        out: &mut dyn Write,
        mut params: Context,
        _level: usize,
    ) -> Result<()> {
        let loop_items: ValuesList = loop_value.into();
        let items_size = loop_items.len();
        let context = params.enter_scope();
        for (item_idx, item) in loop_items.iter().enumerate() {
            let mut loop_map = ValuesMap::default();
            loop_map.insert("index".to_string(), Value::Integer((item_idx + 1) as i64));
            loop_map.insert("index0".to_string(), Value::Integer(item_idx as i64));
            loop_map.insert("first".to_string(), Value::Boolean(item_idx == 0));
            loop_map.insert(
                "last".to_string(),
                Value::Boolean(item_idx == items_size - 1),
            );

            {
                let mut context = context.write().unwrap();
                if self.vars.len() > 1 {
                    todo!();
                } else {
                    context.insert(self.vars[0].clone(), item.clone());
                }
                context.insert("loop".to_string(), Value::ValuesMap(loop_map));
            }
            params.enter_scope();
            self.body.as_ref().unwrap().render(out, params.clone())?;
            params.exit_scope();
        }

        params.exit_scope();
        Ok(())
    }
}
impl<'a> Render for ForStatement<'a> {
    fn render(&self, out: &mut dyn Write, params: Context) -> Result<()> {
        let loop_value = self.value.evaluate(params.clone())?;
        self.render_loop(loop_value, out, params, 0)?;
        Ok(())
    }
}

pub enum Statement<'a> {
    If(IfStatement<'a>),
    Else(ElseStatement<'a>),
    For(ForStatement<'a>),
    With(WithStatement<'a>),
}
impl<'a> Statement<'a> {
    pub fn set_main_body(&mut self, body: Arc<ComposedRenderer<'a>>) {
        match self {
            Statement::If(statement) => statement.set_main_body(body),
            Statement::Else(statement) => statement.set_main_body(body),
            Statement::For(statement) => statement.set_main_body(body),
            Statement::With(statement) => statement.set_main_body(body),
        }
    }
    pub fn add_else_branch(&mut self, branch: Statement<'a>) {
        match self {
            Statement::If(statement) => statement.add_else_branch(branch),
            Statement::Else(_statement) => todo!(),
            _ => unreachable!(),
        }
    }
}
impl<'a> Render for Statement<'a> {
    fn render(&self, out: &mut dyn Write, params: Context) -> Result<()> {
        match self {
            Statement::If(statement) => statement.render(out, params),
            Statement::Else(statement) => statement.render(out, params),
            Statement::For(statement) => statement.render(out, params),
            Statement::With(statement) => statement.render(out, params),
        }
    }
}

pub struct StatementInfo<'a> {
    mode: StatementInfoType,
    pub current_composition: Arc<ComposedRenderer<'a>>,
    compositions: Vec<Arc<ComposedRenderer<'a>>>,
    _token: Token<'a>,
    renderer: Option<Statement<'a>>,
}

pub enum StatementInfoType {
    TemplateRoot,
    IfStatement,
    ElseIfStatement,
    ForStatement,
    WithStatement,
}

impl<'a> StatementInfo<'a> {
    pub fn new(
        mode: StatementInfoType,
        _token: Token<'a>,
        renderers: Arc<ComposedRenderer<'a>>,
    ) -> Self {
        let current_composition = renderers.clone();
        let compositions = vec![renderers];
        Self {
            mode,
            _token,
            current_composition,
            compositions,
            renderer: None,
        }
    }
}

pub type StatementInfoList<'a> = Vec<StatementInfo<'a>>;
