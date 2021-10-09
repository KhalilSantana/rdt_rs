use crate::packet::*;
use crate::udt::UnreliableDataTransport;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};
#[derive(Debug)]
pub struct ReliableDataTransport<S> {
    state: S,
    seq_num: u32,
    ack_num: u32,
    buffer: Vec<u32>,
    udt_layer: UnreliableDataTransport,
}
#[derive(Debug)]
pub struct Waiting {}
#[derive(Debug)]
pub struct SendData {}
#[derive(Debug)]
pub struct ExpectingAnswer {}
#[derive(Debug)]
pub struct RetrainsmitPacket {}

impl ReliableDataTransport<Waiting> {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        let rdt = ReliableDataTransport {
            state: Waiting {},
            seq_num: 0,
            ack_num: 0,
            buffer: vec![],
            udt_layer: UnreliableDataTransport::new(tx, rx),
        };
        rdt
    }
}

impl From<ReliableDataTransport<Waiting>> for ReliableDataTransport<SendData> {
    fn from(sm: ReliableDataTransport<Waiting>) -> ReliableDataTransport<SendData> {
        ReliableDataTransport {
            state: SendData {},
            seq_num: sm.seq_num,
            ack_num: sm.ack_num,
            buffer: sm.buffer,
            udt_layer: sm.udt_layer,
        }
    }
}

impl ReliableDataTransport<SendData> {
    pub fn send(&mut self, data: &[u8]) {
        self.buffer = split_input_data(data);
        for mut i in 0..self.buffer.len() {
            let pkt = Packet::new(self.seq_num, self.ack_num, self.buffer.pop().unwrap());
            self.udt_layer.send(&pkt);
            let response = self.udt_layer.receive();
            if !response.checksum_ok() || !response.is_ack {
                i -= 1;
                continue;
            }
            self.seq_num += 1;
            self.ack_num = response.ack_num;
        }
    }
}

pub fn split_input_data(data: &[u8]) -> Vec<u32> {
    let mut rdr = Cursor::new(data);
    let mut pkt_data: Vec<u32> = Vec::with_capacity(data.len() / 4);
    for i in 0..data.len() / 4 {
        let d = rdr.read_u32::<LittleEndian>().unwrap();
        pkt_data.push(d);
    }
    pkt_data
}
