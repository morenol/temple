use jinja::error::Result;
use jinja::{Template, TemplateEnv};
use std::rc::Rc;

#[test]
fn render_hello_world() -> Result<()> {
    let template_env = Rc::new(TemplateEnv::default());
    let mut template = Template::new(&template_env)?;
    template.load("Hello world", None)?;
    let result = template.render_as_string()?;
    assert_eq!(result, "Hello world".to_string());
    Ok(())
}
