extern crate resp;
extern crate env_logger;
extern crate dotenv;
use resp::{Server, Config};

fn main() {
    dotenv::dotenv().ok();
    ::std::env::set_var("RUST_LOG", "actix_web=error");
    env_logger::init();

    let server = Server::new(Config{});
    ::std::process::exit(server.start());
}