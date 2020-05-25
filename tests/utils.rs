use std::rc::Rc;
use temple::error::Result;
use temple::{Template, TemplateEnv};

pub fn assert_render_template_eq(input: &str, expected: &str) -> Result<()> {
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(&temp_env);
    let mut template = Template::new(&template_env)?;
    template.load(input)?;
    let result = template.render_as_string()?;
    assert_eq!(result, expected.to_string());
    Ok(())
}
