use crate::packet::*;
use crate::udt::UnreliableDataTransport;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::sync::mpsc::{Receiver, Sender};
pub struct ReliableDataTransportRX<S> {
    state: S,
    seq_num: u32,
    ack_num: u32,
    buffer: Vec<Packet>,
    data: Vec<u32>,
    udt_layer: UnreliableDataTransport,
}
#[derive(Debug)]
pub struct Waiting {}
#[derive(Debug)]
pub struct ReceiveData {}
#[derive(Debug)]
pub struct SendACK {}
#[derive(Debug)]
pub struct SendNACK {}

impl ReliableDataTransportRX<Waiting> {
    pub fn new(tx: Sender<Packet>, rx: Receiver<Packet>) -> Self {
        let rdt = ReliableDataTransportRX {
            state: Waiting {},
            seq_num: 0,
            ack_num: 0,
            buffer: vec![],
            data: vec![],
            udt_layer: UnreliableDataTransport::new(tx, rx),
        };
        rdt
    }
}

impl From<ReliableDataTransportRX<Waiting>> for ReliableDataTransportRX<ReceiveData> {
    fn from(sm: ReliableDataTransportRX<Waiting>) -> ReliableDataTransportRX<ReceiveData> {
        ReliableDataTransportRX {
            state: ReceiveData {},
            ack_num: sm.ack_num,
            seq_num: sm.seq_num,
            buffer: sm.buffer,
            data: vec![],
            udt_layer: sm.udt_layer,
        }
    }
}
impl ReliableDataTransportRX<ReceiveData> {
    pub fn receive(&mut self) -> Result<u32, u32> {
        if self.buffer.len() != 0 {
            let pkt = self.buffer.pop().unwrap();
            if !pkt.checksum_ok() {
                return Err(self.ack_num);
            }
            return Ok(pkt.pkt_data);
        }
        Err(0)
    }
}

impl ReliableDataTransportRX<SendACK> {
    pub fn send(&mut self, data: &[u8]) {
        let pkt = Packet::new(self.seq_num, self.ack_num, 0);
        self.udt_layer.send(&pkt);
        let response = self.udt_layer.receive();
        if !response.checksum_ok() || !response.is_ack {
            self.seq_num += 1;
            self.ack_num += 1;
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
