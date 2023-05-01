use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use warp::{Filter};

use env_logger::Env;
use log::info;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct KeyValue {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Set up a shared state for storing key-value pairs
    let key_value_store = Arc::new(Mutex::new(HashMap::new()));

    // Define a GET route for retrieving a value by key
    let get_route = warp::get()
        .and(warp::path!("get" / String))
        .and(with_key_value_store(key_value_store.clone()))
        .and_then(handle_get);

    // Define a POST route for setting a value by key
    let post_route = warp::post()
        .and(warp::path!("post"))
        .and(warp::body::json())
        .and(with_key_value_store(key_value_store.clone()))
        .and_then(handle_post);

    // Combine the routes into a single filter
    let routes = get_route.or(post_route);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 80)).await;
}

// A filter that injects the key-value store into route handlers
fn with_key_value_store(
    store: Arc<Mutex<HashMap<String, String>>>,
) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, String>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}

// A route handler for GET requests
async fn handle_get(
    key: String,
    store: Arc<Mutex<HashMap<String, String>>>,
) -> Result<impl warp::Reply, warp::Rejection> {

    info!("A GET Request for key: {}", key);

    let store = store.lock().unwrap();

    if let Some(value) = store.get(&key) {
        Ok(warp::reply::json(&KeyValue {
            key: key.clone(),
            value: value.clone(),
        }))
    } else {
        Err(warp::reject::not_found())
    }
}

// A route handler for POST requests
async fn handle_post(
    new_key_value: KeyValue,
    store: Arc<Mutex<HashMap<String, String>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = store.lock().unwrap();
    let cloned_key_value = new_key_value.clone();
    store.insert(new_key_value.key, new_key_value.value);
    
    info!("Stored key-value pair: {:?}", cloned_key_value);
    Ok("Success")
}
