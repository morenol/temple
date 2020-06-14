use super::utils::assert_render_template_eq;
use std::sync::Arc;
use temple::error::Result;
use temple::value::{Value, ValuesMap};

#[test]
fn filter_basic() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("intValue".to_string(), Value::Integer(-1));
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    let context = Arc::new(context);
    assert_render_template_eq("{{ intValue | abs }}", "1", Some(context.clone()))?;
    assert_render_template_eq("{{ intValue | float }}", "-1.0", Some(context.clone()))?;
    assert_render_template_eq("{{ stringValue | length }}", "12", Some(context.clone()))?;
    assert_render_template_eq("{{ [0, 1, 2, 3] | length }}", "4", Some(context.clone()))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | length }}",
        "3",
        Some(context.clone()),
    )?;
    assert_render_template_eq("{{ 3.14 | int }}", "3", Some(context))
}

#[test]
fn filter_last_first() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("intValue".to_string(), Value::Integer(-1));
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    let context = Arc::new(context);

    assert_render_template_eq("{{ [0, 1, 2, 3] | first }}", "0", Some(context.clone()))?;
    assert_render_template_eq("{{ stringValue | first }}", "H", Some(context.clone()))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | first }}",
        "-1",
        Some(context.clone()),
    )?;
    assert_render_template_eq("{{ [0, 1, 2, 3] | last }}", "3", Some(context.clone()))?;
    assert_render_template_eq("{{ stringValue | last }}", "!", Some(context.clone()))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | last }}",
        "false",
        Some(context),
    )
}
#[test]
fn filter_lower_upper() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    let context = Arc::new(context);

    assert_render_template_eq(
        "{{ stringValue | lower | capitalize }}",
        "Hello world!",
        Some(context.clone()),
    )?;
    assert_render_template_eq(
        "{{ stringValue | lower }}",
        "hello world!",
        Some(context.clone()),
    )?;
    assert_render_template_eq(
        "{{ stringValue | upper }}",
        "HELLO WORLD!",
        Some(context.clone()),
    )
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
    let context = Arc::new(context);

    assert_render_template_eq(
        "{{ br_tag | escape }}",
        "&lt;/br&gt;",
        Some(context.clone()),
    )?;
    assert_render_template_eq("{{ ampersand | escape }}", "&amp;", Some(context.clone()))?;
    assert_render_template_eq("{{ quotes | escape }}", "&#34;&#39;", Some(context.clone()))
}

#[test]
fn default_filter() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("undefined".to_string(), Value::Empty);
    context.insert("value".to_string(), Value::Integer(1000));
    let context = Arc::new(context);
    assert_render_template_eq(
        "{{ undefined | default(default_value=\"undefined value\") }}",
        "undefined value",
        Some(context.clone()),
    )?;
    assert_render_template_eq(
        "{{ undefined | default(default_value=value) }}",
        "1000",
        Some(context.clone()),
    )?;
    assert_render_template_eq("{{ undefined | default }}", "", Some(context.clone()))
}