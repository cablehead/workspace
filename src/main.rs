#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let index = warp::method()
        .and(warp::path::full())
        .and(warp::header::headers_cloned())
        .map(|method, path, headers| {
            println!("{:?}", method);
            println!("{:?}", path);
            println!("{:?}", headers);
            "hello"
        });
    warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}
