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
    let store_get = store.clone();
    let store_add = store.clone();
    let store_delete = store.clone();
    let store_purge = store.clone();
    let store_contents = store.clone();
    router.get("/get/:key", middleware! {|req| store_get.get(req)});
    router.get("/add/:key/:value", middleware! {|req| store_add.add(req)});
    router.get("/delete/:key", middleware! {|req| store_delete.delete(req)});
    router.get("/purge", middleware! {|_| store_purge.purge()});
    router.get("/contents", middleware! {|_| store_contents.contents()});
    router.get("/update/:key/:value", middleware! {|req|store.add(req)});
    router
}
