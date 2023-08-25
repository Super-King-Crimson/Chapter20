use std::{
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs}, path::Path, thread, time::Duration,
};

use crate::thread_pool::ThreadPool;

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

pub fn listen(addr: impl Display + ToSocketAddrs, mut pool: ThreadPool) {
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on {}", addr);

    //alright now let's build our thread pool
    for stream in listener.incoming() {
        pool.enter(|| {
            handle_connection(stream.unwrap()); 
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    
    let req_header = reader.lines().next().unwrap().unwrap();

    let uri = HttpRequest::parse(&req_header).unwrap().uri;

    if uri.contains("favicon") {
        return;
    }

    let mut status = HTTP_OK;

    let html = uri_to_path(uri)
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
    let mut path = format!("./routes{uri}");

    match path.as_ref() {
        "./routes/" => Ok(path + "init.html"),
        "./routes/slow" => {
            thread::sleep(Duration::from_secs(5));
            Ok(path + ".html")
        },
        "./routes/ultraslow" => {
            thread::sleep(Duration::from_secs(20));
            Ok(path + ".html")
        },
        _ => { 
            let metadata = fs::metadata(&path)
                .or_else(|_| {
                    path = format!("{path}.html");

                    fs::metadata(&path)
                })?;

            if metadata.is_dir() {
                path += "/init.html"; 
            };

            Ok(path)
        }
    }
}

fn format_response(status: &str, contents: &str) -> String {
    let len = contents.len();

    format!("HTTP/1.1{status}\r\nContent-Length: {len}\r\n\r\n{contents}").to_owned()
}
