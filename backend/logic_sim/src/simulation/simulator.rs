use std::collections::VecDeque;

use crate::wire::wire::Wire;

struct Simulator {
    // hold a queue of next wires to update
    next_updates: Vec<Wire>,    
}

impl Simulator {
    fn update(&mut self) {
        let next_wire = self.next_updates.pop().unwrap();
        let next_gates =  next_wire.outputs;
        for 
    }
}