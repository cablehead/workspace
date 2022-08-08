use futures_util::{FutureExt, StreamExt};

use tokio::io::AsyncWriteExt;

use warp::http::header::HeaderMap;
use warp::Filter;

#[tokio::main]
async fn main() {
    serve().await
}

async fn serve() {
    println!("1");
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
    println!("2");

    warp::serve(ws.or(http)).run(([127, 0, 0, 1], 3030)).await;
    println!("3");
}

pub async fn http(
    method: http::method::Method,
    path: warp::filters::path::FullPath,
    headers: HeaderMap,
    body: warp::hyper::body::Bytes,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    // next steps:
    // - flesh out packet
    // - write packet to child stdin
    // - read child stdout/err/status code
    // - decode read response
    // - construct http response

    println!("{:?}", headers);
    println!("{:?}", body);
    let packet = serde_json::json!({
        "method": method.as_str(),
        // "headers": headers,
        "path": path.as_str(),
        // "body": body,
    });

    let res = process("cat", packet.to_string().as_bytes()).await;
    Ok(http::Response::builder().status(200).body(bytes::Bytes::from(res)).unwrap())
}

async fn process(command: &str, i: &[u8]) -> Vec<u8> {
    let mut p = tokio::process::Command::new(command)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    {
        let mut stdin = p.stdin.take().unwrap();
        stdin.write_all(i).await.unwrap();
    }

    let res = p.wait_with_output().await.expect("todo");
    // todo
    assert_eq!(res.status.code().unwrap(), 0);
    res.stdout
}

#[tokio::test]
async fn test_process() {
    assert_eq!(process("cat", b"foo").await, b"foo");
}

#[tokio::test]
async fn test_serve() {
    tokio::spawn(serve());
    // give the server a chance to start
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    let resp = reqwest::get("http://127.0.0.1:3030/")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("resp: {}", resp);
}
