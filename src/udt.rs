use std::sync::mpsc::{Receiver, Sender};

use crate::packet::Packet;
use rand::Rng;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct UnreliableDataTransport {
    tx: Sender<Packet>,
    rx: Receiver<Packet>,
    label: &'static str,
}
impl UnreliableDataTransport {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>, label: &'static str) -> Self {
        Self { tx, rx, label }
    }
    pub fn send(&self, pkt: &Packet) {
        self.tx.send(pkt.clone());
    }
    pub fn receive(&self) -> Result<Packet, std::sync::mpsc::RecvError> {
        let response = self.rx.recv()?;
        Ok(response)
    }

    pub fn maybe_send(&self, pkt: &Packet) {
        let rand = rand::thread_rng().gen_range(0..100);
        let _ = match rand {
            //0..=10 => println!("{} - Loss", pkt.seq_num),
            //11..=19 => println!("{} - Corrupt data", pkt.seq_num),
            // 20..=29 => {
            //     println!(
            //         "[UDT] - {} - {} - Corrupt Checksum",
            //         pkt.seq_num, self.label
            //     );
            //     stdout().flush();
            //     let mut pkt2 = pkt.clone();
            //     pkt2.corrupt_headers();
            //     println!("{}", pkt2.checksum_ok());
            //     self.send(&pkt2);
            // }
            _ => {
                self.send(pkt);
                println!("[UDT] - {} - {} - Sent", pkt.seq_num, self.label);
                stdout().flush();
            }
        };
    }
}
