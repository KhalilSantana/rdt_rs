use std::sync::mpsc::{Receiver, Sender};

use crate::packet::Packet;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct UnreliableDataTransport {
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
    label: &'static str,
    rng: rand_chacha::ChaCha8Rng,
}
impl UnreliableDataTransport {
    pub fn new(
        tx: Sender<Packet>,
        rx: Receiver<Packet>,
        label: &'static str,
        rng_seed: u64,
    ) -> Self {
        Self {
            tx,
            rx,
            label,
            rng: ChaCha8Rng::seed_from_u64(rng_seed),
        }
    }
    pub fn send(&self, pkt: &Packet) {
        self.tx.send(pkt.clone());
    }
    pub fn receive(&self) -> Result<Packet, std::sync::mpsc::RecvError> {
        let response = self.rx.recv()?;
        Ok(response)
    }

    pub fn maybe_send(&mut self, pkt: &Packet) {
        let _ = match self.rng.gen_range(0..100) {
            //0..=10 => println!("{} - Loss", pkt.seq_num),
            //11..=19 => println!("{} - Corrupt data", pkt.seq_num),
            20..=29 => {
                println!(
                    "\n[UDT] - SeqNum: {} - {} - Corrupt Checksum",
                    pkt.seq_num, self.label
                );
                stdout().flush();
                let mut pkt2 = pkt.clone();
                pkt2.corrupt_headers();
                self.send(&pkt2);
            }
            _ => {
                self.send(pkt);
                println!("[UDT] - SeqNum: {} - {} - Sent", pkt.seq_num, self.label);
                stdout().flush();
            }
        };
    }
}
