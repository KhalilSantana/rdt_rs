use std::sync::mpsc::{Receiver, Sender};

use crate::packet::Packet;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct UnreliableDataTransport {
    transmitter: Sender<Packet>,
    receiver: Receiver<Packet>,
    label: &'static str,
    rng: rand_chacha::ChaCha8Rng,
}

impl UnreliableDataTransport {
    pub fn new(
        transmitter: Sender<Packet>,
        receiver: Receiver<Packet>,
        label: &'static str,
        rng_seed: u64,
    ) -> Self {
        Self {
            transmitter,
            receiver,
            label,
            rng: ChaCha8Rng::seed_from_u64(rng_seed),
        }
    }

    pub fn send(&self, pkt: &Packet) {
        self.transmitter.send(pkt.clone());
    }

    pub fn receive(&self) -> Result<Packet, std::sync::mpsc::RecvError> {
        let response = self.receiver.recv()?;
        Ok(response)
    }

    pub fn maybe_send(&mut self, packet: &Packet) {
        let _ = match self.rng.gen_range(0..100) {
            //TODO: simular atraso de pacote
           // thread::sleep(time::Duration::from_millis(10));

            //0..=10 => println!("{} - Loss", pkt.seq_num),
            //11..=19 => println!("{} - Corrupt data", pkt.seq_num),
            20..=29 => {
                println!(
                    "\n[UDT] - SeqNum: {} - {} - Corrupt Checksum",
                    packet.sequence_number, self.label
                );
                stdout().flush();
                let mut packet2 = packet.clone();
                packet2.corrupt_headers();
                self.send(&packet2);
            }
            _ => {
                self.send(packet);
                println!("[UDT] - SeqNum: {} - {} - Sent", packet.sequence_number, self.label);
                stdout().flush();
            }
        };
    }
}
