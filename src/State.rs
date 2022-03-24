extern crate nickel;
use nickel::{Request};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    pub store: Arc<RwLock<HashMap<String, String>>>,
}

impl State {
    pub fn new() -> State {
        State {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn add(&self, req: &mut Request) -> String {
        let key = req.param("key").unwrap_or("blank");
        let value = req.param("value").unwrap_or("blank");
        let mut store = self.store.write().unwrap();
        store.insert(key.to_string(), value.to_string());
        format!("OK \nKey: {} \nValue: {}", key, value)
    }
    pub fn get(&self, req: &mut Request) -> String {
        let key = req.param("key").unwrap_or("blank");
        let missing = String::from("MISSING");
        let store = self.store.try_read().unwrap();
        let value = store.get(&key.to_string()).unwrap_or(&missing);
        format!("OK \nKey: {} \nValue: {}", key, value)
    }
    pub fn delete(&self, req: &mut Request) -> String {
        let key = req.param("key").unwrap_or("blank");
        let missing = String::from("MISSING");
        let mut store = self.store.write().unwrap();
        store.remove(&key.to_string());
        let value = store.get(&key.to_string()).unwrap_or(&missing);
        format!("OK \nKey: {} \nValue: {}", key, value)
    }
    // fn update() {}
    pub fn purge(&self) -> String {
        let mut store = self.store.write().unwrap();
        store.clear();
        "OK".to_string()
    }
    pub fn contents(&self) -> String {
        let store = self.store.try_read().unwrap();
        let mut res = String::new();
        for (key, val) in store.iter() {
            res = format!("{}\nKey: {}\nValue: {}", &res, &key, &val)
        }
        res
    }
}
