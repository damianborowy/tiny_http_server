use std::env;
use std::fs::File;
use tiny_http::{Server, Response, Method};

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(..) => "5000".to_string(),
    };

    let server = Server::http(["0.0.0.0", &port].join(":")).unwrap();

    for request in server.incoming_requests() {
        if request.method() == &Method::Get {
            if request.url() == "/" {
                let _ = request.respond(Response::from_file(File::open("index.html").unwrap()));
                continue;
            }
        }

        if request.method() == &Method::Post {
            if request.url() == "/echo" {
                let _ = request.respond(Response::from_string("echoed"));
                continue;
            }
        }
    }
}
