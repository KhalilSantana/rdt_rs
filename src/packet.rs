use crate::payload::*;

#[derive(Debug, Clone)]
pub struct Packet {
    pub seq_num: u32,
    pub pkt_type: PacketType,
    pub payload: Payload,
    pub checksum: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PacketType {
    Acknowlodge,
    NotAcklodge,
    Data,
}

impl Packet {
    pub fn new(seq_num: u32, pkt_type: PacketType, payload: Payload) -> Self {
        let mut tmp = Packet {
            seq_num,
            pkt_type,
            payload,
            checksum: 0,
        };
        tmp.create_checksum();
        tmp
    }
    pub fn ack(seq_num: u32) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::Acknowlodge,
            payload: Payload::new([0; 5]),
            checksum: 0,
        };
        pkt.create_checksum();
        pkt
    }
    pub fn nack(seq_num: u32) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::NotAcklodge,
            payload: Payload::new([0; 5]),
            checksum: 0,
        };
        pkt.create_checksum();
        pkt
    }

    pub fn data(seq_num: u32, payload: Payload) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::Data,
            payload,
            checksum: 0,
        };
        pkt.create_checksum();
        pkt
    }
    pub fn create_checksum(&mut self) {
        self.checksum = 42;
    }
    pub fn checksum_ok(&self) -> bool {
        match self.checksum {
            42 => true,
            _ => false,
        }
    }
    // pub fn corrupt_data(&mut self) {
    //     self.pkt_data = self.pkt_data.reverse_bits();
    // }
    pub fn corrupt_headers(&mut self) {
        self.checksum = 1337;
    }
}
