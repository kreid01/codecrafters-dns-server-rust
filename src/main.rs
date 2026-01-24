use std::net::UdpSocket;

fn main() {
    println!("Starting DNS server on 127.0.0.1:2053");

    let socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind UDP socket");

    let mut buf = [0u8; 512];

    loop {
        let (_size, source) = socket.recv_from(&mut buf).expect("Failed to receive data");

        let id = [buf[0], buf[1]];
        let opcode = (buf[2] >> 3) & 0x0F;
        let qdcount = u16::from_be_bytes([buf[4], buf[5]]) as usize;

        let mut pos = 12;
        for _ in 0..qdcount {
            // QNAME
            while buf[pos] != 0 {
                pos += buf[pos] as usize + 1;
            }
            pos += 1;
            pos += 4;
        }

        let questions = &buf[12..pos];

        let mut flags_hi = 0x80;
        flags_hi |= buf[2] & 0x79;

        let rcode = if opcode == 0 { 0 } else { 4 };
        let flags_lo = rcode & 0x0F;

        let ancount = if opcode == 0 { qdcount as u16 } else { 0 };

        let header = [
            id[0],
            id[1],
            flags_hi,
            flags_lo,
            buf[4],
            buf[5], // QDCOUNT
            (ancount >> 8) as u8,
            (ancount & 0xFF) as u8, // ANCOUNT
            0x00,
            0x00, // NSCOUNT
            0x00,
            0x00, // ARCOUNT
        ];

        let mut response = Vec::new();
        response.extend_from_slice(&header);
        response.extend_from_slice(questions);

        if opcode == 0 {
            let answer = [
                0xC0, 0x0C, // NAME
                0x00, 0x01, // TYPE A
                0x00, 0x01, // CLASS IN
                0x00, 0x00, 0x00, 0x3C, 0x00, 0x04, // RDLENGTH
                0x7F, 0x00, 0x00, 0x01,
            ];

            for _ in 0..qdcount {
                response.extend_from_slice(&answer);
            }
        }

        socket
            .send_to(&response, source)
            .expect("Failed to send response");
    }
}
