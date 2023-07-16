use rust_web_server::ThreadPool;
use std::{
    thread,
    fs,
    time::Duration,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener =  TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Connection established!");
  
        pool.execute(|| {
            handle_connection(stream)
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
      ("HTTP/1.1 200 OK", "response.html")

    } else if request_line == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "response.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
    );

    stream.write_all(response.as_bytes()).unwrap();

   
}