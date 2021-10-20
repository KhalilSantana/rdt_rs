#![allow(unused)]

pub mod messages_receiver {
    use crate::packet::Packet;
    use chrono::{DateTime, Local, NaiveTime};

    pub fn log_message_udt_corrupt(sequence_number: u32, label: &str)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - UDT] - SeqNum: {} - {} - Corrupt Checksum", dt, sequence_number, label)
    }

    pub fn log_message_udt_sent(sequence_number: u32, label: &str)  {
        let dt: NaiveTime = Local::now().time();
        println!("[{} - UDT] - SeqNum: {} - {} - Sent", dt, sequence_number, label)
    }
}