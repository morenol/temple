use std::rc::Rc;
use temple::error::{Error, ErrorKind, Result};
use temple::{Template, TemplateEnv};

#[test]
fn expected_endraw() -> Result<()> {
    let temp_env = TemplateEnv::default();
    let template_env = Rc::new(&temp_env);
    let mut template = Template::new(&template_env)?;
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
    let template_env = Rc::new(&temp_env);
    let mut template = Template::new(&template_env)?;
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
    let template_env = Rc::new(&temp_env);
    let mut template = Template::new(&template_env)?;
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
    let template_env = Rc::new(&temp_env);
    let mut template = Template::new(&template_env)?;
    let result = template.load("{{            }}");
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
