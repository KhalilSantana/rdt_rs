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
#[derive(Clone, Copy, Debug)]
pub struct Payload {
    content: [u8; 5],
    padding: u8,
}

impl Payload {
    pub fn new(content: [u8; 5]) -> Self {
        Payload {
            content,
            padding: 0,
        }
    }
}

impl std::fmt::Display for Payload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "".to_owned();
        for i in 0..self.padding as usize {
            output.push_str(&self.content[i].to_string());
        }
        write!(f, "{}", output)
    }
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

pub fn split_data(data: &[u8]) -> Vec<Payload> {
    // TODO: Don't hardcode 5 here
    let mut chunks = data.chunks_exact(5);
    let mut output: Vec<Payload> = vec![];
    for e in &mut chunks {
        output.push(Payload::new([e[0], e[1], e[2], e[3], e[4]]))
    }
    // Padding for the last payload
    let remainder = chunks.remainder();
    let mut last_payload = [0; 5];
    for i in 0..remainder.len() {
        last_payload[i] = remainder[i];
    }
    output.push(Payload {
        content: last_payload,
        padding: (5 - remainder.len()) as u8,
    });
    output
}
