use std::{
    net::{TcpListener, ToSocketAddrs, TcpStream}, 
    fmt::Display, io::{BufReader, BufRead}};

pub fn listen(addr: impl Display + ToSocketAddrs) {
    //unwraps all the way down, this is just a test

    //bind can fail: 
        //connecting to port 80 requires admin priviliges
        //nonadministrators can only listen on ports higher than 1023
        //TCP also can only have one program listening to a port at a time
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on {}", addr);

    //a stream represents an open connection btwn client and server (incoming returns a TcpStream)
        //A connection is a full request and response process:
            //client connects to server,
            //server generates response
            //server closes connection
    for stream in listener.incoming() {
        //we're iterating over connection attempts, which may fail for any number of reasons
        //some are OS specific: only x number of connections open at a time
        handle_connection(stream.unwrap());
    }
    //here stream is dropped and the connection is closed
    //just like files are closed when they go out of scope, 
        //a stream's connection is closed when it goes out of scope
}

fn handle_connection(mut stream: TcpStream) {
    //hey, BufReaders can read streams too!
    let reader = BufReader::new(&mut stream);

    //Remember to import the BufRead trait so you get access to the lines() method
        //of course, this is reading as bytes,
        //so it'll fail if the line isn't utf8
    let req: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    //A browser signals the end of an HTTP request by sending two newline characters in a row,
        //so we take lines until we get to a line that contains an empty string
    
    /* 
    HTTP REQUEST HEADER OR SMTHN\n
    data\n
    more data \n
    \n (we're done)
    */

    //pretty print the payload
    println!("Request: {:#?}", req);
}
