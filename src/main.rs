mod app;
mod http;
mod tap;

const RUST_PORT: &str = "127.0.0.1:7878";

fn main() {
    app::listen(RUST_PORT);
}
