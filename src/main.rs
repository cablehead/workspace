use futures_util::{FutureExt, StreamExt};

use tokio::io::AsyncWriteExt;

use clap::Parser;
use serde::{Deserialize, Serialize};

use warp::Filter;

/*
 * todo:
 * - rework HTTP to take uri + headers as an optional command line arg
 *  - and optionally as fd:4
 *  - then the request body can be streamed to stdin
 * - respond with status code and headers via fd:5?
 *  - then stdout can stream to the response body
 */

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(value_parser)]
    command: String,
    #[clap(value_parser)]
    args: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    serve(args.command, args.args, 3030).await
}

async fn serve(command: String, args: Vec<String>, port: u16) {
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

    fn with_command(
        command: String,
    ) -> impl Filter<Extract = (String,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || command.clone())
    }

    fn with_args(
        args: Vec<String>,
    ) -> impl Filter<Extract = (Vec<String>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || args.clone())
    }

    let http = warp::method()
        .and(warp::path::full())
        .and(warp::header::headers_cloned())
        .and(warp::body::bytes())
        .and(with_command(command))
        .and(with_args(args))
        .and_then(http);

    warp::serve(ws.or(http)).run(([127, 0, 0, 1], port)).await;
}

pub async fn http(
    method: http::method::Method,
    path: warp::filters::path::FullPath,
    headers: http::header::HeaderMap,
    body: warp::hyper::body::Bytes,
    command: String,
    args: Vec<String>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    #[derive(Serialize, Deserialize)]
    struct Request {
        #[serde(with = "http_serde::method")]
        method: http::method::Method,
        #[serde(with = "http_serde::header_map")]
        headers: http::header::HeaderMap,
        #[serde(with = "http_serde::uri")]
        path: http::Uri,
        body: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        status: Option<u16>,
        headers: Option<std::collections::HashMap<String, String>>,
        body: String,
    }

    let request = serde_json::json!(Request {
        method: method,
        path: path.as_str().parse().unwrap(),
        headers: headers,
        body: String::from_utf8(body.to_vec()).unwrap(),
    });

    let res = process(command, args, request.to_string().as_bytes()).await;
    let res: Response = serde_json::from_slice(&res.stdout).unwrap();

    let mut builder = http::Response::builder()
        .status(res.status.unwrap_or(200))
        .header("Content-Type", "text/html; charset=utf8");

    if let Some(src_headers) = res.headers {
        let headers = builder.headers_mut().unwrap();
        for (key, value) in src_headers.iter() {
            headers.insert(
                http::header::HeaderName::try_from(key.clone()).unwrap(),
                http::header::HeaderValue::try_from(value.clone()).unwrap(),
            );
        }
    }

    Ok(builder.body(res.body).unwrap())
}

async fn process(command: String, args: Vec<String>, i: &[u8]) -> std::process::Output {
    let mut p = tokio::process::Command::new(command)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    {
        let mut stdin = p.stdin.take().unwrap();
        stdin.write_all(i).await.unwrap();
    }

    let res = p.wait_with_output().await.expect("todo");
    assert_eq!(res.status.code().unwrap(), 0);
    res
}

#[tokio::test]
async fn test_process() {
    assert_eq!(
        process("cat".to_string(), vec![], b"foo").await.stdout,
        b"foo"
    );
}

#[tokio::test]
async fn test_serve_defaults() {
    tokio::spawn(serve(
        "echo".to_string(),
        vec![r#"{"body": "hai"}"#.to_string()],
        3030,
    ));
    // give the server a chance to start
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    let resp = reqwest::get("http://127.0.0.1:3030/").await.unwrap();

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/html; charset=utf8",
    );
    assert_eq!(resp.text().await.unwrap(), "hai");
}

#[tokio::test]
async fn test_serve_override() {
    tokio::spawn(serve(
        "echo".to_string(),
        vec![r#"{
            "body": "sorry",
            "status": 404,
            "headers": {
                "content-type": "text/plain"
            }
        }"#
        .to_string()],
        3031,
    ));
    // give the server a chance to start
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    let resp = reqwest::get("http://127.0.0.1:3031/").await.unwrap();

    assert_eq!(resp.status(), 404);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/plain");
    assert_eq!(resp.text().await.unwrap(), "sorry");
}
