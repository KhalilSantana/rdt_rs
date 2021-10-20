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

    pub fn new(transmitter: Sender<Packet>, receiver: Receiver<Packet>, rng_seed: u64) -> Self {
        ReliableDataTransportTransmitter {
            state: RdtTransmitterState::SendData,
            next_state: RdtTransmitterState::WaitingZero,
            sequence_number: 0,
            udt_layer: UnreliableDataTransport::new(transmitter, receiver, "TRANSMITTER -> RECEIVER", rng_seed),
            is_done: false,
            label: "TRANSMITTER -> RECEIVER",
        }
    }

    pub fn next(&mut self, data_buff: Payload) -> Result<(), std::sync::mpsc::RecvError> {
        match self.state {
            RdtTransmitterState::WaitingZero => {
                let packet = self.udt_layer.receive()?;

                if packet.checksum_ok() && packet.packet_type == PacketType::Acknowlodge && packet.sequence_number == 0 {
                    log_message_transmitter_received_ack(packet.sequence_number as usize);
                    stdout().flush();

                    self.sequence_number = 1;
                    self.next_state = RdtTransmitterState::WaitingOne;
                }else {
                    log_message_transmitter_failed(self.sequence_number, data_buff);
                    stdout().flush();
                }

                send_data(self, data_buff);
            }
            RdtTransmitterState::WaitingOne => {
                let packet = self.udt_layer.receive()?;

                if packet.checksum_ok() && packet.packet_type == PacketType::Acknowlodge && packet.sequence_number == 1 {
                    log_message_transmitter_received_ack(packet.sequence_number as usize);
                    stdout().flush();

                    self.sequence_number = 0;
                    self.next_state = RdtTransmitterState::WaitingZero;
                } else {
                    log_message_transmitter_failed(self.sequence_number, data_buff);
                    stdout().flush();
                }

                send_data(self, data_buff);
            }
            RdtTransmitterState::SendData => send_data(self, data_buff),
        }

        self.state = self.next_state;
        Ok(())
    }
}

fn send_data(rdt_transmitter: &mut ReliableDataTransportTransmitter, data_buff: Payload) {
    let packet = Packet::data(rdt_transmitter.sequence_number, data_buff);

    log_message_transmitter_sending(packet.sequence_number, &packet);
    stdout().flush();

    rdt_transmitter.udt_layer.maybe_send(&packet)
}
