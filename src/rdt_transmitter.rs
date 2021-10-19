use crate::packet::*;
use crate::payload::*;
use crate::udt::UnreliableDataTransport;
use std::io::{stdout, Write};
use std::sync::mpsc::{Receiver, Sender};
use crate::enums::messages_transmitter::messages_transmitter::*;

#[derive(Debug)]
pub struct ReliableDataTransportTransmitter {
    state: RdtTransmitterState,
    next_state: RdtTransmitterState,
    sequence_number: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<Payload>,
    is_done: bool,
    label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub enum RdtTransmitterState {
    SendData,
    WaitingZero,
    WaitingOne,
}

/** Transmissor **/
impl ReliableDataTransportTransmitter {

    pub fn new(transmitter: Sender<Packet>, receiver: Receiver<Packet>, data_buff: &[u8], rng_seed: u64) -> Self {
        ReliableDataTransportTransmitter {
            state: RdtTransmitterState::SendData,
            next_state: RdtTransmitterState::WaitingZero,
            sequence_number: 0,
            udt_layer: UnreliableDataTransport::new(transmitter, receiver, "TRANSMITTER -> RECEIVER", rng_seed),
            data_buff: crate::payload::split_data(data_buff),
            is_done: false,
            label: "TRANSMITTER -> RECEIVER",
        }
    }

    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvError> {

        match self.state {
            RdtTransmitterState::WaitingZero => {
                let packet = self.udt_layer.receive()?;

                if packet.checksum_ok() && packet.packet_type == PacketType::Acknowlodge && packet.sequence_number == 0 {
                    log_message_transmitter_received_ack_zero(packet.sequence_number);

                    stdout().flush();
                    self.set_attributes(1,0,RdtTransmitterState::WaitingOne);
                    // self.data_buff.remove(0);
                    // self.sequence_number = 1;
                    // self.next_state = RdtTransmitterState::WaitingOne;
                    send_data(self);
                } else {
                    log_message_transmitter_failed(self.sequence_number, self.data_buff.first().unwrap());
                    stdout().flush();
                    send_data(self);
                }
            }

            RdtTransmitterState::WaitingOne => {
                let packet = self.udt_layer.receive()?;
                if packet.checksum_ok() && packet.packet_type == PacketType::Acknowlodge && packet.sequence_number == 1 {
                    log_message_transmitter_received_ack_one(packet.sequence_number);
                    stdout().flush();
                    self.set_attributes(0,0,RdtTransmitterState::WaitingZero);
                    // self.data_buff.remove(0);
                    // self.sequence_number = 0;
                    // self.next_state = RdtTransmitterState::WaitingZero;
                    send_data(self);
                } else {
                    log_message_transmitter_failed(self.sequence_number, self.data_buff.first().unwrap());
                    stdout().flush();
                    send_data(self);
                }
            }
            RdtTransmitterState::SendData => send_data(self),
        }
        self.state = self.next_state;
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }

    fn set_done(&mut self) {
        println!("\n[RDT] == Entire data buffer sent, quitting ==");
        stdout().flush();
        self.is_done = true;
    }

    fn set_attributes(&mut self, sequence_number: u32, index: usize, next_state: RdtTransmitterState){
        self.data_buff.remove(index);
        self.sequence_number = sequence_number;
        self.next_state = next_state;
    }
}

fn send_data(rdt_transmitter: &mut ReliableDataTransportTransmitter) {
    if rdt_transmitter.data_buff.is_empty() {
        rdt_transmitter.set_done();
        return;
    }

    let packet = Packet::data(rdt_transmitter.sequence_number, *rdt_transmitter.data_buff.first().unwrap());
    log_message_transmitter_sending(packet.sequence_number,&packet);
    stdout().flush();
    rdt_transmitter.udt_layer.maybe_send(&packet)
}
