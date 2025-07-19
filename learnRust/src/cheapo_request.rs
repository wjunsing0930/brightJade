use async_std::io::prelude::*;
use async_std::net;

async fn cheapo_request(host:&str, port:u16, path:&str)
                        -> std::io::Result<String>
{
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n",path,host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    
    Ok(response)
}
#[test]
fn test_async_response() -> std::io::Result<()>{
    use async_std::task;
    let response = task::block_on(cheapo_request("example.com",80,"/"))?;
    println!("response: {}", response);
    Ok(())
}

#[tokio::test]
async fn test_async_response2() ->std::io::Result<()>{
    let response = cheapo_request("example.com",80,"/").await?;
    println!("response: {}", response);
    Ok(())
}