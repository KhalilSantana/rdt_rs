use std::sync::mpsc::{Receiver, Sender};

use crate::packet::Packet;
use rand::Rng;

#[derive(Debug)]
pub struct UnreliableDataTransport {
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
}
impl UnreliableDataTransport {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        Self { tx, rx }
    }
    pub fn send(&self, pkt: &Packet) {
        dbg!(&pkt);
        self.tx.send(pkt.clone());
    }
    pub fn receive(&self) -> Result<Packet, std::sync::mpsc::RecvError> {
        let response = self.rx.recv()?;
        //        dbg!(&response);
        Ok(response)
    }

    pub fn maybe_send(&self, pkt: &Packet) {
        let rand = rand::thread_rng().gen_range(0..100);
        let _ = match rand {
            0..=10 => println!("{} - Loss", pkt.seq_num),
            11..=19 => println!("{} - Corrupt data", pkt.seq_num),
            20..=29 => println!("{} - Corrupt headers", pkt.seq_num),
            _ => {
                self.send(pkt);
                println!("{} - Sent", pkt.seq_num)
            }
        };
    }
    pub fn maybe_recieve(&self) -> Result<Packet, std::sync::mpsc::RecvError> {
        self.receive()
    }
}
