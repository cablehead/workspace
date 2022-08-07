#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::path::end()
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .map(|h| {
            println!("{:?}", h);
            "hello"
        });
    warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}
