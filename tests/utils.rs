use temple::error::Result;
use temple::value::ValuesMap;
use temple::{Template, TemplateEnv};

pub fn assert_render_template_eq(
    input: &str,
    expected: &str,
    params: Option<ValuesMap>,
) -> Result<()> {
    let temp_env: TemplateEnv = TemplateEnv::default();
    let mut template = Template::new(&temp_env)?;
    template.load(input)?;
    let default_context = ValuesMap::default();
    let context = params.unwrap_or(default_context);
    let result = template.render_as_string(context)?;
    assert_eq!(result, expected.to_string());
    Ok(())
}
