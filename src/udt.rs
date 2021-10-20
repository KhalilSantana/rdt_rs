use std::sync::mpsc::{Receiver, Sender};

use crate::packet::Packet;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::io::{stdout, Write};
use std::time::Duration;
use crate::messages::messages_udt::messages_receiver::{log_message_udt_sent, log_message_udt_corrupt, log_message_udt_loss, log_message_udt_delay};

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
            0..=10 => log_message_udt_loss(packet.sequence_number, self.label),

            11..=19 => {
                let delay = self.rng.gen_range(490..750);
                std::thread::sleep(Duration::from_millis(delay));
                log_message_udt_delay(packet.sequence_number, self.label, &delay);
                self.send(packet);
            }
            20..=29 => {
                log_message_udt_corrupt(packet.sequence_number, self.label);
                stdout().flush();
                let mut packet2 = packet.clone();
                packet2.corrupt_headers();
                self.send(&packet2);
            }
            _ => {
                self.send(packet);
                log_message_udt_sent(packet.sequence_number, self.label);
                stdout().flush();
            }
        };
    }
}
