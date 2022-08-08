use futures_util::{FutureExt, StreamExt};

// use serde::{Deserialize, Serialize};
//
use warp::http::method::Method;
use warp::filters::path::FullPath;
use warp::http::header::HeaderMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    let ws = warp::ws()
        .and(warp::path::full())
        .and(warp::header::headers_cloned())
        .map(|ws: warp::ws::Ws, path, headers| {
            ws.on_upgrade(move |websocket| {
                println!("{:?}", path);
                println!("{:?}", headers);
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let http = warp::method()
        .and(warp::path::full())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and_then(http);

    warp::serve(ws.or(http)).run(([127, 0, 0, 1], 3030)).await;
}

/*
pub async fn http() -> Result<impl warp::Reply, Infallible> {
    let todos = db.lock().await;
    let todos: Vec<Todo> = todos
        .clone()
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .collect();
    Ok(warp::reply::json(&todos))
}
*/

pub async fn http(
    method: Method,
    path: FullPath,
    headers: HeaderMap,
    body: warp::hyper::body::Bytes,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    // next steps:
    // - flesh out packet
    // - write packet to child stdin
    // - read child stdout/err/status code
    // - decode read response
    // - construct http response
    let packet = serde_json::json!({
        // "method": method,
        // "headers": headers,
        // "url": path,
        // "body": body,
    });
    let mut child = tokio::process::Command::new("echo")
        .arg("hello")
        .arg("world")
        .spawn()
        .expect("failed to spawn");

    // Await until the command completes
    let status = child.wait().await.expect("todo");
    println!("the command exited with: {}", status);

    println!("{:?}", method);
    println!("{:?}", path);
    println!("{:?}", headers);
    println!("{:?}", body);
    Ok(warp::reply())
}
