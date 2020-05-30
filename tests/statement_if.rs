use super::utils::assert_render_template_eq;
use temple::error::Result;

#[test]
fn render_if_body() -> Result<()> {
    assert_render_template_eq(
        "{% if true %}
Hello, world!
{% endif %}",
        "
Hello, world!
",
    )
}

#[test]
fn dont_render_if_body() -> Result<()> {
    assert_render_template_eq(
        "Only render this.{% if false %}
this not
{% endif %}",
        "Only render this.",
    )
}

#[test]
fn render_else() -> Result<()> {
    assert_render_template_eq(
        "{% if true == false %}
        This should not be rendered
    {% else %}Rendered from else branch{% endif %}",
        "Rendered from else branch",
    )
}

#[test]
fn render_elif() -> Result<()> {
    assert_render_template_eq(
        "{% if 5 > 7 %}
        This should not be rendered
    {% elif 5 == 6 %}Not rendered from elif elif branch
    {% elif 5 == 5 %}Rendered from elif branch{% else %} 
    Ignored{% endif %}",
        "Rendered from elif branch",
    )
}
