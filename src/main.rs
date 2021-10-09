#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_assignments,
    unused_mut
)]

mod packet;
mod rdt_rx;
mod rdt_tx;
mod udt;

use crate::packet::*;
use crate::rdt_rx::ReliableDataTransportRX;
use crate::rdt_tx::ReliableDataTransportTX;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
fn main() {
    let data = "Hello World!".as_bytes();
    let pkt_data = rdt_tx::split_input_data(data);
    let (tx_sender, rx_receiver) = channel();
    let (tx_receiver, rx_sender) = channel();
    let t0 = thread::spawn(move || {
        let mut rdt_tx =
            ReliableDataTransportTX::new(tx_sender, rx_receiver, rdt_tx::split_input_data(data));
        loop {
            rdt_tx.next();
        }
    });
    let t1 = thread::spawn(move || {
        let mut rdt_rx = ReliableDataTransportRX::new(tx_receiver, rx_sender);
        loop {
            rdt_rx.next();
        }
    });
    t0.join().unwrap();
    t1.join().unwrap();
}
