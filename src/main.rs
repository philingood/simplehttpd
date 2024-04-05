use std::io::{BufRead, Write};

fn main() {
    let listner = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
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
                if resource.ends_with("/") {
                    // p.push("site");
                    p.push("index.html");
                }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                println!("Path to file: {:?}", p);
                stream.write_all(&std::fs::read(p).unwrap()).unwrap();
            }
            _ => todo!()
        }
    }
}