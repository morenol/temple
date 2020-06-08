use crate::value::ValuesMap;
use std::sync::{Arc, RwLock};

pub struct Context {
    global_scope: Arc<RwLock<ValuesMap>>,
    external_scope: Arc<ValuesMap>,
}

impl Context {
    pub fn new(global_scope: Arc<RwLock<ValuesMap>>, external_scope: Arc<ValuesMap>) -> Self {
        Self {
            global_scope,
            external_scope,
        }
    }
    pub fn values(&self) -> ValuesMap {
        let mut value_map = ValuesMap::new();
        for (key, value) in &*self.global_scope.read().unwrap() {
            value_map.insert(key.to_string(), value.clone());
        }
        for (key, value) in &*self.external_scope {
            value_map.insert(key.to_string(), value.clone());
        }
        value_map
    }
}
