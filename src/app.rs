use std::{
    net::{TcpListener, ToSocketAddrs, TcpStream}, 
    fmt::Display, io::{BufReader, BufRead, Write}};

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

    //let's send back a Success instead of just leaving the req out to die
    let response = b"HTTP/1.1 200 OK \r\n\r\n";

    //now we don't see any errors!

    stream.write_all(response).unwrap();
}
