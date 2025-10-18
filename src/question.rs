use core::str;
use std::usize;
use std::vec::Vec;

use crate::FullPacket;

// get all the QNAMEs out of the packet.
// TODO: Could probably make this faster by not using a String since we have a well-known upper
// bound on the size of labels
pub fn get_qnames(packet: &FullPacket) -> Vec<&str> {
    // let qcount = get_qd_count_from_header(packet);
    let mut q_ptr: usize = 12; // The header is always 12 bytes and the question starts
                               // immediately after

    let mut labels: Vec<&str> = Vec::new();

    let mut loop_count = 0;
    loop {
        let label_len = packet[q_ptr] as usize;
        if loop_count > 100 {
            panic!("not finding the end of the label");
        }
        if label_len == 0 {
            break;
        }
        let label = str::from_utf8(&packet[q_ptr + 1..=q_ptr + label_len]).unwrap();
        labels.push(label);
        q_ptr += label_len + 1;
        loop_count += 1;
    }
    //labels are restricted to 63 octets or less

    labels
}

pub fn get_questions(packet: &FullPacket) -> (Vec<&str>, u16, u16) {
    // let qcount = get_qd_count_from_header(packet);
    let mut q_ptr: usize = 12; // The header is always 12 bytes and the question starts
                               // immediately after

    let mut labels: Vec<&str> = Vec::new();

    let mut loop_count = 0;
    loop {
        let label_len = packet[q_ptr] as usize;
        if loop_count > 100 {
            panic!("not finding the end of the label");
        }
        if label_len == 0 {
            q_ptr += 1;
            break;
        }
        let label = str::from_utf8(&packet[q_ptr + 1..=q_ptr + label_len]).unwrap();
        labels.push(label);
        q_ptr += label_len + 1;
        loop_count += 1;
    }
    //labels are restricted to 63 octets or less
    let qtype: u16 = (packet[q_ptr] as u16) << 8 | packet[q_ptr + 1] as u16;
    q_ptr += 2;
    let qclass: u16 = (packet[q_ptr] as u16) << 8 | packet[q_ptr + 1] as u16;

    (labels, qtype, qclass)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FullPacket;
    use crate::MAX_PACKET_LEN;

    #[test]
    fn test_get_qnames() {
        let packet = make_test_packet();
        let labels = get_qnames(&packet);
        let name = labels.join(".");

        assert_eq!(name, "google.com")
    }

    #[test]
    fn test_get_questions() {
        let packet = make_test_packet();
        let (labels, qtype, qclass) = get_questions(&packet);
        let name = labels.join(".");

        assert_eq!(name, "google.com");
        assert_eq!(qtype, 1);
        assert_eq!(qclass, 1);
    }

    fn make_test_packet() -> FullPacket {
        let mut buf: FullPacket = [0; MAX_PACKET_LEN];
        let test_query = [
            0x92_u8, 0xd8_u8, 0x01_u8, 0x20_u8, 0x00_u8, 0x01_u8, 0x00_u8, 0x00_u8, 0x00_u8,
            0x00_u8, 0x00_u8, 0x00_u8, 0x06_u8, 0x67_u8, 0x6f_u8, 0x6f_u8, 0x67_u8, 0x6c_u8,
            0x65_u8, 0x03_u8, 0x63_u8, 0x6f_u8, 0x6d_u8, 0x00_u8, 0x00_u8, 0x01_u8, 0x00_u8,
            0x01_u8,
        ];

        // 00000000  92 d8 01 20 00 01 00 00  00 00 00 00 06 67 6f 6f  |... .........goo|
        // 00000010  67 6c 65 03 63 6f 6d 00  00 01 00 01              |gle.com.....|
        // 0000001c

        buf[..test_query.len()].copy_from_slice(&test_query[..]);
        buf
    }
}
