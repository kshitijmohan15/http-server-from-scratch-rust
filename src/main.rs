use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 4221))).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = handle_connection(stream);
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct ReqContent<'a> {
    http_method: Option<&'a str>,
    path: Option<&'a str>,
    http_version: Option<&'a str>,
    user_agent: Option<&'a str>,
    host: Option<&'a str>,
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut request = [0u8; 1024];
    stream.read(&mut request).unwrap();

    let req_content = request.into_iter().map(char::from).collect::<String>();

    let splitted_content = req_content
        .split("\r\n")
        .filter(|it| !it.is_empty() && !it.contains('\0'))
        .collect::<Vec<&str>>();

    let req_struct = get_req_content(&splitted_content);
    dbg!(&req_struct);
    let response = if req_struct.path.unwrap().eq("/") {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    let _ = stream.write(response.as_bytes()).unwrap();
    Ok(())
}
impl<'a> ReqContent<'a> {
    pub fn new() -> ReqContent<'a> {
        Self {
            http_method: None,
            http_version: None,
            path: None,
            host: None,
            user_agent: None,
        }
    }
}
fn get_req_content<'a>(vec: &'a [&str]) -> ReqContent<'a> {
    let mut req_content = ReqContent::new();

    let split_line_1 = vec[0].split_whitespace().collect::<Vec<&str>>();
    let (meth, path, version) = (split_line_1[0], split_line_1[1], split_line_1[2]);
    req_content.http_method = Some(meth);
    req_content.http_version = Some(version);
    req_content.path = Some(path);
    for idx in 1..vec.len() {
        match vec[idx] {
            x if x.starts_with("Host:") => {
                req_content.host = Some(x.split(": ").collect::<Vec<_>>()[1]);
            }
            x if x.starts_with("User-Agent:") => {
                req_content.user_agent = Some(x.split(": ").collect::<Vec<_>>()[1]);
            }
            _ => {}
        }
    }

    // vec[0]
    //     .split_whitespace()
    //     .for_each(|line|) ;
    // ReqContent {
    //     http_method: Some(meth),
    //     path: Some(path),
    //     http_version: Some(version),
    //     host: None,
    //     user_agent: None,
    // }
    req_content
}
