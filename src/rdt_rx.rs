use crate::packet::*;
use crate::udt::UnreliableDataTransport;
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransportRX {
    state: RdtRXState,
    next_state: RdtRXState,
    seq_num: u32,
    ack_num: u32,
    udt_layer: UnreliableDataTransport,
}
#[derive(Debug, Clone, Copy)]
pub enum RdtRXState {
    Waiting,
    //    ReceiveData,
    SendAck,
    SendNack,
}
impl ReliableDataTransportRX {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        let rdt = ReliableDataTransportRX {
            state: RdtRXState::Waiting,
            next_state: RdtRXState::Waiting,
            seq_num: 0,
            ack_num: 0,
            udt_layer: UnreliableDataTransport::new(tx, rx),
        };
        rdt
    }
    pub fn next(&mut self) {
        dbg!(&self);
        self.state = self.next_state;
        match self.state {
            RdtRXState::Waiting => {
                let pkt = self.udt_layer.receive();
                match (pkt.checksum_ok(), pkt.pkt_type) {
                    (true, PacketType::Acknowlodge) => {
                        dbg!("[RX] Got packet data {?}", pkt.pkt_data);
                    }
                    (_, _) => {
                        self.next_state = RdtRXState::SendNack;
                    }
                }
            }
            RdtRXState::SendAck => {
                let pkt = Packet::ack(self.seq_num, self.ack_num);
                self.ack_num += 1;
                self.udt_layer.send(&pkt);
                self.next_state = RdtRXState::Waiting;
            }
            RdtRXState::SendNack => {
                let pkt = Packet::nack(self.seq_num, self.ack_num);
                self.udt_layer.send(&pkt)
            }
        }
    }
}
