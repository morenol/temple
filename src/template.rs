use crate::error::Result;
use crate::template_env::TemplateEnv;
use crate::template_parser::{Renderer, TemplateParser};
use std::rc::Rc;

#[derive(Debug)]
pub struct Template {
    template_env: Rc<TemplateEnv>,
    template_body: Option<String>,
    renderer: Option<Renderer>,
}

impl Template {
    pub fn new(template_env: &Rc<TemplateEnv>) -> Result<Self> {
        Ok(Self {
            template_env: template_env.clone(),
            template_body: None,
            renderer: None,
        })
    }

    pub fn load<S>(&mut self, tpl_body: S) -> Result<()>
    where
        S: Into<String>,
    {
        let s = tpl_body.into();
        self.template_body = Some(s);
        let parser = TemplateParser::new(self.template_body.as_ref(), &*self.template_env);
        todo!()
    }

    pub fn render(&self) -> Result<()> {
        todo!()
    }

    pub fn render_as_string(&self) -> Result<String> {
        todo!()
    }
}
