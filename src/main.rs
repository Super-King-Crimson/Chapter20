use std::thread;

mod app;
mod http;

const RUST_PORT: &str = "127.0.0.1:7878";

fn main() {
    let app_thread = thread::spawn(|| {
        app::listen(RUST_PORT);
    });
    
    http::requests::explain(true);

    app_thread.join().unwrap();
    //Now we don't have to print anything, as the fact that the browser isn't showing any errors means
        //we're good!
    //now we just have to return proper html...
}
