#![allow(warnings)]
use std::{fs, thread};
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use httpd::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    let pool = ThreadPool::new(4);

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //
    //     pool.execute(|| {
    //         handle_conn(stream);
    //     });
    // }

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }

    println! ("关闭中。");
}




// --跳过代码--
fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{req_line}");

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ( "HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 0K", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let resp =
        format! ("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(resp.as_bytes()).unwrap();
}

