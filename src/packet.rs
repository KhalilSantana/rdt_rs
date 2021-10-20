use crate::payload::*;

#[derive(Debug, Clone)]
pub struct Packet {
    pub sequence_number: u32,
    pub nack_number: u32,
    pub packet_type: PacketType,
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
    pub fn new(sequence_number: u32, packet_type: PacketType, payload: Payload) -> Self {
        let mut variable_temporary = Packet {
            sequence_number,
            nack_number: sequence_number,
            packet_type,
            payload,
            checksum: 0,
        };
        variable_temporary.checksum = variable_temporary.create_checksum();
        variable_temporary
    }

    pub fn ack(sequence_number: u32) -> Self {
        let mut packet = Packet {
            sequence_number,
            nack_number: sequence_number,
            packet_type: PacketType::Acknowlodge,
            payload: Payload::new([0; 5]),
            checksum: 0,
        };
        packet.checksum = packet.create_checksum();
        packet
    }

    pub fn nack(nack_number: u32, ack_number: u32) -> Self {
        let mut packet = Packet {
            sequence_number: ack_number,
            nack_number,
            packet_type: PacketType::Acknowlodge,
            payload: Payload::new([0; 5]),
            checksum: 0,
        };
        packet.checksum = packet.create_checksum();
        packet
    }

    pub fn data(sequence_number: u32, payload: Payload) -> Self {
        let mut packet = Packet {
            sequence_number,
            nack_number: sequence_number,
            packet_type: PacketType::Data,
            payload,
            checksum: 0,
        };
        packet.checksum = packet.create_checksum();
        packet
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
