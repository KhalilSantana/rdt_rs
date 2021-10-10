#[derive(Debug, Clone)]
pub struct Packet {
    pub seq_num: u32,
    pub pkt_type: PacketType,
    pub pkt_data: u32,
    pub checksum: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub enum PacketType {
    Acknowlodge,
    NotAcklodge,
}
impl Packet {
    pub fn new(seq_num: u32, pkt_type: PacketType, pkt_data: u32) -> Self {
        let mut tmp = Packet {
            seq_num,
            pkt_type,
            pkt_data,
            checksum: 0,
        };
        tmp.create_checksum();
        tmp
    }
    pub fn ack(seq_num: u32) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::Acknowlodge,
            pkt_data: 0,
            checksum: 0,
        };
        pkt.create_checksum();
        pkt
    }
    pub fn nack(seq_num: u32) -> Self {
        let mut pkt = Packet {
            seq_num,
            pkt_type: PacketType::NotAcklodge,
            pkt_data: 0,
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
    pub fn corrupt_data(&mut self) {
        self.pkt_data = self.pkt_data.reverse_bits();
    }
    pub fn corrupt_headers(&mut self) {
        self.checksum = 1337;
    }
}
