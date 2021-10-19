#![allow(unused)]

pub mod messages_transmitter {
    use crate::packet::Packet;
    use crate::payload::Payload;

    pub fn log_message_transmitter_received_ack_zero (sequence_number: u32)  {
        println!( "[RDT] - SeqNum: {} - TRANSMITTER -  Received Client's ACK Zero", sequence_number)
    }

    pub fn log_message_transmitter_received_ack_one (sequence_number: u32)  {
        println!( "[RDT] - SeqNum: {} - TRANSMITTER -  Received Client's ACK One", sequence_number)
    }

    pub fn log_message_transmitter_failed (sequence_number: u32, buffer: &Payload)  {
        println!("[RDT] - SeqNum: {} - TRANSMITTER -  Failed.. retransmiting last - {}", sequence_number, buffer);
    }

    pub fn log_message_transmitter_sending (sequence_number: u32, packet: &Packet)  {
        println!("\n[RDT] - SeqNum: {} - TRANSMITTER -  Sending - {}", sequence_number, packet);
    }
}



