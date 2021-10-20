use crate::messages::messages_receiver::messages_receiver::*;
use crate::packet::*;
use crate::payload::*;
use crate::udt::UnreliableDataTransport;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct ReliableDataTransportReceiver {
    state: RdtReceiverState,
    next_state: RdtReceiverState,
    sequence_number: u32,
    udt_layer: UnreliableDataTransport,
    data_buff: Vec<Payload>,
}

#[derive(Debug, Clone, Copy)]
pub enum RdtReceiverState {
    WaitingZero,
    WaitingOne,
}

/** Receptor **/
impl ReliableDataTransportReceiver {
    pub fn new(transmitter: Sender<Packet>, receiver: Receiver<Packet>, rng_seed: u64) -> Self {
        ReliableDataTransportReceiver {
            state: RdtReceiverState::WaitingZero,
            next_state: RdtReceiverState::WaitingZero,
            sequence_number: 0,
            udt_layer: UnreliableDataTransport::new(
                transmitter,
                receiver,
                "RECEIVER    -> TRANSMITTER",
                rng_seed,
            ),
            data_buff: vec![],
        }
    }

    pub fn next(&mut self) -> Result<(), std::sync::mpsc::RecvTimeoutError> {
        match self.state {
            RdtReceiverState::WaitingZero => {
                let packet = self.udt_layer.maybe_receive()?;
                if packet.checksum_ok() && packet.sequence_number == 0 {
                    self.data_buff.push(packet.payload);
                    log_message_receiver_payload(packet.sequence_number);

                    send_response(
                        self,
                        PacketType::Acknowlodge,
                        self.sequence_number,
                        self.sequence_number,
                    );

                    self.sequence_number = 1;
                    self.next_state = RdtReceiverState::WaitingOne;
                }

                if !packet.checksum_ok() {
                    log_message_receiver_garbage(packet.sequence_number);
                    let mut nack_number: u32 = 0;
                    if self.sequence_number == 0 {
                        nack_number = 1;
                    }
                    send_response(
                        self,
                        PacketType::NotAcklodge,
                        self.sequence_number,
                        nack_number,
                    )
                }

                if packet.checksum_ok() && packet.sequence_number == 1 {
                    log_message_receiver_dup(self.sequence_number);
                    send_response(self, PacketType::Acknowlodge, 1, self.sequence_number)
                }
            }

            RdtReceiverState::WaitingOne => {
                let packet = self.udt_layer.maybe_receive()?;
                if packet.checksum_ok() && packet.sequence_number == 1 {
                    self.data_buff.push(packet.payload);
                    log_message_receiver_payload(packet.sequence_number);

                    self.sequence_number = 0;
                    self.next_state = RdtReceiverState::WaitingZero;
                    send_response(self, PacketType::Acknowlodge, 1, self.sequence_number);
                }

                if !packet.checksum_ok() {
                    log_message_receiver_garbage(packet.sequence_number);
                    send_response(self, PacketType::NotAcklodge, 1, 0)
                }
                if packet.checksum_ok() && packet.sequence_number == 0 {
                    log_message_receiver_dup(self.sequence_number);
                    send_response(self, PacketType::Acknowlodge, 0, self.sequence_number)
                }
            }
        }
        self.state = self.next_state;
        Ok(())
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut output: Vec<u8> = Vec::with_capacity(self.data_buff.len());
        if !self.data_buff.is_empty() {
            for i in self.data_buff.iter() {
                for element in i.content.iter() {
                    output.push(*element);
                }
            }
            let padding = self.data_buff.last().unwrap().padding;
            if padding != 0 {
                output.truncate(output.len() - padding as usize);
            }
        }
        output
    }
}

fn send_response(
    rdt_receiver: &mut ReliableDataTransportReceiver,
    packet_type: PacketType,
    sequence_number: u32,
    nack_number: u32,
) {
    match packet_type {
        PacketType::Acknowlodge => {
            let packet = &Packet::ack(sequence_number);
            log_message_receiver_sending_ack(sequence_number, packet);
            rdt_receiver.udt_layer.maybe_send(packet);
        }

        PacketType::NotAcklodge => {
            let packet = &Packet::nack(nack_number, sequence_number);
            log_message_receiver_sending_nack(sequence_number, packet);
            rdt_receiver.udt_layer.maybe_send(packet);
        }
        _ => unreachable!("Client should never send other packet types!"),
    }
}
