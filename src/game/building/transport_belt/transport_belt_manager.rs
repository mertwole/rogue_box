use std::rc::Rc;
use std::collections::HashMap;

use super::TransportBelt;
use crate::game::game_entity::*;
use crate::common::math::IVec2;
use crate::common::direction::Direction;

struct TransportBeltIteratorFork {
    pos : IVec2,
    unchecked_directions : Vec<Direction>
}

struct TransportBeltIterator {
    curr_belt : Option<IVec2>,
    fork_stack : Vec<TransportBeltIteratorFork>
}

impl TransportBeltIterator {
    fn get_next_belt(&mut self, system : &TransportBeltSystem) -> Option<IVec2> {
        let curr_belt = system.belts.get(&self.curr_belt.unwrap()).unwrap();
        let next_directions = curr_belt.inputs.clone();
        let next_belts : Vec<IVec2> = next_directions.iter()
        .map(|dir| { self.curr_belt.unwrap() + dir.to_ivec2() })
        .filter(|pos| { system.belts.contains_key(pos) })
        .collect();
        match next_belts.len() {
            1 => { return Some(next_belts[0]); }
            0 => { 
                loop {
                    let mut last_fork = match self.fork_stack.pop() {
                        Some(fork) => { fork }
                        None => { return None; }
                    };

                    let next_belt = Some(last_fork.pos + last_fork.unchecked_directions[0].to_ivec2());
                    match last_fork.unchecked_directions.len() {
                        0 => { continue; }
                        1 => { return next_belt; }
                        _ => {
                            last_fork.unchecked_directions = last_fork.unchecked_directions
                            .clone().into_iter().skip(1).collect();
                            self.fork_stack.push(last_fork);
                            return next_belt;
                        }
                    }
                }
            }
            _ => { 
                self.fork_stack.push(TransportBeltIteratorFork { 
                    pos : self.curr_belt.unwrap(), 
                    unchecked_directions : next_directions.into_iter().skip(1).collect()
                });
                return Some(next_belts[0]);
            }
        }
    }

    fn next(&mut self, system : &TransportBeltSystem) -> Option<IVec2> {
        let output = self.curr_belt.clone();
        if self.curr_belt.is_none() { return None; }
        self.curr_belt = self.get_next_belt(system);
        output
    }
}

struct TransportBeltSystem {
    belts : HashMap<IVec2, Rc<TransportBelt>>,
    head_belt : IVec2
}

impl TransportBeltSystem {
    fn new(belt : Rc<TransportBelt>, position : IVec2) -> TransportBeltSystem {
        let mut belts = HashMap::new();
        belts.insert(position, belt);
        TransportBeltSystem {
            belts,
            head_belt : position
        }
    }

    fn try_add_transport_belt(&mut self, belt : Rc<TransportBelt>, position : IVec2) -> bool {
        let mut can_connect_positions = Vec::new();
        let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        for dir in directions {
            let pos = position + dir.to_ivec2();
            if self.belts.contains_key(&pos) {
                if self.belts.get(&pos).unwrap().check_can_connect(belt.as_ref()) {
                    can_connect_positions.push(pos);
                }
            }
        }

        if can_connect_positions.is_empty() { return false; }

        self.belts.insert(position, belt);

        let curr_system_out = self.head_belt + self.belts.get(&self.head_belt).unwrap().output.to_ivec2();
        if self.belts.contains_key(&curr_system_out) {
            // System outputs items to newly added belt.
            self.head_belt = curr_system_out;
        }

        true
    }

    fn unite(mut self, other : TransportBeltSystem) -> TransportBeltSystem {
        let self_head = &self.belts.get(&self.head_belt).unwrap();
        let self_head_out = self.head_belt + self_head.output.to_ivec2();
        self.head_belt = if other.belts.contains_key(&self_head_out) {
            // Self outputs items to other.
            other.head_belt
        } else {
            // In reverse.
            self.head_belt
        };

        for (pos, belt) in other.belts {
            // Check for common belt.
            if !self.belts.contains_key(&pos) {
                self.belts.insert(pos, belt);
            }
        }

        self
    }

    // Returns systems that remain when the current system is fractured.
    fn remove_transport_belt(&self, position : IVec2) -> Vec<TransportBeltSystem> {
        Vec::new()
    }

    fn get_tick_order(&self) -> TransportBeltIterator {
        TransportBeltIterator { 
            curr_belt : Some(self.head_belt), 
            fork_stack : Vec::new() 
        }
    }
}

pub struct TransportBeltManager {
    systems : Vec<Rc<TransportBeltSystem>>
}

struct TickOrder {
    tick_orders : Vec<(Rc<TransportBeltSystem>, TransportBeltIterator)>,
    curr : usize
}

impl Iterator for TickOrder {
    type Item = IVec2;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (curr_system, curr_order) = &mut self.tick_orders[self.curr];
            match curr_order.next(curr_system.as_ref()) {
                Some(pos) => { return Some(pos); }
                None => { 
                    if self.curr == self.tick_orders.len() - 1 { return None; }
                    self.curr += 1;
                    continue;
                }
            }
        }
    }
}

impl TransportBeltManager {
    pub fn new() -> TransportBeltManager {
        TransportBeltManager {
            systems : Vec::new()
        }
    }

    pub fn add_transport_belt(&mut self, belt : Rc<TransportBelt>, position : IVec2) {
        let mut added_to_ids = Vec::new();
        for system_id in 0..self.systems.len() {
            let system = Rc::get_mut(&mut self.systems[system_id]).unwrap();
            if system.try_add_transport_belt(belt.clone(), position) {
                added_to_ids.push(system_id);
            }
        }

        match added_to_ids.len() {
            1 => {  }
            0 => { 
                let new_system = TransportBeltSystem::new(belt.clone(), position); 
                self.systems.push(Rc::from(new_system));
            }
            _ => { 
                let mut added_to = Vec::new();
                for id in added_to_ids {
                    let system = Rc::try_unwrap(self.systems.remove(id)); 
                    added_to.push(system.ok().unwrap());
                }
                let acc = added_to.pop().unwrap();
                let new_system = added_to.into_iter().fold(acc, |acc, x| acc.unite(x));
                self.systems.push(Rc::from(new_system));
            }
        }
    }

    pub fn remove_transport_belt(&mut self, position : IVec2) {

    }

    pub fn get_tick_order(&self) -> impl Iterator<Item = IVec2> {
        let mut tick_orders = Vec::new();
        for system in &self.systems {
            let order = system.get_tick_order();
            tick_orders.push((system.clone(), order));
        }
        TickOrder { tick_orders, curr : 0 }
    }
}