use super::utils::assert_render_template_eq;
use temple::error::Result;

#[test]
fn basic_math_expression() -> Result<()> {
    assert_render_template_eq("{{10 + 1}}", "11")?;
    assert_render_template_eq("{{ -1 }}", "-1")?;
    assert_render_template_eq("{{ 1 - 10}}", "-9")?;
    assert_render_template_eq("{{ 2 ** 3 }}", "8")?;
    assert_render_template_eq("{{ 0.1 + 1 }}", "1.1")?;
    assert_render_template_eq("{{ 1 + 0.33 }}", "1.33")?;
    assert_render_template_eq("{{ 0.1 - 10.5 }}", "-10.4")?;
    assert_render_template_eq("{{ 2 * 10 }}", "20")?;
    assert_render_template_eq("{{ 10 / 4 }}", "2.5")?;
    assert_render_template_eq("{{ 10 // 4 }}", "2")?;
    assert_render_template_eq("{{ 10 % 3 }}", "1")?;
    assert_render_template_eq("{{ 10.5 % 3 }}", "1.5")?;
    assert_render_template_eq("{{ 2 ** 3 }}", "8")?;
    assert_render_template_eq("{{ 2.5 ** 2 }}", "6.25")
}

#[test]
fn basic_string_expression() -> Result<()> {
    assert_render_template_eq("{{ \"123\" * 3 }}", "123123123")?;
    assert_render_template_eq("{{ \"abc\" * 0 }}", "")?;
    assert_render_template_eq("{{ \"hello\" + \" \" + \"world\"}}", "hello world")
}

#[test]
fn math_order_expression() -> Result<()> {
    assert_render_template_eq("{{ ( 1 + 4 ) * 3 - 1 }}", "14")?;
    assert_render_template_eq("{{ ( 1 + 4 ) * (3 - 1) }}", "10")?;
    assert_render_template_eq("{{ 1 + 4 * 3 - 1 }}", "12")?;
    assert_render_template_eq("{{ -(-1) }}", "1")
    // assert_render_template_eq("{{ 5 - 2 - 2 }}", "1") TODO: solve left associative operations.
}

#[test]
fn logical_compare() -> Result<()> {
    assert_render_template_eq("{{ 1 == 1 }}", "true")?;
    assert_render_template_eq("{{ 1 == 1.0 }}", "true")?;
    assert_render_template_eq("{{ 2 > 1.0 }}", "true")?;
    assert_render_template_eq("{{ 2.7 < 3.14 }}", "true")?;
    assert_render_template_eq("{{ 10 >= -5.0 }}", "true")?;
    assert_render_template_eq("{{  true != true }}", "false")?;
    assert_render_template_eq("{{ false == false }}", "true")?;
    assert_render_template_eq("{{ \"foo\" == \"bar\" }}", "false")?;
    assert_render_template_eq("{{ \"foo\" == \"foo\" }}", "true")?;
    assert_render_template_eq("{{ \"bar\" != \"bara\" }}", "true")
}
