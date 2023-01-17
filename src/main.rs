#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
  // GET /
  let root = warp::path::end().map(|| "Hello root");

  // GET /hello
  let hello = warp::path("hello").map(|| "Hello world");

  // GET /bye/:string
  let bye = warp::path("bye")
    .and(warp::path::param())
    .map(|name: String| format!("Bye {}", name));

  let routes = warp::get().and(
    root
      .or(hello)
      .or(bye),
  );

  warp::serve(routes)
    .run(([0, 0, 0, 0, 0, 0, 0, 0], 8080))
    .await;
}
