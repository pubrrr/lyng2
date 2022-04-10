use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc::channel;
use std::thread::{sleep, spawn};
use std::time::Duration;

use http::header::{CONNECTION, SEC_WEBSOCKET_ACCEPT, UPGRADE};
use http::{HeaderMap, Response, Version};
use httparse::{Request, EMPTY_HEADER};
use sha1_smol::Sha1;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    server.set_nonblocking(true).unwrap();

    let (sender, receiver) = channel();

    let mut clients = vec![];

    loop {
        if let Ok((mut stream, address)) = server.accept() {
            println!("{} connected", address);
            clients.push(stream.try_clone().unwrap());
            let sender = sender.clone();

            spawn(move || {
                loop {
                    let mut input = [0; 1000];

                    match stream.read(&mut input) {
                        // match stream.read_to_string(&mut input) {
                        // match stream.read(&mut input) {
                        // match stream.read_exact(&mut input) {
                        Ok(number_of_bytes_read) => {
                            if number_of_bytes_read == 0 {
                                continue;
                            }
                            println!("-------------------");
                            println!("bytes: {:?}", input);
                            let input = String::from_utf8_lossy(&input[0..number_of_bytes_read])
                                .to_string();
                            println!("got: {}", input);
                            // println!("sending: {}", String::from_utf8_lossy(&input));
                            println!("-------------------");

                            sender.send(input).unwrap();
                        }
                        Err(err) if err.kind() == ErrorKind::WouldBlock => {}
                        Err(err) => {
                            println!("{} disconnected: {:?}", address, err);
                            println!("################################\n");
                            break;
                        }
                    }

                    sleep(Duration::from_millis(100));
                }
            });
        }

        if let Ok(message) = receiver.try_recv() {
            println!("received message: {:?}", message);

            let mut headers = [EMPTY_HEADER; 20];
            let mut request = Request::new(&mut headers);
            request.parse(message.as_bytes());

            let string = if let Some(websocket_key) = request
                .headers
                .iter()
                .find(|header| header.name == "Sec-WebSocket-Key")
            {
                let websocket_key = String::from_utf8_lossy(websocket_key.value).to_string();

                let websocket_accept = compute_websocket_accept(websocket_key);

                let response = Response::builder()
                    .status(101)
                    .version(Version::HTTP_11)
                    .header(SEC_WEBSOCKET_ACCEPT, websocket_accept)
                    // .header(SEC_WEBSOCKET_EXTENSIONS, "permessage-deflate")
                    .header(UPGRADE, "websocket")
                    .header(CONNECTION, "Upgrade")
                    .body(())
                    .unwrap();

                to_string(response)
            } else {
                message
            };

            println!("sending:");
            println!("{}", string);
            println!("--");

            for client in &mut clients {
                client.write(string.as_bytes()).unwrap();
            }
        }

        sleep(Duration::from_secs_f32(1.));
    }
}

fn to_string(response: Response<()>) -> String {
    format!(
        "{protocol:?} {status}\r\n{headers}\r\n",
        protocol = response.version(),
        status = response.status(),
        headers = format_headers(response.headers())
    )
}

fn format_headers(headers: &HeaderMap) -> String {
    headers
        .iter()
        .fold(String::new(), |mut result, (name, value)| {
            result.push_str(&format!("{}: {}\r\n", name, value.to_str().unwrap()));
            result
        })
}

fn compute_websocket_accept(websocket_key: String) -> String {
    let sha1_hash = Sha1::from(websocket_key + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11").digest();
    base64::encode(sha1_hash.bytes())
}

#[cfg(test)]
mod tests {
    use http::{Response, Version};

    use crate::{compute_websocket_accept, to_string};

    #[test]
    fn test() {
        let actual = compute_websocket_accept(String::from("dGhlIHNhbXBsZSBub25jZQ=="));

        assert_eq!("s3pPLMBiTxaQ9kYGzzhZRbK+xOo=", actual)
    }

    #[test]
    fn test2() {
        let response = Response::builder()
            .status(101)
            .version(Version::HTTP_11)
            .header("Sec-WebSocket-Accept", "muh")
            .body(())
            .unwrap();

        let expected = "HTTP/1.1 101 Switching Protocols\r\nsec-websocket-accept: muh\r\n\r\n";
        assert_eq!(expected, to_string(response));
    }
}
