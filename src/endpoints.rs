use std::collections::hash_map;

let mut store = HashMap::new();



async fn add(req: Request<()>) -> tide::Result {
    let key = req.param("key").unwrap_or("blank");
    let value = req.param("value").unwrap_or("blank");
    store.add(key, value)
    Ok(format!("{} Added to key {}", key, value))
}