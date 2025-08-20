use std::io::{self, Write, Read};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // 指定服务器地址和端口
    let server_addr = "127.0.0.1:17007";

    // 连接到服务器
    let mut stream = TcpStream::connect(server_addr)?;

    println!("Connected to {}", server_addr);

    // 向服务器发送数据
    let msg = b"Hello, Server!";
    stream.write_all(msg)?;

    println!("Sent message: {:?}", msg);

    // 接收服务器响应
    let mut response = [0; 1024];
    let n = stream.read(&mut response)?;

    // 打印服务器返回的消息
    println!("Received message: {:?}", &response[..n]);

    Ok(())
}
