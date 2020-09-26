use crate::context::Context;
use crate::error::{Error, Result};
use crate::renderer::{ComposedRenderer, Render};
use crate::template_env::TemplateEnv;
use crate::template_parser::TemplateParser;
use crate::value::ValuesMap;
use std::borrow::Cow;
use std::io::Write;
use std::sync::Arc;
pub struct Template<'a> {
    body: Cow<'a, str>,
    template_env: Arc<&'a TemplateEnv<'a>>,
    renderer: Option<ComposedRenderer<'a>>,
    template_name: Option<String>,
}

impl<'a> Template<'a> {
    pub fn new(template_env: Arc<&'a TemplateEnv>) -> Result<Self> {
        Ok(Self {
            template_env,
            renderer: None,
            body: Cow::Borrowed(""),
            template_name: None,
        })
    }
    pub fn new_with_filename(
        template_env: Arc<&'a TemplateEnv>,
        template_name: String,
    ) -> Result<Self> {
        Ok(Self {
            template_env,
            renderer: None,
            body: Cow::Borrowed(""),
            template_name: Some(template_name),
        })
    }

    pub fn parse(&self) -> Result<ComposedRenderer<'a>> {
        let mut parser = match &self.body {
            Cow::Borrowed(template_body) => {
                TemplateParser::new(template_body, self.template_env.clone())?
            }
            Cow::Owned(_template_body_owned) => {
                // This allows the parser to have references to the template body.
                // This is safe as long as `body` field is never mutated or dropped.
                let unsafe_source: &'a str = unsafe { &*(&*self.body as *const str) };
                TemplateParser::new(unsafe_source, self.template_env.clone())?
            }
        };
        parser.parse()
    }
    pub fn load<S>(&mut self, tpl_body: S) -> Result<()>
    where
        S: Into<Cow<'a, str>>,
    {
        self.body = tpl_body.into();
        let renderer = self.parse()?;
        self.renderer = Some(renderer);

        Ok(())
    }

    pub fn render_as_string(&self, params: ValuesMap) -> Result<String> {
        let mut b: Vec<u8> = Vec::new();
        let mut context = Context::new(params, self.template_env.clone());
        context.set_global(self.template_env.globals());

        self.render(&mut b, context)?;
        Ok(String::from_utf8(b).expect("Found invalid UTF-8"))
    }
}

impl<'a> Render for Template<'a> {
    fn render(&self, out: &mut dyn Write, params: Context) -> Result<()> {
        if let Some(ref renderer) = self.renderer {
            let result = renderer.render(out, params);
            if let Err(Error::ParseError(mut parse_error)) = result {
                if let Some(tpl_name) = &self.template_name {
                    let mut source = parse_error.location;
                    source.set_filename(tpl_name.to_owned());
                    parse_error.location = source;
                }
                Err(Error::ParseError(parse_error))
            } else {
                result
            }
        } else {
            todo!()
        }
    }
}
