#![allow(unused)]

pub mod messages_transmitter {
    use crate::packet::Packet;
    use crate::payload::Payload;

    pub fn log_message_transmitter_received_ack(sequence_number: usize) {
        let mut texts: Vec<String> = vec![];
        texts.push(String::from("ZERO"));
        texts.push(String::from("ONE"));
        println!(
            "[RDT] - SeqNum: {} - TRANSMITTER -  Received Client's ACK {}",
            sequence_number,
            texts.get(sequence_number).unwrap()
        );
    }

    pub fn log_message_transmitter_failed(sequence_number: u32, buffer: Payload) {
        println!(
            "[RDT] - SeqNum: {} - TRANSMITTER -  Failed.. retransmiting last - {}",
            sequence_number, buffer
        );
    }

    pub fn log_message_transmitter_sending(sequence_number: u32, packet: &Packet) {
        println!(
            "\n[RDT] - SeqNum: {} - TRANSMITTER -  Sending - {}",
            sequence_number, packet
        );
    }
    pub fn log_message_transmitter_timeout(sequence_number: u32) {
        println!(
            "[RDT] - SeqNum: {} - TRANSMITTER    - Timed-out waiting for response..",
            sequence_number
        );
    }
}
