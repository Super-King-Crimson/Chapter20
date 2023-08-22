mod app;

//127.0.0.1 is our home IP address 
//7878 = RUST, that's our port
//also this port isn't usually used for HTTP
const RUST_PORT: &str = "127.0.0.1:7878";

fn main() {
    //this generates multiple "We in boys" messages:
    //one for page, and a couple more for other things (like favicon.ico browser icon)
    //browser could also just be retrying to connect, because sometimes the problem is temporary
    app::listen(RUST_PORT);
}
