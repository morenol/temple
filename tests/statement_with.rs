use super::utils::assert_render_template_eq;
use temple::error::Result;
use temple::value::{Value, ValuesMap};

#[test]
fn with_simple() -> Result<()> {
    assert_render_template_eq("{% with inner = 42  %}{{ inner }}{% endwith %}", "42", None)
}

#[test]
fn with_multiplee() -> Result<()> {
    assert_render_template_eq(
        "{% with inner = 42, inner2 = \"Hello\"  %}{{ inner2 }}, {{ inner }}{% endwith %}",
        "Hello, 42",
        None,
    )
}

#[test]
fn with_basic() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("outer".to_string(), Value::Integer(100));
    assert_render_template_eq(
        "{{ outer -}}
{% with outer = 'Hello World', inner = outer %}
{{ inner }}
{{ outer }}
{%- endwith %}
{{ outer }}",
        "100
100
Hello World
100",
        Some(context),
    )
}
