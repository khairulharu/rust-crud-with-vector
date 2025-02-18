use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

use crate::status;

const SERVER_ADDRESS: &str = "127.0.0.1:8080";

pub fn start_server() {
     let listener = TcpListener::bind(SERVER_ADDRESS).unwrap();

     println!("SERVER START RUN AT {}", SERVER_ADDRESS);

     for stream in listener.incoming() {

          match stream { 
               Ok(stream) => {
                    handle_client(stream);
               },
               Err(err) => {
                    println!("Error: {}", err)
               },
          }
     }
}

fn handle_client(mut stream: TcpStream) {
     let mut buffer = [0; 1024];
     let mut request = String::new();

     match stream.read(&mut buffer) {
          Ok(size) => {
               request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

               let (status_line, content) = match &*request {
                    r if r.starts_with("GET /models") => handle_get_all_models(r),
                    r if r.starts_with("POST /models") => handle_post_request(r),
                    _ => (status::NOT_FOUND.to_string(), "404 not found".to_string())
               };

               stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
          },
          Err(err) => {
               println!("Error: {}", err);
          },
     }
}

fn handle_post_request(request: &str) -> (String, String) {
     (status::STATUS_OK.to_string(), request.to_string())
}

fn handle_get_all_models(_request: &str) -> (String, String) {
     (status::STATUS_OK.to_string(), "OK client applied requested".to_string())
}