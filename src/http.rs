pub mod requests {
    use std::{io::{BufRead, BufReader}, fs::OpenOptions};

    pub fn _explain(verbose: bool) {
        println!("The moment we've all been waiting for... networking!");

        // A HTTP request takes this format:
        /* 
            Method Request-URI HTTP-Version CRLF
            headers CRLF
            message-body
        */

        if verbose {
            _show_full_http_req();
        }

        writing::_explain();
    }

    pub mod writing {
        pub fn _explain() {
            println!("An HTTP response request looks like this");

            /* 
                HTTP-Version Status-Code Reason-Phrase CRLF
                headers CRLF
                message-body
            */ 
        }
    }

    fn _show_full_http_req() {
        let file = OpenOptions::new()
            .read(true)
            .open("./txt/http_request.txt")
            .unwrap();

        println!("Here's what our HTTP request looked like (GET requests have no header)\n");
        BufReader::new(file)
            .lines()
            .for_each(|line| println!("{}", line.unwrap()));
    }
}
