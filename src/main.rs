mod model;
mod repository;
mod server;
mod status;

mod tests;

fn main() {
    server::start_server();
}