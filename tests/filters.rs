use super::utils::assert_render_template_eq;
use temple::error::Result;
use temple::value::{Value, ValuesMap};
use temple::Context;

#[test]
fn filter_basic() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("intValue".to_string(), Value::Integer(-1));
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    let context = Context::new(context);
    assert_render_template_eq("{{ intValue | abs }}", "1", Some(context.clone()))?;
    assert_render_template_eq("{{ intValue | float }}", "-1.0", Some(context.clone()))?;
    assert_render_template_eq("{{ stringValue | length }}", "12", Some(context.clone()))?;
    assert_render_template_eq("{{ [0, 1, 2, 3] | length }}", "4", Some(context.clone()))?;
    assert_render_template_eq(
        "{{ {\"key1\": intValue, \"key2\": stringValue, \"key3\": false} | length }}",
        "3",
        Some(context.clone()),
    )
}
#[test]
fn filter_center() -> Result<()> {
    assert_render_template_eq(
        "{{ 'x' | center }}",
        "                                        x                                       ",
        None,
    )?;
    assert_render_template_eq("{{ 'x' | center(width=5)", "  x  ", None)?;
    assert_render_template_eq("{{ 'x' | center(width=0)", "x", None)?;
    assert_render_template_eq("{{ '  x' | center(5) }}", "   x ", None)
}

#[test]
fn filter_int() -> Result<()> {
    assert_render_template_eq("{{ 3.14 | int }}", "3", None)?;
    assert_render_template_eq("{{ undefined | int(default=100) }}", "100", None)?;
    assert_render_template_eq("{{ undefined | int }}", "0", None)
}

#[test]
fn filter_float() -> Result<()> {
    assert_render_template_eq("{{ 3 | float }}", "3.0", None)?;
    assert_render_template_eq("{{ undefined | float(40) }}", "40.0", None)?;
    assert_render_template_eq("{{ undefined | float }}", "0.0", None)?;
    assert_render_template_eq("{{ pi | float(default=3.14) }}", "3.14", None)
}
#[test]
fn filter_last_first() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("intValue".to_string(), Value::Integer(-1));
    context.insert(
        "stringValue".to_string(),
        Value::String("Hello World!".to_string()),
    );
    let context = Context::new(context);

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
    let context = Context::new(context);

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
    let context = Context::new(context);

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
    let context = Context::new(context);
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
#[test]
fn truncate_filter() -> Result<()> {
    assert_render_template_eq("{{ ('a' * 20) | truncate(10) }}", "aaaaaaa...", None)?;
    assert_render_template_eq("{{ ('a' * 20) | truncate(length=10) }}", "aaaaaaa...", None)?;
    assert_render_template_eq(
        "{{ ('a' * 20) | truncate(length=10, end='bc') }}",
        "aaaaaaaabc",
        None,
    )?;
    assert_render_template_eq("{{ ('a' * 20) | truncate }}", "aaaaaaaaaaaaaaaaaaaa", None)
}

#[test]
fn title_filter() -> Result<()> {
    assert_render_template_eq("{{ 'hello world!' | title }}", "Hello World!", None)?;
    assert_render_template_eq("{{ 'HellO wOrlD!' | title }}", "Hello World!", None)
}

#[test]
fn round_filter() -> Result<()> {
    assert_render_template_eq("{{ 5.8 | round }}", "6.0", None)?;
    assert_render_template_eq("{{ 3.14 | round(method='ceil') }}", "4.0", None)?;
    assert_render_template_eq("{{ 5.8 | round(method='floor') }}", "5.0", None)?;
    assert_render_template_eq("{{ 4.834 | round(precision=2) }}", "4.83", None)
}
