use crate::packet::*;
use crate::udt::UnreliableDataTransport;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransportTX {
    state: RdtTXState,
    next_state: RdtTXState,
    seq_num: u32,
    ack_num: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<u32>,
}
#[derive(Debug, Clone, Copy)]
pub enum RdtTXState {
    Waiting,
    SendData,
}
impl ReliableDataTransportTX {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>, data_buff: Vec<u32>) -> Self {
        let rdt = ReliableDataTransportTX {
            state: RdtTXState::SendData,
            next_state: RdtTXState::SendData,
            seq_num: 0,
            ack_num: 0,
            udt_layer: UnreliableDataTransport::new(tx, rx),
            data_buff,
        };
        rdt
    }
    pub fn next(&mut self) {
        dbg!(&self);
        self.state = self.next_state;
        match self.state {
            RdtTXState::Waiting => {
                let pkt = self.udt_layer.receive();
                match (pkt.checksum_ok(), pkt.pkt_type) {
                    (true, PacketType::Acknowlodge) => {
                        dbg!("[TX] - Got ACK");
                        self.data_buff.pop();
                        self.seq_num += 1;
                        self.next_state = RdtTXState::SendData;
                        if self.data_buff.len() == 0 {
                            println!("Entire data buffer sent, quitting");
                            std::process::exit(0);
                        }
                    }
                    (_, _) => {
                        self.next_state = RdtTXState::SendData;
                    }
                }
            }
            RdtTXState::SendData => {
                let pkt = Packet::new(
                    self.seq_num,
                    self.ack_num,
                    PacketType::Acknowlodge,
                    *self.data_buff.first().unwrap(),
                );
                self.next_state = RdtTXState::Waiting;
                self.udt_layer.send(&pkt);
            }
        }
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
