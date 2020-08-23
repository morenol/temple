use super::utils::assert_render_template_eq;
use temple::error::{Error, ErrorKind, Result};

#[test]
fn expected_endraw() -> Result<()> {
    let result = assert_render_template_eq("{% raw %} there is not endraw", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedRawEnd(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "{{% endraw %}} expected".to_string()
    );
    Ok(())
}

#[test]
fn unexpected_endraw() -> Result<()> {
    let result = assert_render_template_eq("{% raw %} {% endraw %} {% endraw %}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::UnexpectedRawEnd(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Unexpected raw block end {{% endraw %}}".to_string()
    );

    Ok(())
}

#[test]
fn unexpected_endcomment() -> Result<()> {
    let result = assert_render_template_eq("end of comment #}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::UnexpectedCommentEnd(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Unexpected comment block end ('#}}')".to_string()
    );
    Ok(())
}

#[test]
fn expected_expression() -> Result<()> {
    let result = assert_render_template_eq("{{          }}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedExpression(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Expression expected".to_string()
    );
    let result = assert_render_template_eq("{{ \"text\"[]         }}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedExpression(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Expression expected".to_string()
    );

    Ok(())
}

#[test]
fn expected_right_bracket() -> Result<()> {
    let result = assert_render_template_eq("{{ \"text\"[2   }}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedSquareBracket(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "']' expected".to_string()
    );
    let result = assert_render_template_eq("{{ (2 + 2   }}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedRoundBracket(_)))
    );

    assert_eq!(
        result.err().unwrap().to_string(),
        "')' expected".to_string()
    );

    Ok(())
}

#[test]
fn undefined_value() -> Result<()> {
    let result = assert_render_template_eq("{{ undefinedValue }}", "", None);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::UndefinedValue(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Value is not defined".to_string()
    );
    Ok(())
}
