extern crate rustc_serialize;
use rustc_serialize::json;
use std::net;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Request  {
    name: String,
    x: u16,
    y: u16,
    data: Vec<u8>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Response  {
    name: String,
    id: u8,
}

fn main() {
    let request_struct = Request {
        name: "Bot".to_string(),
        x: 321,
        y: 123,
        data: vec![5,4,3,2,1],
    };

    let request_json = json::encode(&request_struct).unwrap();
    println!("-> {}", request_json);

    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let server_addr = net::SocketAddrV4::new(ip, 2000);
    let client_addr = net::SocketAddrV4::new(ip, 2001);
    let socket = net::UdpSocket::bind(net::SocketAddr::V4(client_addr)).unwrap();

    socket.send_to(&request_json.into_bytes(), net::SocketAddr::V4(server_addr)).unwrap();

    let mut buf = [0; 10000];
    let (bytes, _): (usize, net::SocketAddr) = socket.recv_from(&mut buf).unwrap();
    let response_json: String = String::from_utf8(buf[..bytes].to_vec()).unwrap();
    println!("<- {}", response_json);

    let response_struct: Response = json::decode(&response_json).unwrap();
    println!("Parsed name: {}, id: {}", response_struct.name, response_struct.id);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sample() {
        assert_eq!(1, 1);
    }
}
