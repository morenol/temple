use super::utils::assert_render_template_eq;
use temple::error::Result;

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
fn accessors() -> Result<()> {
    assert_render_template_eq("{{ \"hola\"[2] }}", "l", None)
}
