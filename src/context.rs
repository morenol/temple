use crate::value::{Value, ValuesMap};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Default)]
pub struct Context {
    global_scope: Arc<RwLock<ValuesMap>>,
    external_scope: ValuesMap,
    scopes: Vec<Arc<RwLock<ValuesMap>>>,
}

impl Context {
    pub fn new(external_scope: ValuesMap) -> Self {
        Self {
            global_scope: Arc::new(RwLock::new(ValuesMap::default())),
            external_scope,
            scopes: vec![],
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
    pub fn find(&self, key: &str) -> Value {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.read().unwrap().get(key) {
                return value.clone();
            }
        }
        if let Some(value) = self.external_scope.get(key) {
            value.clone()
        } else if let Some(value) = self.global_scope.read().unwrap().get(key) {
            value.clone()
        } else {
            Value::Empty
        }
    }
    pub fn set_global(&mut self, global_scope: Arc<RwLock<ValuesMap>>) {
        self.global_scope = global_scope;
    }
}
