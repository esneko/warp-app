#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
  // GET /
  let root = warp::path::end().map(|| "Root");

  // GET /hello
  let hello = warp::path("hello").map(|| "Hello");

  // GET /echo/:string
  let echo = warp::path("echo")
    .and(warp::path::param())
    .map(|name: String| format!("Echo: {}", name));

  let routes = warp::get().and(
    root
      .or(hello)
      .or(echo),
  );

  warp::serve(routes)
    .run(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
    .await;
}
