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

    pub fn from_ivec2(vec : IVec2) -> Direction {
        match vec {
            IVec2 { x : 0, y : 1 } => { Direction::Up }
            IVec2 { x : 0, y : -1 } => { Direction::Down }
            IVec2 { x : 1, y : 0 } => { Direction::Right }
            IVec2 { x : -1, y : 0 } => { Direction::Left }
            _ => { Direction::None }
        } 
    }

    pub fn negate(&self) -> Direction {
        match self {
            Direction::Up =>    Direction::Down,
            Direction::Down =>  Direction::Up,
            Direction::Left =>  Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None =>  Direction::None
        }
    }
}