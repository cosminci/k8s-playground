use warp::Filter;

#[tokio::main]
async fn main() {
    let hello =
        warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let health =
        warp::path("health").map(|| format!("Ok"));
    let api = hello.or(health);

    warp::serve(api)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
