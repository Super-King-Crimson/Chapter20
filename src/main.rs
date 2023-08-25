mod app;
mod http;
mod tap;
mod thread_pool;

use thread_pool::ThreadPool;

const RUST_PORT: &str = "127.0.0.1:7878";

fn main() {
    //In line with compiler driven development, let's make our functions
    let pool = ThreadPool::new(4);

    app::listen(RUST_PORT, pool);
}
