use crate::packet::*;
use crate::udt::UnreliableDataTransport;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::io::{stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransportTX {
    state: RdtTXState,
    next_state: RdtTXState,
    seq_num: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<u32>,
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
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>, data_buff: Vec<u32>) -> Self {
        let rdt = ReliableDataTransportTX {
            state: RdtTXState::SendData,
            next_state: RdtTXState::WaitingZero,
            seq_num: 0,
            udt_layer: UnreliableDataTransport::new(tx, rx, "TX->RX"),
            data_buff,
            is_done: false,
            label: "TX->RX",
        };
        rdt
    }
    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvError> {
        match self.state {
            RdtTXState::WaitingZero => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.pkt_type == PacketType::Acknowlodge {
                    if pkt.seq_num == 1 {
                        println!(
                            "[RDT] - {} - TX      - Received DUP pkt from Client",
                            pkt.seq_num
                        );
                    }
                    println!("[RDT] - {} - TX     - Received Client's Ack", pkt.seq_num);
                    stdout().flush();
                    self.data_buff.remove(0);
                    self.seq_num = 1;
                    self.next_state = RdtTXState::WaitingOne;
                    println!(
                        "[RDT] - {} - TX     - Sending data to Client - {}",
                        pkt.seq_num, pkt.pkt_data
                    );
                    self.udt_layer.maybe_send(&Packet::data(
                        self.seq_num,
                        *self.data_buff.first().unwrap(),
                    ));
                    if self.data_buff.len() == 0 {
                        println!("[RDT] == Entire data buffer sent, quitting ==");
                        stdout().flush();
                        self.is_done = true;
                    }
                } else {
                    println!("[RDT] - {} - TX     - Failed.. retransmit", pkt.seq_num);
                    stdout().flush();
                    self.udt_layer.maybe_send(&Packet::data(
                        self.seq_num,
                        *self.data_buff.first().unwrap(),
                    ))
                }
            }
            RdtTXState::WaitingOne => {
                let pkt = self.udt_layer.receive()?;
                if pkt.checksum_ok() && pkt.pkt_type == PacketType::Acknowlodge {
                    if pkt.seq_num == 0 {
                        println!(
                            "[RDT] - {} - TX      - Received DUP pkt from Client",
                            pkt.seq_num
                        );
                    }
                    println!("[RDT] - {} - TX     - Received Client's Ack", pkt.seq_num);
                    stdout().flush();
                    self.data_buff.remove(0);
                    self.seq_num = 0;
                    self.next_state = RdtTXState::WaitingZero;
                    println!(
                        "[RDT] - {} - TX     - Sending data to Client - {}",
                        pkt.seq_num, pkt.pkt_data
                    );
                    self.udt_layer.maybe_send(&Packet::data(
                        self.seq_num,
                        *self.data_buff.first().unwrap(),
                    ));
                    if self.data_buff.len() == 0 {
                        println!("[RDT] == Entire data buffer sent, quitting ==");
                        stdout().flush();
                        self.is_done = true;
                    }
                } else {
                    println!("[RDT] - {} - TX     - Failed.. retransmit", pkt.seq_num);
                    stdout().flush();
                    self.udt_layer.maybe_send(&Packet::data(
                        self.seq_num,
                        *self.data_buff.first().unwrap(),
                    ))
                }
            }
            RdtTXState::SendData => {
                let pkt = Packet::new(
                    self.seq_num,
                    PacketType::Acknowlodge,
                    *self.data_buff.first().unwrap(),
                );
                println!(
                    "[RDT] - {} - TX     - Sending data to Client - {}",
                    pkt.seq_num, pkt.pkt_data
                );
                self.udt_layer.maybe_send(&pkt);
            }
        }
        self.state = self.next_state;
        return Ok(());
    }
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

pub fn split_input_data(data: &[u8]) -> Vec<u32> {
    let mut rdr = Cursor::new(data);
    let mut pkt_data: Vec<u32> = Vec::with_capacity(data.len() / 4);
    for _i in 0..data.len() / 4 {
        let d = rdr.read_u32::<LittleEndian>().unwrap();
        pkt_data.push(d);
    }
    pkt_data
}
