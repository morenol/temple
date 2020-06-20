use super::utils::assert_render_template_eq;
use temple::error::Result;
use temple::value::{Value, ValuesMap};
use temple::Context;

#[test]
fn basic_math_expression() -> Result<()> {
    assert_render_template_eq("{{10 + 1}}", "11", None)?;
    assert_render_template_eq("{{ -1 }}", "-1", None)?;
    assert_render_template_eq("{{ 1 - 10}}", "-9", None)?;
    assert_render_template_eq("{{ 2 ** 3 }}", "8", None)?;
    assert_render_template_eq("{{ 0.1 + 1 }}", "1.1", None)?;
    assert_render_template_eq("{{ 1 + 0.33 }}", "1.33", None)?;
    assert_render_template_eq("{{ 0.1 - 10.5 }}", "-10.4", None)?;
    assert_render_template_eq("{{ 2 * 10 }}", "20", None)?;
    assert_render_template_eq("{{ 10 / 4 }}", "2.5", None)?;
    assert_render_template_eq("{{ 10 // 4 }}", "2", None)?;
    assert_render_template_eq("{{ 10 % 3 }}", "1", None)?;
    assert_render_template_eq("{{ 10.5 % 3 }}", "1.5", None)?;
    assert_render_template_eq("{{ 2 ** 3 }}", "8", None)?;
    assert_render_template_eq("{{ 2.5 ** 2 }}", "6.25", None)
}

#[test]
fn basic_string_expression() -> Result<()> {
    assert_render_template_eq("{{ \"hello, world!\" }}", "hello, world!", None)?;
    assert_render_template_eq("{{ 'single quotes' }}", "single quotes", None)?;
    assert_render_template_eq("{{ \"123\" * 3 }}", "123123123", None)?;
    assert_render_template_eq("{{ \"abc\" * 0 }}", "", None)?;
    assert_render_template_eq("{{ \"hello\" + \" \" + \"world\"}}", "hello world", None)?;
    assert_render_template_eq("{{ \"hello \" ~ 123 }}", "hello 123", None)?;
    assert_render_template_eq("{{ \"hello\" ~ \" \" ~ false }}", "hello false", None)
}

#[test]
fn math_order_expression() -> Result<()> {
    assert_render_template_eq("{{ ( 1 + 4 ) * 3 - 1 }}", "14", None)?;
    assert_render_template_eq("{{ ( 1 + 4 ) * (3 - 1) }}", "10", None)?;
    assert_render_template_eq("{{ 1 + 4 * 3 - 1 }}", "12", None)?;
    assert_render_template_eq("{{ -(-1) }}", "1", None)
    // assert_render_template_eq("{{ 5 - 2 - 2 }}", "1", None) TODO: solve left associative operations.
}

#[test]
fn logical_compare() -> Result<()> {
    assert_render_template_eq("{{ 1 == 1 }}", "true", None)?;
    assert_render_template_eq("{{ 1 == 1.0 }}", "true", None)?;
    assert_render_template_eq("{{ 2 > 1.0 }}", "true", None)?;
    assert_render_template_eq("{{ 2.7 < 3.14 }}", "true", None)?;
    assert_render_template_eq("{{ 10 >= -5.0 }}", "true", None)?;
    assert_render_template_eq("{{ 5.0 <= 5  }}", "true", None)?;
    assert_render_template_eq("{{  true != true }}", "false", None)?;
    assert_render_template_eq("{{ false == false }}", "true", None)?;
    assert_render_template_eq("{{ not false == false }}", "false", None)?;
    assert_render_template_eq("{{ \"foo\" == \"bar\" }}", "false", None)?;
    assert_render_template_eq("{{ \"foo\" == \"foo\" }}", "true", None)?;
    assert_render_template_eq("{{ \"bar\" != \"bara\" }}", "true", None)
}

#[test]
fn logical_operators() -> Result<()> {
    assert_render_template_eq("{{ true and false }}", "false", None)?;
    assert_render_template_eq("{{ true and true }}", "true", None)?;
    assert_render_template_eq("{{ false or false }}", "false", None)?;
    assert_render_template_eq("{{ false or true }}", "true", None)
}
#[test]
fn render_lists() -> Result<()> {
    assert_render_template_eq("{{ [] }}", "[]", None)?;
    assert_render_template_eq("{{ [\"a\", \"b\", \"c\"] }}", "[a, b, c]", None)
}

#[test]
fn render_dicts() -> Result<()> {
    assert_render_template_eq("{{ {} }}", "{}", None)?;
    assert_render_template_eq(
        "{{ {\"foo\": \"bar\", \"a\": 10} }}",
        "{\"a\": 10, \"foo\": bar}",
        None,
    )
}

#[test]
fn render_with_context() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("foo".to_string(), Value::Integer(42));
    context.insert("bar".to_string(), Value::Double(3.5));
    let context = Context::new(context);

    assert_render_template_eq("{{ foo }}", "42", Some(context.clone()))?;
    assert_render_template_eq("{{ foo + bar }}", "45.5", Some(context.clone()))
}
#[test]
fn accessors() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("text".to_string(), Value::String("hello".to_string()));
    let context = Context::new(context);
    assert_render_template_eq("{{ text[2] }}", "l", Some(context))?;
    assert_render_template_eq("{{ [0, 1, 2][2] }}", "2", None)?;
    assert_render_template_eq("{{ (0, 1, 2)[2] }}", "2", None)?;
    assert_render_template_eq("{{ {\"one\": 1, \"two\":2}[\"two\"] }}", "2", None)
}
