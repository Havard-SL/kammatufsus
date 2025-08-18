use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const VALID_FILES: [&str; 10] = ["hello.html", "pumpupjam.mp3", "favicon.ico", "Hugo.jpeg", "ganesha.jpeg", "mosque.html", "meditation.mp3", "robots.txt", "scripts.js", "styles.css"];

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut request_line = request_line.split(' ');

    let command = request_line.next().unwrap();
    let mut filename = &request_line.next().unwrap()[1..];
    let http_version = request_line.next().unwrap();

    let status_line;

    if filename.is_empty() {
        filename = "hello.html";
    }

    println!("{}", filename);

    if command == "GET" && VALID_FILES.contains(&filename) && http_version == "HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        filename = "404.html"
    }

    let contents = fs::read(filename).unwrap();
    
    let message = format!("{status_line}\r\nContent-Length: {}\r\n\r\n", contents.len());
    let response =  &[message.as_bytes(), &contents[..]].concat();

    stream.write_all(response).unwrap();
    // println!("Request: {http_request:#?}");
}
