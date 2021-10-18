#![allow(unused_must_use, dead_code)]
mod packet;
mod payload;
mod rdt_rx;
mod rdt_tx;
mod udt;

use crate::rdt_rx::ReliableDataTransportRX;
use crate::rdt_tx::ReliableDataTransportTX;
use std::sync::mpsc::channel;
use std::thread;
fn main() {
    let data = "Hello World!".as_bytes();
    //let data = "Hello".as_bytes();
    println!("Server sending data {:?}", data);
    let (tx_sender, rx_receiver) = channel();
    let (tx_receiver, rx_sender) = channel();
    let t0 = thread::spawn(move || {
        let mut rdt_tx = ReliableDataTransportTX::new(tx_sender, rx_sender, data);
        while !rdt_tx.is_done() {
            if rdt_tx.next().is_err() {
                return;
            };
        }
    });
    let t1 = thread::spawn(move || {
        let mut rdt_rx = ReliableDataTransportRX::new(tx_receiver, rx_receiver);
        loop {
            if rdt_rx.next().is_err() {
                break;
            }
        }
        println!("Client got data {:?}", rdt_rx.get_data());
        println!(
            "UTF-8: {}",
            std::str::from_utf8(&rdt_rx.get_data()).expect("Parse error!")
        );
    });
    t0.join().unwrap();
    t1.join().unwrap();
}
