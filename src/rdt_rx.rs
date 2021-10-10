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
    data_buff: Vec<u32>,
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
            udt_layer: UnreliableDataTransport::new(tx, rx, "RX->TX"),
            data_buff: vec![],
        };
        rdt
    }
    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvError> {
        self.state = self.next_state;
        match self.state {
            RdtRXState::Waiting => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.pkt_type == PacketType::Acknowlodge {
                    self.data_buff.push(pkt.pkt_data);
                    self.next_state = RdtRXState::SendAck;
                    println!(
                        "[RDT] - {} - RX     - Received Server's Payload",
                        pkt.seq_num
                    );
                } else {
                    println!(
                        "[RDT] - {} - RX     - Received Garbage from Server",
                        pkt.seq_num
                    );
                    self.next_state = RdtRXState::SendNack;
                }
            }
            RdtRXState::SendAck => {
                let pkt = Packet::ack(self.seq_num, self.ack_num);
                self.ack_num += 1;
                self.next_state = RdtRXState::Waiting;
                self.udt_layer.maybe_send(&pkt);
            }
            RdtRXState::SendNack => {
                let pkt = Packet::nack(self.seq_num, self.ack_num);
                self.next_state = RdtRXState::Waiting;
                println!("[RDT] - {} - RX     - Sending NACK to Server", pkt.seq_num);
                self.udt_layer.maybe_send(&pkt)
            }
        }
        return Ok(());
    }
    pub fn get_data(&self) -> Vec<u32> {
        self.data_buff.clone()
    }
}
