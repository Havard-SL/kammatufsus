use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

enum FileType {
    html,
    mp3,
}

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

    println!("{}", &request_line);

    let (status_line, filename, filetype) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html", FileType::html),
        "GET /pumpupjam.mp3 HTTP/1.1" => ("HTTP/1.1 200 OK", "pumpupjam.mp3", FileType::mp3),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html", FileType::html),
    };

    let response =  match filetype {
        FileType::html => {
            let contents = fs::read_to_string(filename).unwrap();
            let message = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{contents}", contents.len());
            message.as_bytes()
        },
        FileType::mp3 => {
            todo!();
            let contents = fs::read(filename).unwrap();
            let message = format!("{status_line}\r\nContent-Length: {}\r\n\r\n", contents.len()).as_bytes();
            &vec![message, &contents[..]].concat()
        },
    };

    stream.write_all(response).unwrap();
    // println!("Request: {http_request:#?}");
}
