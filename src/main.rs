use std::env;
use std::fs::File;
use tiny_http::{Server, Response, Request, Method};

fn prepare_response(request: &Request) -> Response<File> {
    if request.method() == &Method::Get {
        if request.url() == "/" {
            return Response::from_file(File::open("index.html").unwrap());
        }
    }

    if request.method() == &Method::Post {
        return Response::from_file(File::open("index.html").unwrap());
    }

    return Response::from_file(File::open("index.html").unwrap());
}

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(..) => "8000".to_string(),
    };

    let server = Server::http(["0.0.0.0", &port].join(":")).unwrap();

    for request in server.incoming_requests() {
        let response = prepare_response(&request);
        let _ = request.respond(response);
    }
}

