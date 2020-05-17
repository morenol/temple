use jinja2rs::error::Result;
use jinja2rs::{Template, TemplateEnv};
use std::rc::Rc;

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
fn render_plain_singe_line() -> Result<()> {
    test_render_template("Hello, world!", "Hello, world!")
}

#[test]
fn render_plain_multiline() -> Result<()> {
    test_render_template(
        "Hello, world!
Hello, world!",
        "Hello, world!
Hello, world!",
    )
}

#[test]
fn render_multiline_with_comment() -> Result<()> {
    test_render_template(
        "Hello, world!
{#Comment to skip #}Hello, world!",
        "Hello, world!
Hello, world!",
    )
}
