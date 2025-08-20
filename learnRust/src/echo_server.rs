#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::net::TcpListener;
use std::thread::spawn;
use std::io;

fn echo_main(addr: &str) -> io::Result<()>{
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop{
        let(mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        let mut write_stream = stream.try_clone()?;
        spawn(move ||{
            io::copy(&mut stream, &mut write_stream)
                .expect("error in client thread: ");
            println!("connection closed.");
        });
    }
}
#[test]
fn echo_main_test(){
    echo_main("127.0.0.1:17007").expect("error : ");
}
