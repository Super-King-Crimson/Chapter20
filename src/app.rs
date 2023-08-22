use std::{
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs}, path::Path,
};

pub const HTTP_OK: &str = "200 OK";
pub const HTTP_NOT_FOUND: &str = "404 NOT FOUND";

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn parse(str: &str) -> Option<HttpMethod> {
        match str.to_ascii_uppercase().as_ref() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            _ => None,
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
}

impl HttpRequest {
    pub fn parse(request: &str) -> Option<HttpRequest> {
        let req_line: Vec<&str> = request.split(char::is_whitespace).collect();

        if req_line.len() != 3 {
            None
        } else {
            let method = HttpMethod::parse(req_line[0])?;
            let uri = req_line[1].to_string();
            let version = req_line[2].to_string();

            Some(HttpRequest { method, uri, version })
        }
    }
}

pub fn listen(addr: impl Display + ToSocketAddrs) {
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on {}", addr);

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);

    //we've been ignoring the request and just sending data no matter what, let's fix that
    //if the user asks for a url we don't have, we'll return 404 not found
    let req_header = reader.lines().next().unwrap().unwrap();

    let uri = HttpRequest::parse(&req_header).unwrap().uri;

    let mut status = HTTP_OK;

    let file = match uri.as_ref() {
        "/" => String::from("./home.html"),
        path => {
            let path = format!("{}.html", &path[1..]);

            if Path::new(&path).exists() {
                path
            } else {
                status = HTTP_NOT_FOUND;
                String::from("./404.html")
            }
        }
    };

    let html = fs::read_to_string(file).unwrap();

    let res = format_response(status, &html);

    stream.write_all(res.as_bytes()).unwrap();
}

fn format_response(status: &str, contents: &str) -> String {
    let len = contents.len();

    let response =
        format!("HTTP/1.1{status}\r\nContent-Length: {len}\r\n\r\n{contents}").to_owned();
    response
}
