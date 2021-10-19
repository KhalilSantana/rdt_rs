use crate::payload::*;

#[derive(Debug, Clone)]
pub struct Packet {
    pub seq_num: u32,
    pub pkt_type: PacketType,
    pub payload: Payload,
    pub checksum: u8,
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
        tmp.checksum = tmp.create_checksum();
        tmp
    }

    pub fn ack(seq_num: u32) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::Acknowlodge,
            payload: Payload::new([0; 5]),
            checksum: 0,
        };
        pkt.checksum = pkt.create_checksum();
        pkt
    }

    pub fn nack(seq_num: u32) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::NotAcklodge,
            payload: Payload::new([0; 5]),
            checksum: 0,
        };
        pkt.checksum = pkt.create_checksum();
        pkt
    }

    pub fn data(seq_num: u32, payload: Payload) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::Data,
            payload,
            checksum: 0,
        };
        pkt.checksum = pkt.create_checksum();
        pkt
    }

    pub fn create_checksum(&self) -> u8 {
        let mut sum = self.checksum_not_flip();
        sum = !sum;
        sum
    }

    pub fn checksum_ok(&self) -> bool {
        let result = self.checksum.overflowing_add(self.checksum_not_flip()).0;
        result == 255
    }

    pub fn corrupt_headers(&mut self) {
        self.checksum = self.checksum.overflowing_add(128).0;
    }

    fn checksum_not_flip(&self) -> u8 {
        let mut sum: u8 = 0;
        let mut overflow: u8 = 0;
        for element in self.payload.content.iter() {
            // This returns a tuple of (OF_sum, sum_OF_flag)
            let tuple = sum.overflowing_add(*element);
            if tuple.1 {
                overflow += 1;
            }
            sum = tuple.0;
        }
        sum += overflow;
        sum
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
