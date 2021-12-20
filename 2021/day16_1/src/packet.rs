// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    version: u8,
    type_id: u8,
    lit_value: Option<u64>,
    op_mode: Option<u8>,
    op_sub_packets_length: Option<usize>,
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
        
        let binary_str = Self::hex_to_binary_str(hex_str);

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
            while !reached_end {
                // Check leading digit for end (0)
                if binary_str[*posn..*posn+1].starts_with("0") {
                    reached_end = true;
                }

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
                    usize::from_str_radix(&binary_str[*posn..*posn+15], 2).expect("Invalid sp length")
                );
                *posn += 15;

                let current_posn: usize = *posn;
                while *posn < current_posn + packet.op_sub_packets_length.unwrap() {

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

            } // end mode 0
            else {
                packet.op_mode = Some(1);
                *posn += 1;

                // Sub-packet count - 11 bits
                packet.op_sub_packets_count = Some (
                    u32::from_str_radix(&binary_str[*posn..*posn+11], 2).expect("Invalid sp count")
                );
                *posn += 11;

                // collect packets recursively
                for _ in 1..=packet.op_sub_packets_count.unwrap() {

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
            } // end mode 1

            packet.op_sub_packets = Some(sub_packets);

        } // end operator

        //println!("Filled packet: {:?}", packet);
    }


    fn hex_to_binary_str(hex_str: String) -> String {
        
        let mut binary_str = String::with_capacity(6000);

        for hex_char in hex_str.trim().chars() {
            let hex_value = hex_char.to_digit(16).unwrap();
            binary_str.push_str(&format!("{:04b}", hex_value));
        }

        binary_str
    }

    pub fn version_sum(&self) -> u32 {
        let mut sum = 0u32;

        sum += self.version as u32;

        if self.op_sub_packets == None {
            return sum;
        }

        for sub_packet in self.op_sub_packets.as_ref().unwrap().iter() {
            sum += sub_packet.version_sum();
        }

        sum
    }

    pub fn value(&self) -> u64 {

        match self.type_id {
            0 => self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64).sum(),
            1 => self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64).product(),
            2 => self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64).min().unwrap(),
            3 => self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64).max().unwrap(),
            4 => self.lit_value.unwrap(),
            5 => { // gt
                let mut iter_gt = self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64);
                if iter_gt.next().unwrap() > iter_gt.next().unwrap() {
                    1
                }
                else {0}
            },
            6 => { //lt
                let mut iter_gt = self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64);
                if iter_gt.next().unwrap() < iter_gt.next().unwrap() {
                    1
                }
                else {0}
            },
            7 => { //eq
                let mut iter_gt = self.op_sub_packets.as_ref().unwrap().iter().map(|packet| packet.value() as u64);
                if iter_gt.next().unwrap() == iter_gt.next().unwrap() {
                    1
                }
                else {0}
            },
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn packet_literal() {
        let result = Packet::new("D2FE28".to_string());
        println!("Literal packet: {:?}", result);
        assert_eq!(6, result.version_sum());   // fail to print
    }

    #[test]
    fn packet_operator_mode1() {
        let result = Packet::new("EE00D40C823060".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(14, result.version_sum());   // fail to print
    }

    #[test]
    fn packet_operator_mode0() {
        let result = Packet::new("38006F45291200".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(9, result.version_sum());   // fail to print
    }

    #[test]
    fn packet_3nestedoperator_lit() {
        let result = Packet::new("8A004A801A8002F478".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(16, result.version_sum());   // fail to print
    }

    #[test]
    fn packet_3nestedoperator_5lit() {
        let result = Packet::new("A0016C880162017C3686B18A3D4780".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(31, result.version_sum());   // fail to print
    }

    #[test]
    fn packet_sum_2lit() {
        let result = Packet::new("C200B40A82".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(3, result.value());   // fail to print
    }

    #[test]
    fn packet_min_3lit() {
        let result = Packet::new("880086C3E88112".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(7, result.value());   // fail to print
    }

    #[test]
    fn packet_max_3lit() {
        let result = Packet::new("CE00C43D881120".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(9, result.value());   // fail to print
    }

    #[test]
    fn packet_gt_2lit() {
        let result = Packet::new("F600BC2D8F".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(0, result.value());   // fail to print
    }


    #[test]
    fn packet_lt_2lit() {
        let result = Packet::new("D8005AC2A8F0".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(1, result.value());   // fail to print
    }

    #[test]
    fn packet_eq_2lit() {
        let result = Packet::new("9C005AC2F8F0".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(0, result.value());   // fail to print
    }

    #[test]
    fn packet_eq_sumprod_2lits() {
        let result = Packet::new("9C0141080250320F1802104A08".to_string());
        println!("Operator packet: {:?}", result);
        assert_eq!(1, result.value());   // fail to print
    }

}