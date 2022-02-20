use crate::game::common::math::IVec2;

mod electric_input;
mod electric_output;
pub use electric_input::*;
pub use electric_output::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub struct Voltage(u32);
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd)]
pub struct WattTick(u32);
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PortId(u32);

impl Voltage {
    pub fn new(value: u32) -> Voltage {
        Voltage(value)
    }
}

impl WattTick {
    pub fn new(value: u32) -> WattTick {
        WattTick(value)
    }
}

impl PortId {
    pub fn new(id: u32) -> PortId {
        PortId(id)
    }
}

pub trait ElectricPort: ElectricPortClone {
    fn as_input(&self) -> Option<&ElectricInput>;
    fn as_output(&self) -> Option<&ElectricOutput>;

    fn as_input_mut(&mut self) -> Option<&mut ElectricInput>;
    fn as_output_mut(&mut self) -> Option<&mut ElectricOutput>;

    fn get_id(&self) -> PortId;
}

pub trait ElectricPortClone {
    fn clone_box(&self) -> Box<dyn ElectricPort>;
}
