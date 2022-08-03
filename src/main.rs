use std::io::Write;
use std::net;
use std::process;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    #[clap(value_parser)]
    command: String,
    #[clap(value_parser)]
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let sock = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)), 80);
    let server = tiny_http::Server::http(sock).unwrap();
    for mut req in server.incoming_requests() {
        let mut buffer = String::new();
        req.as_reader().read_to_string(&mut buffer).unwrap();
        let b64 = base64::encode_config(buffer, base64::URL_SAFE);

        // gosh, this is terrible. I need to get better with rust's type system
        let headers: Vec<(String, String)> = req
            .headers()
            .iter()
            .map(|x| (format!("{}", x.field.as_str()), format!("{}", x.value)))
            .collect();

        let packet = serde_json::json!({
                "method": req.method().as_str(),
                "headers": headers,
                "remote_addr": req.remote_addr(),
                "url": req.url(),
                "body": b64,
        });

        let mut p = process::Command::new(&args.command)
            .args(&args.args)
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()?;
        let mut stdin = p.stdin.take().unwrap();
        stdin.write_all(packet.to_string().as_bytes())?;
    }

    Ok(())
}
