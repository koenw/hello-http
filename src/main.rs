use warp::Filter;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("HELLO_PORT")
        .unwrap_or("8000".to_string())
        .parse().expect("HELLO_PORT is set to an invalid port number");

    let routes = warp::any().map(|| "Hello, world!\n");

    println!("listening on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
