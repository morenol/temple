use std::sync::Arc;
use temple::error::Result;
use temple::value::{Value, ValuesMap};
use temple::{MemoryFileSystem, Template, TemplateEnv};

fn assert_render_template_with_includes_eq(
    input: &str,
    expected: &str,
    params: Option<ValuesMap>,
) -> Result<()> {
    let mut temp_env = TemplateEnv::default();

    let mut handler = MemoryFileSystem::new();
    handler.add_file("simple.j2".to_string(), "Hello world!".to_string());
    handler.add_file("header.j2".to_string(), "[{{ foo }}|{{ bar}}]".to_string());
    handler.add_file("o_printer.j2".to_string(), "({{ o }})".to_string());
    temp_env.add_filesystem_handler(Box::new(handler))?;

    temp_env.add_global("bar".to_string(), 23);
    temp_env.add_global("o".to_string(), 0);
    let template_env = Arc::new(&temp_env);
    let mut template = Template::new(template_env)?;
    template.load(input)?;
    let default_context = ValuesMap::default();
    let context = params.unwrap_or(default_context);
    let result = template.render_as_string(context)?;
    assert_eq!(result, expected.to_string());
    Ok(())
}

#[test]
fn simple_include() -> Result<()> {
    assert_render_template_with_includes_eq("{% include \"simple.j2\" %}", "Hello world!", None)
}

#[test]
fn include_with_context() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("foo".to_string(), Value::Integer(42));
    assert_render_template_with_includes_eq(
        "{% include \"header.j2\" %}",
        "[42|23]",
        Some(context.clone()),
    )?;
    assert_render_template_with_includes_eq(
        "{% include \"header.j2\" with context %}",
        "[42|23]",
        Some(context),
    )
}

#[test]
fn include_without_context() -> Result<()> {
    let mut context = ValuesMap::default();
    context.insert("o".to_string(), Value::Integer(42));
    assert_render_template_with_includes_eq(
        "{% include \"o_printer.j2\" without context %}",
        "(0)",
        Some(context),
    )
}

#[test]
fn include_ignore_missing() -> Result<()> {
    assert_render_template_with_includes_eq(
        "{% include \"missing_inner_header.j2\" ignore missing %}",
        "",
        None,
    )
}

#[test]
fn error_include_missing() -> Result<()> {
    let result = assert_render_template_with_includes_eq(
        "{% include \"missing_inner_header.j2\" %}",
        "",
        None,
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl: error: Template missing_inner_header.j2 not found.".to_string()
    );

    Ok(())
}

#[test]
fn error_include_ignore_missing() -> Result<()> {
    let result = assert_render_template_with_includes_eq(
        "{% include \"missing_inner_header.j2\" ignore mising %}",
        "",
        None,
    );
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:2: error: Specific token expected (missing)".to_string()
    );

    Ok(())
}

#[test]
fn error_include_without_context() -> Result<()> {
    let result =
        assert_render_template_with_includes_eq("{% include \"simple.j2\" without c %}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:2: error: Specific token expected (context)".to_string()
    );

    Ok(())
}

#[test]
fn error_include_with_context() -> Result<()> {
    let result =
        assert_render_template_with_includes_eq("{% include \"simple.j2\" with c %}", "", None);
    assert_eq!(
        result.err().unwrap().to_string(),
        "noname.j2tpl:1:2: error: Specific token expected (context)".to_string()
    );

    Ok(())
}
