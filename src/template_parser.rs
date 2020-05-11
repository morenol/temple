use crate::error::Result;
use crate::keyword::{KEYWORDS, ROUGH_TOKENIZER};
use crate::template_env::TemplateEnv;
use regex::RegexSet;

#[derive(Debug)]
pub struct TemplateParser<'a, 'b> {
    template_body: &'a str,
    env: &'b TemplateEnv,
    rough_tokenizer: RegexSet,
    keywords: RegexSet,
}

impl<'a, 'b> TemplateParser<'a, 'b> {
    pub fn new(body: Option<&'a String>, env: &'b TemplateEnv) -> Result<Self> {
        let rough_tokenizer = RegexSet::new(ROUGH_TOKENIZER).unwrap();
        let keywords = RegexSet::new(KEYWORDS).unwrap();
        match body {
            Some(template_body) => Ok(Self {
                template_body,
                env,
                rough_tokenizer,
                keywords,
            }),
            None => todo!(),
        }
    }

    pub fn farse(&mut self) -> Result<Renderer> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Renderer {}
