use super::utils::assert_render_template_eq;
use temple::error::Result;
use temple::value::{Value, ValuesMap};

#[test]
fn for_over_string() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("word".to_string(), Value::String("hello".to_string()));
    assert_render_template_eq(
        "{% for letter in word  %} {{ letter }}{% endfor %}",
        " h e l l o",
        Some(context),
    )
}

#[test]
fn for_over_list_of_numbers() -> Result<()> {
    let context = ValuesMap::default();
    assert_render_template_eq(
        "{% for even in [2, 4, 6, 8, 10]  %}{% if not loop[\"first\"] %} {%endif %}{{ even // 2 }}{% if loop[\"last\"] %}.{% else %},{% endif %}{% endfor %}",
        "1, 2, 3, 4, 5.",
        Some(context),
    )
}

#[test]
fn nested_for() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("word".to_string(), Value::String("abc".to_string()));
    assert_render_template_eq(
        "{% for letter in word  -%}
{%- for num in [1, 2, 3] -%}
  {{ loop[\"first\"] }}
{% endfor -%}
{{ loop[\"first\"] }}
{% endfor -%}",
        "  true
  false
  false
true
  true
  false
  false
false
  true
  false
  false
false
",
        Some(context),
    )
}
