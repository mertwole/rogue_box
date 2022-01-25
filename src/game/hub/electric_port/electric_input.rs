use super::*;

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

    pub fn request_energy(&mut self, energy : WattTick) -> Option<WattTick> {
        if self.buffer >= energy {
            self.buffer.0 = self.buffer.0 - energy.0;
            Some(energy)
        } else { None }
    }

    pub fn drain(&mut self) -> WattTick {
        let stored = self.buffer;
        self.buffer = WattTick(0);
        stored
    }

    pub fn is_full(&self) -> bool {
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
            MessageBody::SendElectricity(amount) => {
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