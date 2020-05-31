use crate::error::Result;
use crate::renderer::{ComposedRenderer, Render};
use crate::template_env::TemplateEnv;
use crate::template_parser::TemplateParser;
use crate::value::ValuesMap;
use std::io::Write;
use std::rc::Rc;

#[derive(Debug)]
pub struct Template<'a> {
    template_env: Rc<&'a TemplateEnv>,
    template_body: Option<&'a str>,
    renderer: Option<ComposedRenderer<'a>>,
}

impl<'a> Template<'a> {
    pub fn new(template_env: &Rc<&'a TemplateEnv>) -> Result<Self> {
        Ok(Self {
            template_env: template_env.clone(),
            template_body: None,
            renderer: None,
        })
    }

    pub fn load(&mut self, tpl_body: &'a str) -> Result<()> {
        self.template_body = Some(tpl_body);

        let mut parser = TemplateParser::new(tpl_body, &*self.template_env)?;
        self.renderer = Some(parser.parse()?);

        Ok(())
    }

    pub fn render_as_string(&self, params: &ValuesMap) -> Result<String> {
        let mut b: Vec<u8> = Vec::new();
        self.render(&mut b, params);
        Ok(String::from_utf8(b).expect("Found invalid UTF-8"))
    }
}

impl<'a> Render for Template<'a> {
    fn render(&self, out: &mut dyn Write, params: &ValuesMap) {
        if let Some(ref renderer) = self.renderer {
            renderer.render(out, params)
        }
    }
}
