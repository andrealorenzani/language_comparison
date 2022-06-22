use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize,Deserialize)]
struct Data {
    test: Vec<i64>
}

fn main() {
    print!("Starting at 127.0.0.1:7878");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    print!("Starting at 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        print!("Got connection");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = String::new();

    stream.read_to_string(&mut buffer).unwrap();

    print!("Stream: {}", buffer);

    let mut v:Data = serde_json::from_str(&buffer).unwrap();

    print!("{:?}", &mut v.test);

    let contents = insertion_sort(&mut v.test);

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{:?}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn insertion_sort(vec: &mut[i64]) -> &[i64] {
    let array: &mut [i64] = vec;
    for i in 0..array.len() {
        // Start comparing current element with every element before it
        for j in i..=0 {
          
            // Swap elements as required
            if array[j + 1] < array[j] {
                let swap = array[j + 1];
                array[j + 1] = array [j];
                array[j] = swap;
            }
        }
  }
  return array;
}
