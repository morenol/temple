use super::utils::assert_render_template_eq;
use temple::error::Result;

#[test]
fn expected_endraw() -> Result<()> {
    let result = assert_render_template_eq("{% raw %} there is not endraw", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:29: error: {% endraw %} expected".to_string()
    );
    Ok(())
}

#[test]
fn unexpected_endraw() -> Result<()> {
    let result = assert_render_template_eq("{% raw %} {% endraw %} {% endraw %}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:23: error: Unexpected raw block end ('{% endraw %}')".to_string()
    );

    Ok(())
}

#[test]
fn unexpected_endcomment() -> Result<()> {
    let result = assert_render_template_eq("end of comment #}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:15: error: Unexpected comment block end ('#}')".to_string()
    );
    Ok(())
}

#[test]
fn expected_expression() -> Result<()> {
    let result = assert_render_template_eq("{{          }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:12: error: Expression expected".to_string()
    );
    let result = assert_render_template_eq("{{ \"text\"[]         }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:10: error: Expression expected".to_string()
    );

    Ok(())
}

#[test]
fn expected_right_bracket() -> Result<()> {
    let result = assert_render_template_eq("{{ \"text\"[2   }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:14: error: ']' expected".to_string()
    );
    let result = assert_render_template_eq("{{ (2 + 2   }}", "", None);

    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:12: error: ')' expected".to_string()
    );

    Ok(())
}

#[test]
fn undefined_value() -> Result<()> {
    let result = assert_render_template_eq("{{ undefinedValue }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl: error: undefinedValue is not defined.".to_string()
    );
    Ok(())
}
#[test]
fn unexpected_expr_end() -> Result<()> {
    let result = assert_render_template_eq("{%  }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:4: error: Unexpected token".to_string()
    );

    let result = assert_render_template_eq("   }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:3: error: Unexpected expression block end ('}}')".to_string()
    );

    Ok(())
}

#[test]
fn unexpected_statement_end() -> Result<()> {
    let result = assert_render_template_eq("   %}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:3: error: Unexpected statement block end ('%}')".to_string()
    );

    Ok(())
}

#[test]
fn unexpected_raw_begin_end() -> Result<()> {
    let result = assert_render_template_eq("{{ {% raw %} }}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:3: error: Unexpected raw block begin ('{% raw %}')".to_string()
    );

    Ok(())
}

#[test]
fn error_with_multiple_lines() -> Result<()> {
    let result = assert_render_template_eq(
        "{% raw %} 
          {% raw %} 

          {% endraw %} 
{% endraw %}",
        "",
        None,
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:5:0: error: Unexpected raw block end ('{% endraw %}')".to_string()
    );

    Ok(())
}
