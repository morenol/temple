use crate::value::{Value, ValuesMap};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Default)]
pub struct Context {
    global_scope: Arc<RwLock<ValuesMap>>,
    external_scope: ValuesMap,
}

impl Context {
    pub fn new(external_scope: ValuesMap) -> Self {
        Self {
            global_scope: Arc::new(RwLock::new(ValuesMap::default())),
            external_scope,
        }
    }
    pub fn find(&self, key: &str) -> Value {
        if let Some(value) = self.external_scope.get(key) {
            value.clone()
        } else if let Some(value) = self.global_scope.read().unwrap().get(key) {
            value.clone()
        } else {
            Value::Empty
        }
    }
    pub fn values(&self) -> ValuesMap {
        let mut value_map = ValuesMap::new();
        for (key, value) in &*self.global_scope.read().unwrap() {
            value_map.insert(key.to_string(), value.clone());
        }
        for (key, value) in &self.external_scope {
            value_map.insert(key.to_string(), value.clone());
        }
        value_map
    }
    pub fn set_global(&mut self, global_scope: Arc<RwLock<ValuesMap>>) {
        self.global_scope = global_scope;
    }
}
