use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use serde_json::json;

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
            let packet = json!({
                "method": req.method().as_str(),
                // "headers": req.headers(),
                // "remote_addr": req.remote_addr(),
                "uri": req.uri().path(),
                // "body": req.body().to_bytes(),
            });

            println!("{}", packet);
    Ok(Response::new(Body::from("Hello World!\n")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
