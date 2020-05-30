use super::utils::assert_render_template_eq;
use temple::error::Result;

#[test]
fn render_raw_with_whitespace_control() -> Result<()> {
    assert_render_template_eq(
        "{% raw -%}     Some text  
    {%- endraw %}",
        "Some text",
    )?;
    assert_render_template_eq(
        "      {%- raw %}     Some text
  {% endraw -%}  ",
        "     Some text\n  ",
    )?;
    assert_render_template_eq(
        "    {%- raw -%}
Some text
    {%- endraw -%}",
        "Some text",
    )
}

#[test]
fn render_statement_with_whitespace_control() -> Result<()> {
    assert_render_template_eq(
        "  {%- if true -%}    Text striped
    {%- endif %}",
        "Text striped",
    )
}
