#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                let mut pos = 12;

                while buf[pos] != 0 {
                    pos += buf[pos] as usize + 1;
                }
                pos += 1;
                pos += 4;

                let question = &buf[12..pos];

                let header = [
                    buf[0],
                    buf[1],
                    buf[2] | 0x80,
                    buf[3],
                    0x00,
                    0x01,
                    0x00,
                    0x01,
                    0x00,
                    0x00,
                    0x00,
                    0x00,
                ];

                let answer = [
                    0xC0, 0x0C, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x04, 0x7F,
                    0x00, 0x00, 0x01,
                ];

                let mut response = Vec::new();
                response.extend_from_slice(&header);
                response.extend_from_slice(question);
                response.extend_from_slice(&answer);

                udp_socket.send_to(&response, source).unwrap();
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
