#![allow(unused_must_use, dead_code)]
mod packet;
mod payload;
mod rdt_receiver;
mod rdt_transmitter;
mod udt;
mod enums;

use crate::rdt_receiver::ReliableDataTransportReceiver;
use crate::rdt_transmitter::ReliableDataTransportTransmitter;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let data_text = "Hello World!!!!!!!!!";
    let data = data_text.as_bytes();
    println!("\nServer sending data: {}", data_text);
    println!("Server sending data bytes: {:?}", data);
    let (tx_sender, rx_receiver) = channel();
    let (tx_receiver, rx_sender) = channel();
    /* Thread Alice */
    let thread_sender= thread::spawn(move || {
        let mut rdt_transmitter = ReliableDataTransportTransmitter::new(tx_sender, rx_sender, 10);

        let mut buffed_data = crate::payload::split_data(data);

        while !buffed_data.is_empty() {
            if rdt_transmitter.next(*buffed_data.first().unwrap()).is_err() {
                return;
            };
            buffed_data.remove(0);
        }

        println!("\n[RDT] == Entire data buffer sent, quitting ==");
    });
    /* Thread Bob */
    let thread_receiver= thread::spawn(move || {
        let mut rdt_receiver = ReliableDataTransportReceiver::new(tx_receiver, rx_receiver, 42);
        loop {
            if rdt_receiver.next().is_err() {
                break;
            }
        }
        println!("\nClient got data: {:?}", rdt_receiver.get_data());
        println!(
            "UTF-8: {}",
            std::str::from_utf8(&rdt_receiver.get_data()).expect("Parse error!")
        );
    });
    thread_sender.join().unwrap();
    thread_receiver.join().unwrap();
}
