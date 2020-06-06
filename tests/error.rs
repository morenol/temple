use std::rc::Rc;
use temple::error::{Error, ErrorKind, Result};
use temple::value::ValuesMap;
use temple::{Template, TemplateEnv};

#[test]
fn expected_endraw() -> Result<()> {
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(temp_env);
    let mut template = Template::new(template_env)?;
    let result = template.load("{% raw %} there is not endraw");
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
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(temp_env);
    let mut template = Template::new(template_env)?;
    let result = template.load("{% raw %} {% endraw %} {% endraw %}");
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
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(temp_env);
    let mut template = Template::new(template_env)?;
    let result = template.load("end of comment #}");
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
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(temp_env);
    let mut template = Template::new(template_env)?;
    let result = template.load("{{            }}");
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedExpression(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Expression expected".to_string()
    );
    let result = template.load("{{ \"text\"[] ");
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
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(temp_env);
    let mut template = Template::new(template_env)?;
    let result = template.load("{{ \"text\"[2 }}");
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::ExpectedSquareBracket(_)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "']' expected".to_string()
    );
    let result = template.load("{{ (2 + 2   }}");
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
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(temp_env);
    let mut template = Template::new(template_env)?;
    template.load("{{ undefinedValue }}")?;
    let context = ValuesMap::default();
    let result = template.render_as_string(&context);
    assert_matches!(
        result,
        Err(Error::ParseRender(ErrorKind::UndefinedValue(_, _)))
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "Value is not defined".to_string()
    );
    Ok(())
}
