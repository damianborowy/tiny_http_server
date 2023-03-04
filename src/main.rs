use json::object;
use std::env;
use std::fs::File;
use std::process::{Command, Output};
use tiny_http::{Server, Response, Method};

fn command_output_to_json(output: Output, success_text: Option<&str>) -> Vec<u8> {
    let result = match output.stderr.len() + output.stdout.len() {
        0 => success_text.unwrap_or(""),
        _ => &std::str::from_utf8(&output.stdout).unwrap(),
    };

    let data = object!{
        result: result,
        error: String::from_utf8(output.stderr.clone()).unwrap(),
    };

    return data.dump().as_bytes().to_vec();
}

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

            if request.url() == "/echo" {
                let output = Command::new("sh").arg("-c").arg("mkdir test").output().expect("");
                
                let _ = request.respond(Response::from_data(command_output_to_json(output, Some("success!"))));
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
