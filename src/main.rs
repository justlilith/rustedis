#[macro_use]
extern crate nickel;
use nickel::{HttpRouter, Nickel, Request};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

struct State {
    pub store: RwLock<HashMap<String, String>>,
}

impl State {
    pub fn new() -> State {
        State {
            store: RwLock::new(HashMap::new()),
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
    fn delete(&self, req: &mut Request) -> String {
        let key = req.param("key").unwrap_or("blank");
        let missing = String::from("MISSING");
        let mut store = self.store.write().unwrap();
        store.remove(&key.to_string());
        let value = store.get(&key.to_string()).unwrap_or(&missing);
        format!("OK \nKey: {} \nValue: {}", key, value)
    }
    // fn update() {}
    fn purge(&self) -> String {
        let mut store = self.store.write().unwrap();
        store.clear();
        "OK".to_string()
    }
    // fn contents() {}
}

fn main() {
    // let store = Arc::new(RwLock::new(HashMap::new()));
    let store = Arc::new(State::new());
    let mut app = Nickel::new();
    app.get("/", middleware! {|req| hello(req)});
    let store2 = store.clone();
    app.get(
        "/get/:key",
        middleware! {|req| {
            store2.get(req)
        }},
    );
    let store3 = store.clone();
    app.get(
        "/add/:key/:value",
        middleware! {|req| {
            store3.add(req)
        }},
    );
    let store4 = store.clone();
    app.get(
        "/delete/:key",
        middleware! {|req| {
            store4.delete(req)
        }},
    );
    let store5 = store.clone();
    app.get(
        "/purge",
        middleware! {|_| {
            store5.purge()
        }},
    );
    app.get(
        "/update/:key/:value",
        middleware! {|req| {
            store.add(req)
        }},
    );
    app.listen("127.0.0.1:3000").unwrap();
}

fn hello(req: &mut Request) -> String {
    let name = req.param("name").unwrap_or("world");
    format!("Hello, {}!", name)
}