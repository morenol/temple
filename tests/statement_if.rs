use super::utils::assert_render_template_eq;
use std::sync::Arc;
use temple::error::Result;
use temple::value::{Value, ValuesMap};

#[test]
fn render_if_body() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("trueValue".to_string(), Value::Boolean(true));
    let context = Arc::new(context);

    assert_render_template_eq(
        "{% if trueValue %}
Hello, world!
{% endif %}",
        "
Hello, world!
",
        Some(context),
    )
}

#[test]
fn dont_render_if_body() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("falseValue".to_string(), Value::Boolean(false));
    let context = Arc::new(context);

    assert_render_template_eq(
        "Only render this.{% if falseValue %}
this not
{% endif %}",
        "Only render this.",
        Some(context),
    )
}

#[test]
fn render_else() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("six".to_string(), Value::Double(6.0));
    let context = Arc::new(context);

    assert_render_template_eq(
        "{% if six < 5 %}
        This should not be rendered
    {% else %}Rendered from else branch{% endif %}",
        "Rendered from else branch",
        Some(context),
    )
}

#[test]
fn render_elif() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("number".to_string(), Value::Double(42.0));
    let context = Arc::new(context);

    assert_render_template_eq(
        "{% if number > 50 %}
        This should not be rendered
    {% elif number == 43 %}Not rendered from elif elif branch
    {% elif number >= 42 %}Rendered from elif branch{% else %} 
    Ignored{% endif %}",
        "Rendered from elif branch",
        Some(context),
    )
}
