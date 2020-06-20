use std::sync::Arc;
use temple::error::Result;
use temple::{Context, Template, TemplateEnv};

pub fn assert_render_template_eq(
    input: &str,
    expected: &str,
    params: Option<Context>,
) -> Result<()> {
    let temp_env = TemplateEnv::default();
    let template_env = Arc::new(temp_env);
    let mut template = Template::new(template_env)?;
    template.load(input)?;
    let default_context = Context::default();
    let context = params.unwrap_or(default_context);
    let result = template.render_as_string(context)?;
    assert_eq!(result, expected.to_string());
    Ok(())
}
