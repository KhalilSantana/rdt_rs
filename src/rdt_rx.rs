use crate::packet::*;
use crate::udt::UnreliableDataTransport;
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransportRX {
    state: RdtRXState,
    next_state: RdtRXState,
    seq_num: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<u8>,
}
#[derive(Debug, Clone, Copy)]
pub enum RdtRXState {
    WaitingZero,
    WaitingOne,
}
impl ReliableDataTransportRX {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        let rdt = ReliableDataTransportRX {
            state: RdtRXState::WaitingZero,
            next_state: RdtRXState::WaitingZero,
            seq_num: 0,
            udt_layer: UnreliableDataTransport::new(tx, rx, "RX->TX"),
            data_buff: vec![],
        };
        rdt
    }
    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvError> {
        match self.state {
            RdtRXState::WaitingZero => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.seq_num == 0 {
                    self.data_buff.push(pkt.pkt_data);
                    println!(
                        "[RDT] - {} - RX     - Received Server's Payload",
                        pkt.seq_num
                    );
                    println!("[RDT] - {} - RX     - Sending ACK Zero", self.seq_num);
                    send_response(self, PacketType::Acknowlodge, self.seq_num);
                    self.seq_num = 1;
                    self.next_state = RdtRXState::WaitingOne;
                }
                if !pkt.checksum_ok() {
                    println!(
                        "[RDT] - {} - RX     - Received Garbage from Server",
                        pkt.seq_num
                    );
                    println!("[RDT] - {} - RX     - Sending NACK Zero", self.seq_num);
                    send_response(self, PacketType::NotAcklodge, self.seq_num)
                }
                if !pkt.checksum_ok() && pkt.seq_num == 1 {
                    send_response(self, PacketType::Acknowlodge, 1)
                }
            }
            RdtRXState::WaitingOne => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.seq_num == 1 {
                    self.data_buff.push(pkt.pkt_data);
                    println!(
                        "[RDT] - {} - RX     - Received Server's Payload",
                        pkt.seq_num
                    );
                    println!("[RDT] - {} - RX     - Sending ACK One", self.seq_num);
                    self.seq_num = 0;
                    self.next_state = RdtRXState::WaitingZero;
                    send_response(self, PacketType::Acknowlodge, 1);
                } else {
                    println!(
                        "[RDT] - {} - RX     - Received Garbage from Server",
                        pkt.seq_num
                    );
                    println!("[RDT] - {} - RX     - Sending NACK One", self.seq_num);
                    let response = Packet::nack(self.seq_num);
                    self.udt_layer.maybe_send(&response);
                }
            }
        }
        self.state = self.next_state;
        return Ok(());
    }
    pub fn get_data(&self) -> Vec<u8> {
        self.data_buff.clone()
    }
}

fn send_response(rdt_rx: &mut ReliableDataTransportRX, pkt_type: PacketType, seq_num: u32) {
    match pkt_type {
        Acknowlodge => rdt_rx.udt_layer.maybe_send(&Packet::ack(seq_num)),
        NotAcklodge => rdt_rx.udt_layer.maybe_send(&Packet::nack(seq_num)),
    }
}
