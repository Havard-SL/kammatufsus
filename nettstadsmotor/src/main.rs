use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread,
};

#[derive(Clone)]
#[derive(serde::Deserialize)]
// #[derive(serde::Serialize)]
struct ValidFile {
    name: String,
    mime_type: String,
}

const VALID_FILES_PATH: &str = "valid_files.json";

fn file_is_valid_file<'a>(filename: &str, valid_files: &'a Vec<ValidFile>) -> Option<&'a ValidFile> {
    for valid_file in valid_files {
        if filename == valid_file.name {
            return Some(valid_file)
        }
    }
    None
}

fn read_valid_files() -> Vec<ValidFile> {
    let valid_files = fs::read_to_string(VALID_FILES_PATH).unwrap();
    serde_json::from_str(&valid_files).unwrap()
}

// fn write_valid_files() {
//     let a = ValidFile {name: "Test1".to_string(), mime_type: "Test1.1".to_string()};
//     let b = ValidFile {name: "Test2".to_string(), mime_type: "Test2.2".to_string()};
//     let valid_files = vec![a, b];
//     println!("{}", serde_json::to_string(&valid_files).unwrap())
// }

fn main() {
    println!("Hello, world!");

    println!("Parsing valid files.");

    let valid_files = read_valid_files();
    let valid_files = &valid_files;

    println!("Finished parsing valid files.");

    println!("Starting to listen.");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    thread::scope(|s| {
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");

            s.spawn(|| {
                handle_connection(stream, valid_files);
            });
        }
    })
}

fn handle_connection(mut stream: TcpStream, valid_files: &Vec<ValidFile>) {
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

    if command != "GET" || http_version != "HTTP/1.1" {
        print!("Got a weird connection request.");
        return;
    }

    let valid_file = file_is_valid_file(filename, valid_files);
    
    let valid_file = match valid_file {
        Some(v) => {
            status_line = "HTTP/1.1 200 OK";
            v
        },
        None => {
            status_line = "HTTP/1.1 404 NOT FOUND";
            &ValidFile { 
                name: "404.html".to_string(), 
                mime_type: "text/html".to_string() 
            }
        },
    };

    let contents = fs::read(filename).unwrap();

    let headers = format!("Content-Length: {}\r\nContent-Type: {}", contents.len(), valid_file.mime_type);
    
    let message = format!("{status_line}\r\n{headers}\r\n\r\n");
    let response =  &[message.as_bytes(), &contents[..]].concat();

    stream.write_all(response).unwrap();
}
