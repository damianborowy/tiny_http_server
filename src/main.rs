use std::env;
use std::fs::File;
use tiny_http::{Server, Response, Request, Method};

fn prepare_response(request: &Request) -> Response {
    if request.method() == &Method::Get {
        if request.url() == "/" {
            return Response::from_file(File::open("index.html").unwrap());
        }
    }

    if request.method() == &Method::Post {
        if request.url() == "/echo" {
            return Response::from_string("echoed");
        }

        return Response::from_string("not implemented");
    }

    return Response::from_string("not implemented");
}

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(..) => "5000".to_string(),
    };

    let server = Server::http(["0.0.0.0", &port].join(":")).unwrap();

    for request in server.incoming_requests() {
        let response = prepare_response(&request);
        let _ = request.respond(response);
    }
}
