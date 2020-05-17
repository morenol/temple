use std::rc::Rc;
use temple::error::Result;
use temple::{Template, TemplateEnv};

fn test_render_template(input: &str, expected: &str) -> Result<()> {
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(&temp_env);
    let mut template = Template::new(&template_env)?;
    template.load(input)?;
    let result = template.render_as_string()?;
    assert_eq!(result, expected.to_string());
    Ok(())
}

#[test]
fn basic_expression() -> Result<()> {
    test_render_template("{{10 + 1}}", "11")
}
