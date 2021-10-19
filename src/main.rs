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
    let data = "Hello World!".as_bytes();
    //let data = "Hello".as_bytes();
    println!("\nServer sending data: {:?}", data);
    let (tx_sender, rx_receiver) = channel();
    let (tx_receiver, rx_sender) = channel();
    let t0 = thread::spawn(move || {
        let mut rdt_tx = ReliableDataTransportTransmitter::new(tx_sender, rx_sender, data, 10);
        while !rdt_tx.is_done() {
            if rdt_tx.next().is_err() {
                return;
            };
        }
    });
    let t1 = thread::spawn(move || {
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
    t0.join().unwrap();
    t1.join().unwrap();
}
