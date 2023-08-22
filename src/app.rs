use std::{
    net::{TcpListener, ToSocketAddrs, TcpStream}, 
    fmt::Display, io::{BufReader, BufRead, Write}, fs};

pub const HTTP_OK: &str = "200 OK";

pub fn listen(addr: impl Display + ToSocketAddrs) {
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on {}", addr);

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    } 
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);

    let _req: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let html = fs::read_to_string("./test.html").unwrap();

    //now let's send some data
    let res = format_response(HTTP_OK, &html);
    stream.write_all(res.as_bytes()).unwrap();
}

fn format_response(status: &str, contents: &str) -> String {
    let len = contents.len();

    //for some reaosn we need content length
    //after the CRLF we put our data

    //we also have a content length header which is the length of our data,
    //this is required to be a valid HTTP response
    let response = format!("HTTP/1.1{status}\r\nContent-Length: {len}\r\n\r\n{contents}").to_owned();
    response
}
