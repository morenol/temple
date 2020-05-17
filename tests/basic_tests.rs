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

#[test]
fn render_comment_with_code_inside() -> Result<()> {
    test_render_template(
        "(Hello World
{#Comment to
            {{for}}
            {{endfor}}
skip #}
{#Comment to
             {%
 skip #}
from Parser!)",
        "(Hello World


from Parser!)",
    )
}

#[test]
fn render_raw_test() -> Result<()> {
    test_render_template(
        "{% raw %}
    This is a raw text {{ 2 + 2 }}
{% endraw %}",
        "
    This is a raw text {{ 2 + 2 }}
",
    )
}
