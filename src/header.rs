const MAX_PACKET_LEN: usize = 512;

/// get the message ID from the message header
pub fn get_id_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u16 {
    ((packet[0] as u16) << 8) | packet[1] as u16
}

/// get the query/response bit from the message header.
/// false: response
/// true: query
pub fn get_qr_from_header(packet: &[u8; MAX_PACKET_LEN]) -> bool {
    packet[2] & 0b_10000000_u8 == 0b_10000000_u8
}

/// get the op code from the message header.
/// OPCODE specifies the query type
/// 0: standard query
/// 1: inverse query
/// 2: server status request
/// 3-15: reserved for future use
pub fn get_op_code_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u8 {
    (packet[2] & 0b_01111000) >> 3
}

/// get the authority bit from the message header
/// Authoritative answer is valid in the response and indicates that the answering server is
/// authoritative for the domain name in the question section
pub fn get_aa_from_header(packet: &[u8; MAX_PACKET_LEN]) -> bool {
    (packet[2] & 0b_00000100_u8) == 0b_00000100_u8
}
/// get the truncated bit from the message header.
/// indicates that the message was truncated. likely more than 512 bytes
pub fn get_tc_from_header(packet: &[u8; MAX_PACKET_LEN]) -> bool {
    (packet[2] & 0b_00000010_u8) == 0b_00000010_u8
}

/// get the RD bit from message header
/// Recursion Desired directs the server to recursivly solve the query if possible and supported.
pub fn get_rd_from_header(packet: &[u8; MAX_PACKET_LEN]) -> bool {
    (packet[2] & 0b_00000001_u8) == 0b_00000001_u8
}

/// get the RA bit from the message header.
/// Recursion Available indicates that the server supports recursion
pub fn get_ra_from_header(packet: &[u8; MAX_PACKET_LEN]) -> bool {
    (packet[3] & 0b_10000000_u8) == 0b_10000000_u8
}

/// reserved for future use
pub fn get_z_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u8 {
    (packet[3] & 0b_01110000_u8) >> 4
}

/// get RCODE from messag header
/// set as part of the reponse
/// 0: no error
/// 1: format error
/// 2: server failure
/// 3: name error
/// 4: not implemented
/// 5: refused
/// 6-15: reserved for future use
pub fn get_r_code_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u8 {
    packet[4] & 0b_00001111_u8
}

/// get QDCOUNT from message header
/// specifies the number of entries in the question section
pub fn get_qd_count_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u16 {
    ((packet[4] as u16) << 8) | packet[5] as u16
}

/// get ANCOUNT from message header
/// specifies the number of resource records in the answer section
pub fn get_an_count_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u16 {
    ((packet[6] as u16) << 8) | packet[7] as u16
}

/// get NSCOUNT from message header
/// specifies the number of name server records in the authority section
pub fn get_ns_count_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u16 {
    ((packet[8] as u16) << 8) | packet[9] as u16
}

/// get ARCOUNT from message header
/// specifies the number of resource records in the additional section
pub fn get_ar_count_from_header(packet: &[u8; MAX_PACKET_LEN]) -> u16 {
    ((packet[10] as u16) << 8) | packet[11] as u16
}

pub fn print_header(packet: &[u8; MAX_PACKET_LEN]) {
    let display = format!(
        r#"ID: {}
QR: {}
Opcode: {:?}
AA: {}
TC: {}
RD: {}
RA: {}
RCODE: {}
QDCOUNT: {}
ANCOUNT: {}
NSCOUNT: {}
ARCOUNT: {}
"#,
        get_id_from_header(packet),
        get_qr_from_header(packet),
        get_op_code_from_header(packet),
        get_aa_from_header(packet),
        get_tc_from_header(packet),
        get_rd_from_header(packet),
        get_ra_from_header(packet),
        get_r_code_from_header(packet),
        get_qd_count_from_header(packet),
        get_an_count_from_header(packet),
        get_ns_count_from_header(packet),
        get_ar_count_from_header(packet)
    );
    print!("{display}");
}

#[cfg(test)]
mod tests {
    use crate::header::{
        get_aa_from_header, get_an_count_from_header, get_ar_count_from_header, get_id_from_header,
        get_op_code_from_header, get_qd_count_from_header, get_qr_from_header,
        get_r_code_from_header, get_ra_from_header, get_rd_from_header, get_tc_from_header,
        get_z_from_header,
    };

    use super::MAX_PACKET_LEN;

    #[test]
    fn test_get_id() {
        let packet = make_test_packet();
        assert_eq!(get_id_from_header(&packet), 37592)
    }

    #[test]
    fn test_get_qr() {
        let packet = make_test_packet();
        assert!(!get_qr_from_header(&packet))
    }
    #[test]
    fn test_get_aa() {
        let packet = make_test_packet();
        assert!(!get_aa_from_header(&packet))
    }
    #[test]
    fn test_get_tc() {
        let packet = make_test_packet();
        assert!(!get_tc_from_header(&packet))
    }
    #[test]
    fn test_get_rd() {
        let packet = make_test_packet();
        assert!(get_rd_from_header(&packet))
    }
    #[test]
    fn test_get_ra() {
        let packet = make_test_packet();
        assert!(!get_ra_from_header(&packet))
    }
    #[test]
    fn test_get_r_code() {
        let packet = make_test_packet();
        assert_eq!(get_r_code_from_header(&packet), 0)
    }
    #[test]
    fn test_get_z_code() {
        let packet = make_test_packet();
        assert_eq!(get_z_from_header(&packet), 0)
    }
    #[test]
    fn test_get_qd_count() {
        let packet = make_test_packet();
        assert_eq!(get_qd_count_from_header(&packet), 1)
    }
    #[test]
    fn test_get_an_count() {
        let packet = make_test_packet();
        assert_eq!(get_an_count_from_header(&packet), 0)
    }
    #[test]
    fn test_get_ns_count() {
        let packet = make_test_packet();
        assert_eq!(get_an_count_from_header(&packet), 0)
    }
    #[test]
    fn test_get_ar_count() {
        let packet = make_test_packet();
        assert_eq!(get_ar_count_from_header(&packet), 0)
    }
    #[test]
    fn test_get_op_code() {
        let packet = make_test_packet();
        assert_eq!(get_op_code_from_header(&packet), 1)
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
