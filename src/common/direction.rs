use crate::common::math::IVec2;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

impl Direction {
    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up =>    IVec2::new(0, 1),
            Direction::Down =>  IVec2::new(0, -1),
            Direction::Left =>  IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
            Direction::None =>  IVec2::new(0, 0)
        }
    }
}