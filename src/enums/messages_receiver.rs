#![allow(unused)]

pub mod messages_receiver {
    use crate::packet::Packet;
    use chrono::{DateTime, Local, NaiveTime};

    pub fn log_message_receiver_payload(sequence_number: u32)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - RDT] - SeqNum: {} - RECEIVER    -  Received Server's Payload",dt, sequence_number)
    }

    pub fn log_message_receiver_garbage(sequence_number: u32)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - RDT] - SeqNum: {} - RECEIVER    -  Received Garbage from Server", dt, sequence_number)
    }

    pub fn log_message_receiver_dup(sequence_number: u32)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - RDT] - SeqNum: {} - RECEIVER    -  Received DUP from server..", dt, sequence_number)
    }

    pub fn log_message_receiver_sending_ack(sequence_number: u32, packet: &Packet)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - RDT] - SeqNum: {} - RECEIVER    -  Sending ACK - {{", dt, sequence_number);
        println!("                             Sequence_number - {}",  packet.sequence_number);
        println!("                             Packet_type - {}" ,  packet.packet_type);
        println!("                             Checksum - {}" ,  packet.checksum);
        println!("                             Payload - {}" ,  packet.payload);
        println!("                           }}\n");
    }

    pub fn log_message_receiver_sending_nack(sequence_number: u32, packet: &Packet)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - RDT] - SeqNum: {} - RECEIVER    -  Sending NACK - {{", dt, sequence_number);
        println!("                             Sequence_number - {}",  packet.sequence_number);
        println!("                             Packet_type - {}" ,  packet.packet_type);
        println!("                             Checksum - {}" ,  packet.checksum);
        println!("                             Payload - {}" ,  packet.payload);
        println!("                           }}\n");
    }
}



