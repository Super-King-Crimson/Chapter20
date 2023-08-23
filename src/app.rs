use std::{
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs}, path::Path,
};

pub const HTTP_OK: &str = "200 OK";
pub const HTTP_NOT_FOUND: &str = "404 NOT FOUND";

pub const ERR_PAGE: &str = "./routes/404.html";

#[derive(Debug)]
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
#[derive(Debug)]
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

    let path = uri_to_path(uri);

    let html = path
        .and_then(|path| {
            if Path::new(&path).extension().unwrap().ne("html") {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Files must be html"))
            } else {
                fs::read_to_string(path)
            }
        })
        .unwrap_or_else(|_| {
            status = HTTP_NOT_FOUND;
            fs::read_to_string(ERR_PAGE.to_string()).unwrap()
        });

    let res = format_response(status, &html);

    stream.write_all(res.as_bytes()).unwrap();
}

fn uri_to_path(uri: String) -> Result<String, std::io::Error> {
    let path = format!("./routes{uri}");

    let md = fs::metadata(&path)?;

    if path.eq("./routes/") {
        Ok(path + "init.html")
    } else {
        Ok(if md.is_dir() { path + "/init.html" } else { path })
    }
}

fn format_response(status: &str, contents: &str) -> String {
    let len = contents.len();

    format!("HTTP/1.1{status}\r\nContent-Length: {len}\r\n\r\n{contents}").to_owned()
}
