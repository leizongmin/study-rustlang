use std::fmt;
use std::io::{Read, Write};
use std::net;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl Person {
    fn new(name: String, age: usize) -> Person {
        Person { name, age }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "姓名：{}，年龄：{}", self.name, self.age)
    }
}

fn main() {
    let p = Person::new(String::from("张三"), 20);
    println!("{} {:?}", p, p);

    let s = net::TcpListener::bind("127.0.0.1:4000").unwrap();
    for stream in s.incoming() {
        let s = stream.unwrap();
        println!("new connection {:?}", s);
        handle_connection(s);
    }
}

fn handle_connection(mut s: net::TcpStream) {
    let mut buf = [0; 512];
    while (true) {
        match s.read(&mut buf) {
            Ok(size) => {
                println!("{}", String::from_utf8_lossy(&buf[..]));
                if size < 512 {
                    break;
                }
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
    s.write("HTTP/1.1 200 OK\r\n\r\nHello, world".as_bytes())
        .unwrap();
    s.flush().unwrap();
}
