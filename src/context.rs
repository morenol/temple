use crate::error::{Error, ErrorKind, Result};
use crate::source::SourceLocationInfo;
use crate::value::{Value, ValuesMap};
use crate::TemplateEnv;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Context<'a> {
    global_scope: Arc<RwLock<ValuesMap>>,
    external_scope: ValuesMap,
    scopes: Vec<Arc<RwLock<ValuesMap>>>,
    callback_renderer: Arc<&'a TemplateEnv<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(external_scope: ValuesMap, callback_renderer: Arc<&'a TemplateEnv>) -> Self {
        Self {
            global_scope: Arc::new(RwLock::new(ValuesMap::default())),
            external_scope,
            scopes: vec![],
            callback_renderer,
        }
    }
    pub fn enter_scope(&mut self) -> Arc<RwLock<ValuesMap>> {
        let scope = Arc::new(RwLock::new(ValuesMap::default()));
        self.scopes.push(scope.clone());
        scope
    }
    pub fn exit_scope(&mut self) -> Option<&Arc<RwLock<ValuesMap>>> {
        self.scopes.pop();
        self.scopes.last()
    }
    pub fn find(&self, key: &str) -> Result<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.read().unwrap().get(key) {
                return Ok(value.clone());
            }
        }
        if let Some(value) = self.external_scope.get(key) {
            Ok(value.clone())
        } else if let Some(value) = self.global_scope.read().unwrap().get(key) {
            Ok(value.clone())
        } else {
            Err(Error::from(ErrorKind::UndefinedValue(
                key.to_string(),
                SourceLocationInfo::new(0, 0),
            )))
        }
    }
    pub fn set_global(&mut self, global_scope: Arc<RwLock<ValuesMap>>) {
        self.global_scope = global_scope;
    }
    pub fn get_renderer_callback(&self) -> Arc<&'a TemplateEnv<'a>> {
        self.callback_renderer.clone()
    }
}
