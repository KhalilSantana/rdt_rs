#![allow(unused)]

pub mod messages_receiver {
    use crate::packet::Packet;

    pub fn log_message_receiver_payload(sequence_number: u32) {
        println!(
            "[RDT] - SeqNum: {} - RECEIVER    -  Received Server's Payload",
            sequence_number
        )
    }

    pub fn log_message_receiver_garbage(sequence_number: u32) {
        println!(
            "[RDT] - SeqNum: {} - RECEIVER    -  Received Garbage from Server",
            sequence_number
        )
    }

    pub fn log_message_receiver_dup(sequence_number: u32) {
        println!(
            "[RDT] - SeqNum: {} - RECEIVER    -  Received DUP from server..",
            sequence_number
        )
    }

    pub fn log_message_receiver_sending_ack(sequence_number: u32, packet: &Packet) {
        println!(
            "[RDT] - SeqNum: {} - RECEIVER    -  Sending ACK {}",
            sequence_number, packet
        )
    }

    pub fn log_message_receiver_sending_nack(sequence_number: u32, packet: &Packet) {
        println!(
            "[RDT] - SeqNum: {} - RECEIVER    -  Sending NACK {}",
            sequence_number, packet
        );
    }

    pub fn log_message_receiver_timeout(sequence_number: u32) {
        println!(
            "[RDT] - SeqNum: {} - RECEIVER    - Timed-out waiting for response..",
            sequence_number
        );
    }
}
