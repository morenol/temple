use std::sync::Arc;
use temple::error::Result;
use temple::value::{Value, ValuesMap};
use temple::{Template, TemplateEnv};

#[test]
fn test_global_variable() -> Result<()> {
    let mut temp_env = TemplateEnv::default();
    temp_env.add_global("GLOBAL_VAR".to_string(), "Global");
    let template_env = Arc::new(&temp_env);
    let mut template = Template::new(template_env)?;
    template.load("{{ GLOBAL_VAR }}")?;
    let context = ValuesMap::default();
    let result = template.render_as_string(context)?;
    assert_eq!(result, "Global".to_string());
    Ok(())
}

#[test]
fn test_both_global_and_external_variables() -> Result<()> {
    let mut temp_env = TemplateEnv::default();
    temp_env.add_global("GLOBAL_VAR".to_string(), "Global");
    let template_env = Arc::new(&temp_env);
    let mut template = Template::new(template_env)?;
    template.load(
        "global: {{ GLOBAL_VAR }}
external: {{external_variable}}",
    )?;
    let mut context = ValuesMap::default();
    context.insert(
        "external_variable".to_string(),
        Value::String("External".to_string()),
    );

    let result = template.render_as_string(context)?;
    assert_eq!(
        result,
        "global: Global
external: External"
            .to_string()
    );
    Ok(())
}
#[test]
fn test_override_value() -> Result<()> {
    let mut temp_env = TemplateEnv::default();

    temp_env.add_global("key".to_string(), "Global value");
    let template_env = Arc::new(&temp_env);
    let mut template = Template::new(template_env)?;
    template.load("{{ key }}")?;
    let mut context = ValuesMap::default();
    context.insert(
        "key".to_string(),
        Value::String("overrided value".to_string()),
    );

    let result = template.render_as_string(context)?;
    assert_eq!(result, "overrided value".to_string());
    Ok(())
}
