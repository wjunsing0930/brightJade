use async_std::io::prelude::*;
use async_std::net;

async fn cheapo_request(host:&str, port:u16,path:&str)
                -> std::io::Result<String>
{
    let mut socket = net::TcpStream::connect((host, port)).await?;
    let request = format!("GET {} HTTP/1.1\r\nHost:{}\r\n",path,host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write);

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    Ok(response)
}

async fn many_request(requests: Vec<(String, u16, String)>)
    -> Vec<std::io::Result<String>>
{
    use async_std::task;
    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn(async move {
            cheapo_request(&host, port, &path).await
        }));
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

#[test]
fn test_many_request(){
    let requests = vec![
        ("example.com".to_string(),      80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];

    let results = async_std::task::block_on(many_request(requests));
    for result in results{
        match result{
            Ok(response) => println!{"get response:{}",response},
            Err(err) => eprintln!("error: {}", err),
        }
    }
}

use surf;
pub async fn many_request2(urls: &[String])
                           -> Vec<Result<String, surf::Error>>
{
    let client = surf::Client::new();
    let mut handles = vec![];
    for url in urls{
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }
    let mut results = vec![];
    for handle in handles{
        results.push(handle.await);
    }
    results
}

#[test]
fn test_many_request2(){
    let requests = &["http://example.com".to_string(),
                     "https://www.red-bean.com".to_string(),
                     "https://en.wikipedia.org/wiki/Main_Page".to_string()];

    let results = async_std::task::block_on(many_request2(requests));
    for result in results{
        match result{
            Ok(response) => println!("*** {}\n", response),
            Err(err) => eprintln!("error: {}\n", err),
        }
    }
}
