use super::*;

pub struct ElectricOutput {
    id : PortId,

    connected_inputs : Vec<(IVec2, PortId)>,

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
    pub fn add_energy(&mut self, energy : WattTick) -> WattTick {
        let free_space = self.throughput.0 - self.buffer.0;
        if free_space >= energy.0 {
            self.buffer.0 += energy.0;
            WattTick(0)
        } else {
            self.buffer.0 += free_space;
            WattTick(energy.0 - free_space)
        }
    }

    pub fn fill(&mut self) {
        self.buffer = self.throughput;
    }

    pub fn connect(&mut self, pos : IVec2, port_id : PortId) {
        self.connected_inputs.push((pos, port_id));
    }

    pub fn get_connected_inputs(&self) -> &Vec<(IVec2, PortId)> {
        &self.connected_inputs
    }
}

impl MessageSender for ElectricOutput {
    fn pull_messages(&mut self, tick_id : u32) -> Vec<Message> {
        if self.buffer.0 != 0 {
            let mut sender = MessageExchangeActor::new();
            sender.set_electric_port(self.id);
            vec![
                Message {
                    id : 0,
                    sender,
                    receiver : MessageExchangeActor::new(),
                    target : Target::BroadcastAllConnectedElectricInputs,
                    tick_id,
                    body : MessageBody::SendElectricity(self.buffer)
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
                    MessageBody::SendElectricity(remained) => { self.buffer = remained; }
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