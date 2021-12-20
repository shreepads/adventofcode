// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    version: u8,
    type_id: u8,
    lit_value: Option<u64>,
    op_mode: Option<u8>,
    op_sub_packets_length: Option<u32>,
    op_sub_packets_count: Option<u32>,
    op_sub_packets: Option<Vec<Packet>>,
}

impl Packet {
    pub fn new(hex_str: String) -> Packet {
        
        let mut packet = Packet {
            version: 0,
            type_id: 0,
            lit_value: None,
            op_mode: None,
            op_sub_packets_length: None,
            op_sub_packets_count: None,
            op_sub_packets: None,
        };
        
        let mut binary_str = Self::hex_to_binary_str(hex_str);

        let mut posn = 0usize;

        Self::fill_packet(&mut packet, &binary_str, &mut posn);
        
        packet

    }

    fn fill_packet(packet: &mut Packet, binary_str: &String, posn: &mut usize) {

        // version - 3 bits
        packet.version = u8::from_str_radix(&binary_str[*posn..*posn+3], 2)
            .expect("Invalid version");
        
        *posn += 3;

        // type_id - 3 bits
        packet.type_id = u8::from_str_radix(&binary_str[*posn..*posn+3], 2)
            .expect("Invalid type id");
        
        *posn += 3;

        if packet.type_id == 4 { // literal
            let mut reached_end = false;
            let mut literal_str = String::new();
            let mut literal_chunks = 0;
            while !reached_end {
                // Check leading digit for end (0)
                if binary_str[*posn..*posn+1].starts_with("0") {
                    reached_end = true;
                }

                literal_chunks += 1;
                *posn += 1;

                // Collect 4 bits of literal
                literal_str.push_str(&binary_str[*posn..*posn+4]);
                *posn += 4;
            }

            packet.lit_value = Some
                (u64::from_str_radix(&literal_str, 2).expect("Invalid literal")
            );
        } // end literal
        else { // operator

            let mut sub_packets: Vec<Packet> = Vec::new();

            // Check leading digit for mode (0)
            if binary_str[*posn..*posn+1].starts_with("0") {
                packet.op_mode = Some(0);
                *posn += 1;
                
                // Sub-packet length - 15 bits
                packet.op_sub_packets_length = Some(
                    u32::from_str_radix(&binary_str[*posn..*posn+15], 2).expect("Invalid sp length")
                );
                *posn += 15;

            } else {
                packet.op_mode = Some(1);
                *posn += 1;

                // Sub-packet count - 11 bits
                packet.op_sub_packets_count = Some (
                    u32::from_str_radix(&binary_str[*posn..*posn+11], 2).expect("Invalid sp count")
                );
                *posn += 11;

                for i in 1..=packet.op_sub_packets_count.unwrap() {

                    let mut sub_packet = Packet {
                        version: 0,
                        type_id: 0,
                        lit_value: None,
                        op_mode: None,
                        op_sub_packets_length: None,
                        op_sub_packets_count: None,
                        op_sub_packets: None,
                    };

                    Self::fill_packet(&mut sub_packet, binary_str, posn);
                    sub_packets.push(sub_packet);
                }
            } // end mode

            packet.op_sub_packets = Some(sub_packets);

        } // end operator

    }


    fn hex_to_binary_str(hex_str: String) -> String {
        
        let mut binary_str = String::new();

        for hex_char in hex_str.chars() {
            let hex_value = hex_char.to_digit(16).unwrap();
            binary_str.push_str(&format!("{:04b}", hex_value));
        }

        binary_str
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn packet_literal() {
        let result = Packet::new("D2FE28".to_string());
        println!("Literal packet: {:?}", result);
        assert_eq!(2, 3);   // fail to print
    }

    #[test]
    fn packet_operator_mode1() {
        let result = Packet::new("EE00D40C823060".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(2, 3);   // fail to print
    }
}