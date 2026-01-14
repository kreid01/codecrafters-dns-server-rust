#[allow(unused_imports)]
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = vec![
                    0x04, 0xD2, 0x80, // QR=1, OPCODE=0, AA=0, TC=0, RD=0       │
                    0x00, // RA=0, Z=0, RCODE=0                     │
                    0x00, 0x00, // Question Count (QDCOUNT): 0      │
                    0x00, 0x00, // Answer Record Count (ANCOUNT): 0 │
                    0x00, 0x00, // Authority Record Count           │
                    0x00, 0x00, // Additional Record Count          │
                ];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
