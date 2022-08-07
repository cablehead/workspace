use futures_util::{FutureExt, StreamExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    let ws = warp::ws().map(|ws: warp::ws::Ws| {
        // And then our closure will be called when it completes...
        ws.on_upgrade(|websocket| {
            // Just echo all messages back...
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
        .map(|method, path, headers, body| {
            println!("{:?}", method);
            println!("{:?}", path);
            println!("{:?}", headers);
            println!("{:?}", body);
            "hello"
        });

    warp::serve(ws.or(http)).run(([127, 0, 0, 1], 3030)).await;
}
