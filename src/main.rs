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
use rdt_rx::ReceiveData;

use crate::packet::*;
use crate::rdt_rx::ReliableDataTransportRX;
use crate::rdt_tx::ReliableDataTransport;
use crate::rdt_tx::SendData;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
fn main() {
    let data = "Hello World!".as_bytes();
    let pkt_data = rdt_tx::split_input_data(data);
    let (tx_sender, rx_receiver) = channel();
    let (tx_receiver, rx_sender) = channel();
    let rdt_tx = ReliableDataTransport::new(tx_sender, rx_receiver);
    let t0 = thread::spawn(move || {
        let mut in_send = ReliableDataTransport::<SendData>::from(rdt_tx);
        in_send.send(&data);
        dbg!("{}", in_send);
    });
    let t1 = thread::spawn(move || {
        let rdt_rx = ReliableDataTransportRX::new(tx_receiver, rx_sender);
        let mut in_receive = ReliableDataTransportRX::<ReceiveData>::from(rdt_rx);
        match in_receive.receive() {
            Err(..) => println!("Some error"),
            Ok(d) => println!("Got data {}", d),
        }
    });
    t0.join().unwrap();
    t1.join().unwrap();
}
