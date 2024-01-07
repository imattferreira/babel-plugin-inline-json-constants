
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, serde_json::Value>> = Mutex::new(HashMap::new());
}

pub fn instance() -> MutexGuard<'static, HashMap<String, serde_json::Value>> {
    CACHE.lock().unwrap()
}
