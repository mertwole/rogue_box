use crate::common::math::IVec2;
use crate::game::message::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub struct Voltage(u32);
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub struct WattTick(u32);
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PortId(u32);

impl Voltage {
    pub fn new(value : u32) -> Voltage {
        Voltage(value)
    }
}

impl WattTick {
    pub fn new(value : u32) -> WattTick {
        WattTick(value)
    }
}

impl PortId {
    pub fn new(id : u32) -> PortId {
        PortId(id)
    }
}

pub trait ElectricPort : ElectricPortClone{ 
    fn as_input(&self) -> Option<&ElectricInput>;
    fn as_output(&self) -> Option<&ElectricOutput>;

    fn as_input_mut(&mut self) -> Option<&mut ElectricInput>;
    fn as_output_mut(&mut self) -> Option<&mut ElectricOutput>;

    fn get_id(&self) -> PortId;
}

pub trait ElectricPortClone {
    fn clone_box(&self) -> Box<dyn ElectricPort>;
}

pub struct ElectricInput {
    id : PortId,

    voltage : Voltage,
    request : WattTick,
    buffer : WattTick
}

impl ElectricInput {
    pub fn new(voltage : Voltage, request : WattTick, id : PortId) -> ElectricInput {
        ElectricInput {
            id,
            voltage,
            request,
            buffer : WattTick(0)
        }
    }

    pub(in crate::game::building) fn request_energy(&mut self, energy : WattTick) -> Option<WattTick> {
        if self.buffer >= energy {
            self.buffer.0 = self.buffer.0 - energy.0;
            Some(energy)
        } else { None }
    }

    pub(in crate::game::building) fn drain(&mut self) -> WattTick {
        let stored = self.buffer;
        self.buffer = WattTick(0);
        stored
    }

    pub(in crate::game::building) fn is_full(&self) -> bool {
        self.buffer >= self.request
    }
}

impl ElectricPortClone for ElectricInput {
    fn clone_box(&self) -> Box<dyn ElectricPort> {
        Box::from(
            ElectricInput {
                id : self.id,
                buffer : self.buffer,
                request : self.request,
                voltage : self.voltage
            }
        )
    }
}

impl ElectricPort for ElectricInput { 
    fn as_input(&self) -> Option<&ElectricInput> {
        Some(self)
    }

    fn as_output(&self) -> Option<&ElectricOutput> {
        None
    }

    fn as_input_mut(&mut self) -> Option<&mut ElectricInput> {
        Some(self)
    }

    fn as_output_mut(&mut self) -> Option<&mut ElectricOutput> {
        None
    }

    fn get_id(&self) -> PortId { self.id }
}

impl MessageReceiver for ElectricInput {
    fn try_push_message(&mut self, mut message : Message) -> Option<Message> {
        match &mut message.body {
            MessageBody::SendElectricity(amount, id) => {
                if self.id != *id { return Some(message); }

                let free_space = self.request.0 - self.buffer.0;
                if free_space == 0 { return Some(message); }
                if free_space >= amount.0 {
                    self.buffer.0 += amount.0;
                    None
                } else {
                    (*amount).0 -= free_space;
                    self.buffer = self.request;
                    Some(message)
                }
            }
            _ => { Some(message) }
        }
    }
}


pub struct ElectricOutput {
    id : PortId,

    connected_inputs : Vec<IVec2>,

    voltage : Voltage,
    throughput : WattTick,
    buffer : WattTick
}

impl ElectricOutput {
    pub fn new(voltage : Voltage, throughput : WattTick, id : PortId) -> ElectricOutput {
        ElectricOutput {
            id,
            connected_inputs : Vec::new(),
            voltage,
            throughput,
            buffer : WattTick(0)
        }
    }

    // Returns remained energy.
    pub(in crate::game::building) fn add_energy(&mut self, energy : WattTick) -> WattTick {
        let free_space = self.throughput.0 - self.buffer.0;
        if free_space >= energy.0 {
            self.buffer.0 += energy.0;
            WattTick(0)
        } else {
            self.buffer.0 += free_space;
            WattTick(energy.0 - free_space)
        }
    }

    pub(in crate::game::building) fn fill(&mut self) {
        self.buffer = self.throughput;
    }

    pub fn get_connected_inputs(&self) -> &Vec<IVec2> {
        &self.connected_inputs
    }
}

impl MessageSender for ElectricOutput {
    fn pull_messages(&mut self, tick_id : u32) -> Vec<Message> {
        if self.buffer.0 != 0 {
            vec![
                Message {
                    id : 0,
                    sender : MessageExchangeActor::NotComputedYet,
                    receiver : MessageExchangeActor::NotComputedYet,
                    target : Target::BroadcastAllConnectedElectricInputs,
                    tick_id,
                    body : MessageBody::SendElectricity(self.buffer, self.id)
                }
            ]
        } else { 
            vec![] 
        }
    }

    fn message_send_result(&mut self, result : MessageSendResult) {
        match result.message {
            Some(msg) => {
                match msg.body {
                    MessageBody::SendElectricity(remained, id) => { self.buffer = remained; }
                    _ => { }
                }
            }
            None => { self.buffer = WattTick(0); }
        }
    }
}

impl ElectricPortClone for ElectricOutput {
    fn clone_box(&self) -> Box<dyn ElectricPort> {
        Box::from(
            ElectricOutput {
                id : self.id,
                buffer : self.buffer,
                throughput : self.throughput,
                voltage : self.voltage,
                connected_inputs : self.connected_inputs.clone()
            }
        )
    }
}

impl ElectricPort for ElectricOutput {
    fn as_input(&self) -> Option<&ElectricInput> {
        None
    }

    fn as_output(&self) -> Option<&ElectricOutput> {
        Some(self)
    }

    fn as_input_mut(&mut self) -> Option<&mut ElectricInput> {
        None
    }

    fn as_output_mut(&mut self) -> Option<&mut ElectricOutput> {
        Some(self)
    }

    fn get_id(&self) -> PortId { self.id }
}