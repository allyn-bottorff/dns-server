use core::str;
use std::usize;
use std::vec::Vec;

const MAX_PACKET_LEN: usize = 512;

// get all the QNAMEs out of the packet.
// TODO: Could probably make this faster by not using a String since we have a well-known upper
// bound on the size of labels
pub fn get_qnames(packet: &[u8; MAX_PACKET_LEN]) -> Vec<&str> {
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
        let label = str::from_utf8(&packet[q_ptr..q_ptr + label_len]).unwrap();
        labels.push(label);
        q_ptr += label_len;
        loop_count += 1;
    }
    //labels are restricted to 63 octets or less

    labels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_qnames() {
        let packet = make_test_packet();
        let mut names = get_qnames(&packet);

        let name = match names.pop() {
            Some(n) => n,
            None => "",
        };

        assert_eq!(name, "google.com")
    }

    fn make_test_packet() -> [u8; MAX_PACKET_LEN] {
        let mut buf: [u8; MAX_PACKET_LEN] = [0; MAX_PACKET_LEN];
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
