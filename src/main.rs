mod app;
mod http;
mod tap;
mod thread_pool;

const RUST_PORT: &str = "127.0.0.1:7878";

fn main() {
    thread_pool::explanation::read();

    //Right now our server is single threaded, meaning we can only process one request at a time
    //If we get multiple requests, or a long request, it might take a while before the server responds to a specific requests
    //Let's see the problem by simulating a slow request
    app::listen(RUST_PORT);
}
