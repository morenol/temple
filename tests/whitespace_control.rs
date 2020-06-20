use super::utils::assert_render_template_eq;
use temple::error::Result;
use temple::value::{Value, ValuesMap};
use temple::Context;

#[test]
fn render_raw_with_whitespace_control() -> Result<()> {
    assert_render_template_eq(
        "{% raw -%}     Some text  
    {%- endraw %}",
        "Some text",
        None,
    )?;
    assert_render_template_eq(
        "      {%- raw %}     Some text
  {% endraw -%}  ",
        "     Some text\n  ",
        None,
    )?;
    assert_render_template_eq(
        "    {%- raw -%}
Some text
    {%- endraw -%}",
        "Some text",
        None,
    )
}

#[test]
fn render_statement_with_whitespace_control() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("trueValue".to_string(), Value::Boolean(true));
    let context = Context::new(context);

    assert_render_template_eq(
        "  {%- if trueValue -%}    Text striped
    {%- endif %}",
        "Text striped",
        Some(context),
    )
}
