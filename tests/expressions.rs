use super::utils::assert_render_template_eq;
use temple::error::Result;
use temple::value::{Value, ValuesMap};

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
    assert_render_template_eq("{{ foo }}", "42", Some(&context))?;
    assert_render_template_eq("{{ foo + bar }}", "45.5", Some(&context))
}
#[test]
fn accessors() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("text".to_string(), Value::String("hello".to_string()));
    assert_render_template_eq("{{ text[2] }}", "l", Some(&context))?;
    assert_render_template_eq("{{ [0, 1, 2][2] }}", "2", None)?;
    assert_render_template_eq("{{ (0, 1, 2)[2] }}", "2", None)?;
    assert_render_template_eq("{{ {\"one\": 1, \"two\":2}[\"two\"] }}", "2", None)
}

#[test]
fn filter_basic() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("intValue".to_string(), Value::Integer(-1));
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    assert_render_template_eq("{{ intValue | abs }}", "1", Some(&context))?;
    assert_render_template_eq("{{ intValue | float }}", "-1.0", Some(&context))?;
    assert_render_template_eq("{{ stringValue | length }}", "12", Some(&context))?;
    assert_render_template_eq("{{ [0, 1, 2, 3] | length }}", "4", Some(&context))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | length }}",
        "3",
        Some(&context),
    )?;
    assert_render_template_eq("{{ 3.14 | int }}", "3", Some(&context))
}

#[test]
fn filter_last_first() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("intValue".to_string(), Value::Integer(-1));
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    assert_render_template_eq("{{ [0, 1, 2, 3] | first }}", "0", Some(&context))?;
    assert_render_template_eq("{{ stringValue | first }}", "H", Some(&context))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | first }}",
        "-1",
        Some(&context),
    )?;
    assert_render_template_eq("{{ [0, 1, 2, 3] | last }}", "3", Some(&context))?;
    assert_render_template_eq("{{ stringValue | last }}", "!", Some(&context))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | last }}",
        "false",
        Some(&context),
    )
}
#[test]
fn filter_lower_upper() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );

    assert_render_template_eq(
        "{{ stringValue | lower | capitalize }}",
        "Hello world!",
        Some(&context),
    )?;
    assert_render_template_eq("{{ stringValue | lower }}", "hello world!", Some(&context))?;
    assert_render_template_eq("{{ stringValue | upper }}", "HELLO WORLD!", Some(&context))
}
#[test]
fn filter_minmax() -> Result<()> {
    assert_render_template_eq(
        "{{ {\"key1\": 3.14, \"key2\": 2.0, \"key3\": false} | max }}",
        "false",
        None,
    )?;
    assert_render_template_eq(
        "{{ {\"key1\": 3.14, \"key2\": 2.0, \"key3\": false} | min }}",
        "3.14",
        None,
    )?;
    assert_render_template_eq("{{ [true, 100, 25, -3] | max }}", "100", None)?;
    assert_render_template_eq("{{ [10, false, -5, 0] | min }}", "-5", None)?;
    assert_render_template_eq("{{ \"foobar\" | max }}", "r", None)?;
    assert_render_template_eq("{{ \"foobar\" | min }}", "a", None)
}

#[test]
fn filter_sum() -> Result<()> {
    assert_render_template_eq("{{ [10, 15, 20, -5, 2.5, -4.1] | sum }}", "38.4", None)
}

#[test]
fn multiple_filters() -> Result<()> {
    assert_render_template_eq("{{ \"foobar\" | upper | first }}", "F", None)
}

#[test]
fn filter_to_string() -> Result<()> {
    assert_render_template_eq("{{ 1000 | string | length }}", "4", None)?;
    assert_render_template_eq("{{ [10, 100] | string | first }}", "[", None)
}

#[test]
fn filter_word_count() -> Result<()> {
    assert_render_template_eq("{{ \"Hello, world!\" | wordcount }}", "2", None)?;
    assert_render_template_eq("{{ \"    \" | wordcount }}", "0", None)?;
    assert_render_template_eq("{{ \" hello   \" | wordcount }}", "1", None)
}

#[test]
fn filter_escape() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("br_tag".to_string(), Value::String("</br>".to_string()));
    context.insert("ampersand".to_string(), Value::String("&".to_string()));
    context.insert("quotes".to_string(), Value::String("\"\'".to_string()));
    assert_render_template_eq("{{ br_tag | escape }}", "&lt;/br&gt;", Some(&context))?;
    assert_render_template_eq("{{ ampersand | escape }}", "&amp;", Some(&context))?;
    assert_render_template_eq("{{ quotes | escape }}", "&#34;&#39;", Some(&context))
}
