use crate::packet::*;
use crate::payload::*;
use crate::udt::UnreliableDataTransport;
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransportRX {
    state: RdtRXState,
    next_state: RdtRXState,
    seq_num: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<Payload>,
}
#[derive(Debug, Clone, Copy)]
pub enum RdtRXState {
    WaitingZero,
    WaitingOne,
}
impl ReliableDataTransportRX {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        ReliableDataTransportRX {
            state: RdtRXState::WaitingZero,
            next_state: RdtRXState::WaitingZero,
            seq_num: 0,
            udt_layer: UnreliableDataTransport::new(tx, rx, "RX->TX"),
            data_buff: vec![],
        }
    }
    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvError> {
        match self.state {
            RdtRXState::WaitingZero => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.seq_num == 0 {
                    self.data_buff.push(pkt.payload);
                    println!(
                        "[RDT] - {} - RX     - Received Server's Payload",
                        pkt.seq_num
                    );
                    send_response(self, PacketType::Acknowlodge, self.seq_num);
                    self.seq_num = 1;
                    self.next_state = RdtRXState::WaitingOne;
                }
                if !pkt.checksum_ok() {
                    println!(
                        "[RDT] - {} - RX     - Received Garbage from Server",
                        pkt.seq_num
                    );
                    send_response(self, PacketType::NotAcklodge, self.seq_num)
                }
                if pkt.checksum_ok() && pkt.seq_num == 1 {
                    println!(
                        "[RDT] - {} - RX     - Received DUP from server..",
                        self.seq_num
                    );
                    send_response(self, PacketType::Acknowlodge, 1)
                }
            }
            RdtRXState::WaitingOne => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.seq_num == 1 {
                    self.data_buff.push(pkt.payload);
                    println!(
                        "[RDT] - {} - RX     - Received Server's Payload",
                        pkt.seq_num
                    );
                    self.seq_num = 0;
                    self.next_state = RdtRXState::WaitingZero;
                    send_response(self, PacketType::Acknowlodge, 1);
                }
                if !pkt.checksum_ok() {
                    println!(
                        "[RDT] - {} - RX     - Received Garbage from Server",
                        pkt.seq_num
                    );
                    send_response(self, PacketType::NotAcklodge, 1)
                }
                if pkt.checksum_ok() && pkt.seq_num == 0 {
                    println!(
                        "[RDT] - {} - RX     - Received DUP from server..",
                        self.seq_num
                    );
                    send_response(self, PacketType::Acknowlodge, 0)
                }
            }
        }
        self.state = self.next_state;
        Ok(())
    }
    pub fn get_data(&self) -> Vec<Payload> {
        self.data_buff.clone()
    }
}

fn send_response(rdt_rx: &mut ReliableDataTransportRX, pkt_type: PacketType, seq_num: u32) {
    match pkt_type {
        PacketType::Acknowlodge => {
            println!("[RDT] - {} - RX     - Sending Ack {}", seq_num, seq_num);
            rdt_rx.udt_layer.maybe_send(&Packet::ack(seq_num));
        }
        PacketType::NotAcklodge => {
            println!("[RDT] - {} - RX     - Sending NACK {}", seq_num, seq_num);
            rdt_rx.udt_layer.maybe_send(&Packet::nack(seq_num));
        }
        _ => unreachable!("Client should never send other packet types!"),
    }
}
