use std::sync::mpsc::{Receiver, Sender};

use crate::packet::Packet;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::io::{stdout, Write};
use std::time::Duration;

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

    pub fn send(&self, packet: &Packet) {
        self.transmitter.send(packet.clone());
    }

    pub fn receive(&self) -> Result<Packet, std::sync::mpsc::RecvError> {
        let response = self.receiver.recv()?;
        Ok(response)
    }

    pub fn maybe_receive(&mut self) -> Result<Packet, std::sync::mpsc::RecvTimeoutError> {
        let response = self.receiver.recv_timeout(Duration::from_millis(500))?;
        Ok(response)
    }

    pub fn maybe_send(&mut self, packet: &Packet) {
        let _ = match self.rng.gen_range(0..100) {
            0..=10 => println!("[UDT] - SeqNum: {} - {} - Loss", packet.sequence_number, self.label),
            11..=19 => {
                println!("[UDT] - SeqNum: {} - {} - Delay data", packet.sequence_number, self.label);
                let delay = self.rng.gen_range(200..750);
                    std::thread::sleep(Duration::from_millis(delay));
                
                },
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
