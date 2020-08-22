use crate::error::{Error, ErrorKind, Result};
use crate::value::{Value, ValuesMap};
use crate::FileSystemHandler;
use crate::Template;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, PartialEq)]
enum Jinja2CompatMode {
    None,
    // Version2_10, // Fix in jinja2cpp
}
impl Default for Jinja2CompatMode {
    fn default() -> Jinja2CompatMode {
        Jinja2CompatMode::None
    }
}

/// Global template environment settings
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// Enables use of line statements (yet not supported)
    pub use_line_statements: bool,
    /// Enables blocks trimming the same way as it does python Jinja2 engine
    pub trim_blocks: bool,
    /// Enables blocks stripping (from the left) the same way as it does python Jinja2 engine
    pub lstrip_blocks: bool,
    /// Templates cache size
    pub cache_size: usize,
    /// If auto_reload is set to true (default) every time a template is requested the loader checks if the source changed and if yes, it will reload the template
    pub auto_reload: bool,
    /// Extensions set enabled for templates
    extensions: Extensions,
    /// Controls Jinja2 compatibility mode
    jinja_compat_mode: Jinja2CompatMode,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            use_line_statements: false,
            trim_blocks: false,
            lstrip_blocks: false,
            cache_size: 400,
            auto_reload: true,
            extensions: Extensions::default(),
            jinja_compat_mode: Jinja2CompatMode::default(),
        }
    }
}

/// Extensions set which should be supported
#[derive(Clone, Debug, PartialEq)]
struct Extensions {
    /// Enable use of `do` statement
    do_ext: bool,
}

impl Default for Extensions {
    fn default() -> Extensions {
        Extensions { do_ext: false }
    }
}

pub struct TemplateEnv<'a> {
    settings: Settings,
    global_values: Arc<RwLock<ValuesMap>>,
    filesystem_handlers: Vec<Box<dyn FileSystemHandler + 'a>>,
}

impl<'a> TemplateEnv<'a> {
    pub fn add_global<V>(&mut self, name: String, val: V)
    where
        V: Into<Value>,
    {
        self.global_values.write().unwrap().insert(name, val.into());
    }

    pub fn remove_global(&mut self, name: String) {
        self.global_values.write().unwrap().remove(&name);
    }
    pub fn globals(&self) -> Arc<RwLock<ValuesMap>> {
        self.global_values.clone()
    }

    pub fn set_settings(&mut self, settings: Settings) {
        self.settings = settings;
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
    pub fn add_filesystem_handler(
        &mut self,
        handler: Box<dyn FileSystemHandler + 'a>,
    ) -> Result<()> {
        self.filesystem_handlers.push(handler);
        Ok(())
    }
    pub fn load_template(&self, filename: &str) -> Result<Template> {
        let mut template = Template::new(Arc::new(self))?;
        let mut not_found = true;
        for handler in &self.filesystem_handlers {
            let stream = handler.open_stream(filename);
            let mut content = String::default();

            if let Some(mut reader) = stream {
                reader.read_to_string(&mut content)?;
                template.load(content)?;
                not_found = false;
                break;
            }
        }
        if not_found {
            Err(Error::from(ErrorKind::TemplateNotFound))
        } else {
            Ok(template)
        }
    }
}

impl<'a> Default for TemplateEnv<'a> {
    fn default() -> TemplateEnv<'a> {
        TemplateEnv {
            settings: Settings::default(),
            global_values: Arc::new(RwLock::new(ValuesMap::default())),
            filesystem_handlers: vec![],
        }
    }
}
