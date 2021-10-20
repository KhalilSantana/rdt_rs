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
    received_data: bool,
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
            received_data: false,
            label: "TRANSMITTER -> RECEIVER",
        }
    }

    pub fn next(&mut self, data_buff: Payload) -> Result<(), std::sync::mpsc::RecvTimeoutError> {
        self.received_data = false;
        match self.state {
            RdtTransmitterState::WaitingZero => {
                let packet = self.udt_layer.maybe_receive()?;

                self.generic_waiting(data_buff, RdtTransmitterState::WaitingOne, 0, packet)
            }
            RdtTransmitterState::WaitingOne => {
                let packet = self.udt_layer.maybe_receive()?;

                self.generic_waiting(data_buff, RdtTransmitterState::WaitingZero, 1, packet);
            }
            RdtTransmitterState::SendData => send_data(self, data_buff),
        }

        self.state = self.next_state;
        Ok(())
    }

    pub fn received_data(&mut self) -> bool {
        self.received_data
    }

    fn generic_waiting(&mut self, data_buff: Payload, next_state: RdtTransmitterState, expected_sequence_number: u32, packet: Packet) {

        if packet.checksum_ok() && packet.packet_type == PacketType::Acknowlodge && packet.sequence_number == expected_sequence_number {
            log_message_transmitter_received_ack(packet.sequence_number as usize);
            stdout().flush();

            if packet.sequence_number == 0 {
                self.sequence_number = 1;
            }else {
                self.sequence_number = 0;
            }
            self.next_state = next_state;

            //TODO: remove o time out
        } else {
            log_message_transmitter_failed(self.sequence_number, data_buff);
            stdout().flush();
        }

        send_data(self, data_buff);

        // //TODO: adiciona o time out
        // let (tx, rx) = channel();
        // Timer::new().schedule_with_delay(chrono::Duration::nanoseconds(1),move || {
        //     tx.send(()).unwrap();
        //     //TODO: reenvia o pacote
        //     send_data(self, data_buff);
        //     println!("\nTIME OUT!!!!!!!!!!!!!\n");
        // });
        // rx.recv().unwrap();

        self.received_data = true;
    }
}

fn send_data(rdt_transmitter: &mut ReliableDataTransportTransmitter, data_buff: Payload) {
    let packet = Packet::data(rdt_transmitter.sequence_number, data_buff);

    log_message_transmitter_sending(packet.sequence_number, &packet);
    stdout().flush();

    rdt_transmitter.udt_layer.maybe_send(&packet)
}
