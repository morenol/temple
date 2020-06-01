use crate::error::Result;
use crate::expression_evaluator::Evaluate;
use crate::lexer::Token;
use crate::renderer::ComposedRenderer;
use crate::renderer::Render;
use crate::value::{Value, ValuesMap};
use std::io::Write;
use std::rc::Rc;
pub mod parser;
pub struct IfStatement<'a> {
    expression: Box<dyn Evaluate + 'a>,
    body: Option<Rc<ComposedRenderer<'a>>>,
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
    fn set_main_body(&mut self, body: Rc<ComposedRenderer<'a>>) {
        let if_body = body.clone();
        self.body = Some(if_body);
    }
    pub fn add_else_branch(&mut self, branch: Statement<'a>) {
        self.else_branches.push(branch);
    }
}
impl<'a> Render for IfStatement<'a> {
    fn render(&self, out: &mut dyn Write, params: &ValuesMap) -> Result<()> {
        let value = self.expression.evaluate(params)?;
        if let Value::Boolean(true) = value {
            self.body.as_ref().unwrap().render(out, params)?
        } else {
            for branch in &self.else_branches {
                if let Statement::Else(else_branch) = branch {
                    if else_branch.should_render(params) {
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
    body: Option<Rc<ComposedRenderer<'a>>>,
}

impl<'a> ElseStatement<'a> {
    pub fn new(expression: Option<Box<dyn Evaluate + 'a>>) -> Self {
        Self {
            expression,
            body: None,
        }
    }
    fn set_main_body(&mut self, body: Rc<ComposedRenderer<'a>>) {
        let else_body = body.clone();
        self.body = Some(else_body);
    }

    fn should_render(&self, values: &ValuesMap) -> bool {
        self.expression.is_none()
            || match self.expression.as_ref().unwrap().evaluate(values) {
                Ok(Value::Boolean(boolean)) => boolean,
                _ => todo!(),
            }
    }
}
impl<'a> Render for ElseStatement<'a> {
    fn render(&self, out: &mut dyn Write, params: &ValuesMap) -> Result<()> {
        self.body.as_ref().unwrap().render(out, params)
    }
}

pub enum Statement<'a> {
    If(IfStatement<'a>),
    Else(ElseStatement<'a>),
}
impl<'a> Statement<'a> {
    pub fn set_main_body(&mut self, body: Rc<ComposedRenderer<'a>>) {
        match self {
            Statement::If(statement) => statement.set_main_body(body),
            Statement::Else(statement) => statement.set_main_body(body),
        }
    }
    pub fn add_else_branch(&mut self, branch: Statement<'a>) {
        match self {
            Statement::If(statement) => statement.add_else_branch(branch),
            Statement::Else(_statement) => todo!(),
        }
    }
}
impl<'a> Render for Statement<'a> {
    fn render(&self, out: &mut dyn Write, params: &ValuesMap) -> Result<()> {
        match self {
            Statement::If(statement) => statement.render(out, params),
            Statement::Else(statement) => statement.render(out, params),
        }
    }
}

pub struct StatementInfo<'a> {
    mode: StatementInfoType,
    pub current_composition: Rc<ComposedRenderer<'a>>,
    compositions: Vec<Rc<ComposedRenderer<'a>>>,
    token: Token<'a>,
    renderer: Option<Statement<'a>>,
}

pub enum StatementInfoType {
    TemplateRoot,
    IfStatement,
    ElseIfStatement,
    ForStatement,
    SetStatement,
    ExtendsStatement,
    BlockStatement,
    ParentBlockStatement,
    MacroStatement,
    MacroCallStatement,
    WithStatement,
    FilterStatement,
}

impl<'a> StatementInfo<'a> {
    pub fn new(
        mode: StatementInfoType,
        token: Token<'a>,
        renderers: Rc<ComposedRenderer<'a>>,
    ) -> Self {
        let current_composition = renderers.clone();
        let compositions = vec![renderers];
        Self {
            mode,
            token,
            current_composition,
            compositions,
            renderer: None,
        }
    }
}

pub type StatementInfoList<'a> = Vec<StatementInfo<'a>>;
