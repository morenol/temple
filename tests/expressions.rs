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
fn basic_math_expression() -> Result<()> {
    test_render_template(
        "{{10 + 1}}
{{ 1 - 10}}
{{ 0.1 + 1 }}
{{ 1 + 0.33 }}
{{ 0.1 - 10.5 }}
{{ 2 * 10 }}
{{ 10 / 4 }}
{{ 10 // 4 }}
{{ 10 % 3 }}
{{ 10.5 % 3 }}
{{ 2 ** 3 }}
{{ 2.5 ** 2 }}",
        "11
-9
1.1
1.33
-10.4
20
2.5
2
1
1.5
8
6.25",
    )
}

#[test]
fn basic_string_expression() -> Result<()> {
    test_render_template(
        "{{ \"123\" * 3 }}
{{ \"abc\" * 0 }}
{{ \"hello\" + \" \" + \"world\"}}",
        "123123123

hello world",
    )
}

#[test]
fn math_order_expression() -> Result<()> {
    test_render_template("{{ ( 1 + 4 ) * 3 - 1 }}", "14")?;
    test_render_template("{{ ( 1 + 4 ) * (3 - 1) }}", "10")?;
    test_render_template("{{ 1 + 4 * 3 - 1 }}", "12")
    // test_render_template("{{ 5 - 2 - 2 }}", "1") TODO: solve left associative operations.
}
