pub mod requests {
    use std::{io::{BufRead, BufReader}, fs::OpenOptions};

    pub fn explain(verbose: bool) {
        println!("The moment we've all been waiting for... networking!");

        //A HTTP request takes this format:
        /* 
            Method Request-URI HTTP-Version CRLF
            headers CRLF
            message-body
        */

        //the first line is the REQUEST LINE: holds data about what the client is requesting
            //Method: HTTP verb (GET, POST, etc) that describes how the client is making the request
            //URI (Uniform Resource Idenifier), basically the same as a URL (Uniform Resource Locator)
                //where are you getting your data from
            //HTTP Version (simply the HTTP version the client uses)
            //CRLF - carriage return line feed (\r\n), basically means go to new line
                //separates each line of data: in this case, separates the request line from the rest of the eq
            
        //After the request line, everything's a header, and then that's it (GET requests have no body)

        if verbose {
            show_full_http_req();
        }
    }

    fn show_full_http_req() {
        let file = OpenOptions::new()
            .read(true)
            .open("./src/txt/http_request.txt")
            .unwrap();

        println!("Here's what our HTTP request looked like (remember, GET requests have no header)\n");
        BufReader::new(file)
            .lines()
            .for_each(|line| println!("{}", line.unwrap()));
    }
}
