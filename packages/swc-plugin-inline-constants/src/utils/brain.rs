use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, Value>> = Mutex::new(HashMap::new());
}

fn instance() -> MutexGuard<'static, HashMap<String, Value>> {
    CACHE.lock().unwrap()
}

pub fn insert_and_return(key: &str, inserter: &dyn Fn(&str) -> Option<Value>) -> Value {
    let mut __instance = instance();

    let already_cached = __instance.contains_key(key);

    if !already_cached {
        let value_to_store: Option<Value> = inserter(key);

        if value_to_store.is_some() {
            __instance
                .insert(key.to_string(), value_to_store.unwrap())
                .expect("");
        }
    }

    let result = __instance.get(key).expect("invalid constant").clone();

    result
}
