use warp::{hyper::Method, Filter};
use crate::storage::Storage;
use tokio::sync::watch;


pub async fn listen(tx: watch::Sender<()>) {

    let storage = Storage::new().expect("Failed to initialize storage");

    let payload = warp::post()
        .and(warp::body::json())
        .map({
            let storage = storage.clone();
            let tx = tx.clone();
            move |body: serde_json::Value| {
            println!("Received payload: {}", body); // Log the received payload
            // Store the payload
            if let Err(e) = storage.add_json(body.clone()) {
                eprintln!("Failed to store payload: {}", e);
            }
            let _ = tx.send(());
            format!("hello {}!", body) // Return the formatted string
            }
        });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::POST, Method::OPTIONS])
        .allow_headers(vec!["Content-Type", "Authorization", "Accept", "Origin", "X-Requested-With"])
        .max_age(3600);

    let routes = payload.with(cors);

    println!("Server started at http://127.0.0.1:53821");

    warp::serve(routes).run(([127, 0, 0, 1], 53821)).await;
}
