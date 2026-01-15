#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let mut pos = 12;

                parse_qname(&buf, &mut pos);
                pos += 4;

                let question = &buf[12..pos];

                let header_response = vec![
                    buf[0], buf[1], 0x80, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ];

                let mut response = Vec::new();
                response.extend_from_slice(&header_response);
                response.extend_from_slice(question);

                udp_socket.send_to(&response, source).unwrap();
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}

fn parse_qname(buf: &[u8], pos: &mut usize) -> String {
    let mut labels = Vec::new();

    loop {
        let len = buf[*pos] as usize;
        *pos += 1;

        if len == 0 {
            break;
        }

        let label = &buf[*pos..*pos + len];
        labels.push(String::from_utf8_lossy(label).to_string());
        *pos += len;
    }

    labels.join(".")
}
