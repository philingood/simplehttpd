use std::io::{BufRead, Write};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let server_adress = env::var("SERVER_ADRESS").unwrap();
    let listner = std::net::TcpListener::bind(server_adress).unwrap();
    for mut stream in listner.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&stream);
        let mut line = String::new();
        rdr.read_line(&mut line).unwrap();
        match line.trim().split(" ").collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut line = String::new();
                    rdr.read_line(&mut line).unwrap();
                    if line.trim().is_empty() { break; }
                }
                let mut p = std::path::PathBuf::new();
                p.push("site");
                p.push(resource.trim_start_matches("/"));
                if resource.ends_with("/") { p.push("index.html");}
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                println!("HTTP/1.1 200 OK");
                stream.write_all(&std::fs::read(p).unwrap()).unwrap();
            }
            _ => todo!()
        }
    }
}