use std::io::prelude::*;
use std::str;
use std::net::{TcpListener, TcpStream, Shutdown};
use serde::{Serialize, Deserialize};
use std::thread;
use std::time::SystemTime;

#[derive(Debug, Serialize,Deserialize)]
struct Data {
    test: Vec<i64>
}

fn main() -> std::io::Result<()>{
    println!("Starting at 127.0.0.1:7878");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    for stream in listener.incoming() {
        println!("Connection established");
        let stream = stream.unwrap();
        handle_connection(stream).unwrap();
    }
    Ok(())
}

fn read_stream(stream: &mut TcpStream) -> (String, usize) {
    let buffer_size = 512;
    let mut request_buffer = vec![];
    // let us loop & try to read the whole request data
    let mut request_len = 0usize;
    loop {
        let mut buffer = vec![0; buffer_size];
        match stream.read(&mut buffer) {
            Ok(n) => {

                if n == 0 {
                    break;
                } else {
                    request_len += n;
                    request_buffer.append(&mut buffer);

                    // we need not read more data in case we have read less data than buffer size
                    if n < buffer_size {
                        break;
                    }
                }
            },

            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }

    let s = match str::from_utf8(&request_buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    (s.to_string(), request_len)
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let (mut request, mut len) = read_stream(&mut stream);
    
    if request.contains("Expect: 100-continue") {
        let response = format!(
            "HTTP/1.1 100 OK\r\n",
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        (request, len) = read_stream(&mut stream);
        request.truncate(len);
    }

    let mut v:Data = serde_json::from_str(&request).unwrap();

    println!("Deserialised {} numbers", v.test.len());

    let mut now = SystemTime::now();

    v.test = insertion_sort(&mut v.test).to_vec();

    println!("Insertion sort: {} millis", now.elapsed().expect("wow").as_millis());
    now = SystemTime::now();

    for i in 0..200 {
        let mut newvec = v.test.to_vec();
        let index = i;
        thread::spawn(move || {
            insertion_sort(&mut newvec).to_vec();
            println!("Insertion sort in thread {}: {} millis", index, now.elapsed().expect("wow").as_millis());
        });
    }

    let content = serde_json::to_string(&v).expect("wow");

    let response = format!(
        "HTTP/1.1 200 OK\r\nConnection:close\r\nContent-Length: {}\r\n{:?}\r\n",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
    Ok(())
}

fn insertion_sort(vec: &mut[i64]) -> &[i64] {
    let array: &mut [i64] = vec;
    for i in 0..array.len() {
        // Start comparing current element with every element before it
        for j in (0..i).rev() {
          
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
