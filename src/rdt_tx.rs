use crate::packet::*;
use crate::payload::*;
use crate::udt::UnreliableDataTransport;
use std::io::{stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransportTX {
    state: RdtTXState,
    next_state: RdtTXState,
    seq_num: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<Payload>,
    is_done: bool,
    label: &'static str,
}
#[derive(Debug, Clone, Copy)]
pub enum RdtTXState {
    SendData,
    WaitingZero,
    WaitingOne,
}
impl ReliableDataTransportTX {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>, data_buff: &[u8]) -> Self {
        ReliableDataTransportTX {
            state: RdtTXState::SendData,
            next_state: RdtTXState::WaitingZero,
            seq_num: 0,
            udt_layer: UnreliableDataTransport::new(tx, rx, "TX->RX"),
            data_buff: crate::payload::split_data(data_buff),
            is_done: false,
            label: "TX->RX",
        }
    }
    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvError> {
        match self.state {
            RdtTXState::WaitingZero => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.pkt_type == PacketType::Acknowlodge && pkt.seq_num == 0
                {
                    println!(
                        "[RDT] - {} - TX     - Received Client's Ack Zero",
                        pkt.seq_num
                    );
                    stdout().flush();
                    self.data_buff.remove(0);
                    self.seq_num = 1;
                    self.next_state = RdtTXState::WaitingOne;
                    send_data(self);
                } else {
                    println!(
                        "[RDT] - {} - TX     - Failed.. retransmiting last - {}",
                        self.seq_num,
                        self.data_buff.first().unwrap()
                    );
                    stdout().flush();
                    send_data(self);
                }
            }
            RdtTXState::WaitingOne => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.pkt_type == PacketType::Acknowlodge && pkt.seq_num == 1
                {
                    println!(
                        "[RDT] - {} - TX     - Received Client's Ack One",
                        pkt.seq_num
                    );
                    stdout().flush();
                    self.data_buff.remove(0);
                    self.seq_num = 0;
                    self.next_state = RdtTXState::WaitingZero;
                    send_data(self);
                } else {
                    println!(
                        "[RDT] - {} - TX     - Failed.. retransmit - {}",
                        self.seq_num,
                        self.data_buff.first().unwrap()
                    );
                    stdout().flush();
                    send_data(self);
                }
            }
            RdtTXState::SendData => send_data(self),
        }
        self.state = self.next_state;
        Ok(())
    }
    pub fn is_done(&self) -> bool {
        self.is_done
    }
    fn set_done(&mut self) {
        println!("[RDT] == Entire data buffer sent, quitting ==");
        stdout().flush();
        self.is_done = true;
    }
}

fn send_data(rdt_tx: &mut ReliableDataTransportTX) {
    if rdt_tx.data_buff.is_empty() {
        rdt_tx.set_done();
        return;
    }
    let pkt = Packet::data(rdt_tx.seq_num, *rdt_tx.data_buff.first().unwrap());
    println!(
        "[RDT] - {} - TX     - Sending - {}",
        pkt.seq_num, pkt.payload
    );
    stdout().flush();
    rdt_tx.udt_layer.maybe_send(&pkt)
}
