#[macro_use]
extern crate nickel;
use nickel::{HttpRouter, Nickel, Request};
use rustedis::state::State;

fn main() {
    // let store = Arc::new(RwLock::new(HashMap::new()));
    let store = State::new();
    let mut app = Nickel::new();
    app.utilize(router(store));
    
    app.listen("127.0.0.1:3000").unwrap();
}

fn hello(req: &mut Request) -> String {
    let name = req.param("name").unwrap_or("world");
    format!("Hello, {}!", name)
}

fn router(store: State) -> nickel::Router {
    let mut router = nickel::Router::new();
    router.get("/", middleware! {|req| hello(req)});
    let store2 = store.clone();
    router.get(
        "/get/:key",
        middleware! {|req| {
            store2.get(req)
        }},
    );
    let store3 = store.clone();
    router.get(
        "/add/:key/:value",
        middleware! {|req| {
            store3.add(req)
        }},
    );
    let store4 = store.clone();
    router.get(
        "/delete/:key",
        middleware! {|req| {
            store4.delete(req)
        }},
    );
    let store5 = store.clone();
    router.get(
        "/purge",
        middleware! {|_| {
            store5.purge()
        }},
    );
    let store6 = store.clone();
    router.get(
        "/contents",
        middleware! {|_| {
            store6.contents()
        }},
    );
    router.get(
        "/update/:key/:value",
        middleware! {|req| {
            store.add(req)
        }},
    );
    router
}