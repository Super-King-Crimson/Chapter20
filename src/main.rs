mod app;
mod http;

const RUST_PORT: &str = "127.0.0.1:7878";

fn main() {
    //html time
    app::listen(RUST_PORT);

    println!("{RUST_PORT}");
}
