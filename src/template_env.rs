use crate::value::{Value, ValuesMap};

#[derive(Clone, Debug, PartialEq)]
enum Jinja2CompatMode {
    None,
    Version2_10, // Fix in jinja2cpp
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

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateEnv {
    settings: Settings,
    global_values: ValuesMap,
}

impl TemplateEnv {
    pub fn add_global(&mut self, _name: String, _val: Value) {
        todo!()
    }

    pub fn remove_global(&mut self, _name: String, _val: Value) {
        todo!()
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
}

impl Default for TemplateEnv {
    fn default() -> TemplateEnv {
        TemplateEnv {
            settings: Settings::default(),
            global_values: ValuesMap::default(),
        }
    }
}
