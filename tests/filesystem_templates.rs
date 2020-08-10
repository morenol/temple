use temple::error::Result;
use temple::value::Value;
use temple::{Context, MemoryFileSystem, RealFileSystem, TemplateEnv};

#[test]
pub fn real_filesystem_basic_template() -> Result<()> {
    let mut temp_env = TemplateEnv::default();
    let handler = RealFileSystem::new("tests/tests_data".to_string());
    temp_env.add_filesystem_handler(Box::new(handler))?;
    let template = temp_env.load_template("simple.j2")?;
    let context = Context::default();
    let result = template.render_as_string(context)?;
    assert_eq!(result, "Hello World!\n".to_string());
    Ok(())
}

#[test]
pub fn memory_filesystem_basic_template() -> Result<()> {
    let mut temp_env = TemplateEnv::default();
    temp_env.add_global("key".to_string(), Value::String("Global value".to_string()));
    let mut handler = MemoryFileSystem::new();
    handler.add_file("simple2.j2".to_string(), "Hello Rustaceans!".to_string());
    temp_env.add_filesystem_handler(Box::new(handler))?;
    let template = temp_env.load_template("simple2.j2")?;
    let context = Context::default();
    let result = template.render_as_string(context)?;
    assert_eq!(result, "Hello Rustaceans!".to_string());
    Ok(())
}
