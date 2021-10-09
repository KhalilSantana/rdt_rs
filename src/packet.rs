#[derive(Debug, Clone)]
pub struct Packet {
    pub seq_num: u32,
    pub ack_num: u32,
    pub is_ack: bool,
    pub pkt_data: u32,
    pub checksum: u32,
}
impl Packet {
    pub fn new(seq_num: u32, ack_num: u32, pkt_data: u32) -> Self {
        let mut tmp = Packet {
            seq_num,
            ack_num,
            is_ack: true,
            pkt_data,
            checksum: 0,
        };
        tmp.create_checksum();
        tmp
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
