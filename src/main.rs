use std::net;

pub mod header;
pub mod question;

const MAX_PACKET_LEN: usize = 512;

fn main() {
    let socket = net::UdpSocket::bind("127.0.0.1:1053").expect("couldn't bind to address");
    let mut query_packet: [u8; MAX_PACKET_LEN];

    loop {
        query_packet = [0; MAX_PACKET_LEN];
        // check amount for reading more than 512 bytes
        let (amt, _src) = socket.recv_from(&mut query_packet).unwrap();
        if amt > 512 {
            //TODO: (alb) this panic is just for testing. Handle this more gracefully
            panic!("packet was too large");
        }
        header::print_header(&query_packet);
        let names = question::get_qnames(&query_packet);

        for name in names {
            println!("{name}");
        }
    }
}
