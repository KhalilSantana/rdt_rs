#![allow(unused)]

pub mod messages_transmitter {
    use crate::packet::Packet;
    use crate::payload::Payload;
    use chrono::{DateTime, Local, NaiveTime};

    pub fn log_message_transmitter_received_ack(sequence_number: usize)  {
        let mut texts: Vec<String> = vec![];
        let dt: NaiveTime = Local::now().time();

        texts.push(String::from("ZERO"));
        texts.push(String::from("ONE"));
        println!( "[{} - RDT] - SeqNum: {} - TRANSMITTER -  Received Client's ACK {}", dt,sequence_number, texts.get(sequence_number).unwrap());
    }

    pub fn log_message_transmitter_failed (sequence_number: u32, buffer: Payload)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - RDT] - SeqNum: {} - TRANSMITTER -  Failed.. retransmiting last - {}", dt, sequence_number, buffer);
    }

    pub fn log_message_transmitter_sending (sequence_number: u32, packet: &Packet)  {
        let dt: NaiveTime = Local::now().time();
        println!("\n[{} - RDT] - SeqNum: {} - TRANSMITTER -  Sending - {{", dt, sequence_number);
        println!("                             Sequence_number - {}",  packet.sequence_number);
        println!("                             Packet_type - {}" ,  packet.packet_type);
        println!("                             Checksum - {}" ,  packet.checksum);
        println!("                             Payload - {}" ,  packet.payload);
        println!("                           }}\n");
    }
}



