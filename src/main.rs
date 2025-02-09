mod model;
mod repository;
mod server;
mod client;
mod status;
mod parser;

fn main() {
    server::start_server();
}