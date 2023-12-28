use slotmap::{HopSlotMap, new_key_type};
use std::collections::HashMap;
use crate::logic::logic_defs::LogicState;

pub const MAX_CACHE_INPUTS: u32 = 32;

new_key_type! {
    pub struct LogicBlockPortKey;
}

pub struct LogicBlockPort {
    pub state: LogicState,
    pub is_inverted: bool,
    input_num: u32,
}

impl LogicBlockPort {
    fn new() -> LogicBlockPort {
        LogicBlockPort {
            state: LogicState::LOW,
            is_inverted: false,
            input_num: 0, // for cache
        }   
    }

    fn set_state(&mut self, state: LogicState) {
        if self.is_inverted {
            self.state = !state;
        } else {
            self.state = state;
        }
    }
}

pub struct LogicBlock {
    pub inputs: HopSlotMap<LogicBlockPortKey, LogicBlockPort>, 
    pub num_inputs: u32,
    output: LogicBlockPort,
    input_cache: u32,
    output_cache: HashMap<u32, LogicState>,
}

impl LogicBlock {
    pub fn new() -> LogicBlock {
        LogicBlock {
            inputs : HopSlotMap::with_key(),
            num_inputs: 0,
            output: LogicBlockPort::new(),
            input_cache: 0,
            output_cache: HashMap::new(),
        }
    }

    pub fn add_input(&mut self) -> LogicBlockPortKey {
        let key = self.inputs.insert(LogicBlockPort::new());
        // reset cache
        self.init_cache();
        key
    }

    pub fn remove_input(&mut self, key: LogicBlockPortKey) {
        assert!(!self.inputs.is_empty());
        self.inputs.remove(key);
        self.num_inputs -= 1;
        // reset cache
        self.output_cache = HashMap::new();
        self.input_cache = 0;
        self.init_cache()
    }

    pub fn set_input(&mut self, input_key: LogicBlockPortKey, state: LogicState) {
        // update the input cache
        let input = self.inputs.get_mut(input_key).unwrap();
        // set cached value
        if input.input_num < MAX_CACHE_INPUTS {
            if state == LogicState::HIGH {
                self.input_cache |= 1 << input.input_num;
            } else {
                self.input_cache &= 1 << input.input_num;
            }
            
        }
        // set actual value
        input.set_state(state);
    }

    pub fn set_output(&mut self, state: LogicState) {
        // if not in cache and under max num cachable inputs, add output to cache
        if !self.output_cache.contains_key(&self.input_cache){
            self.output_cache.insert(self.input_cache, state);
        }
        self.output.set_state(state)
    }

    fn init_cache(&mut self) {
        // too many inputs to cache, invalidate cache
        if self.num_inputs > 32 {
            self.output_cache = HashMap::new();
            self.input_cache = 0;
            return
        }
        
        // assign input number to inputs
        for (_, input) in self.inputs.iter_mut() {
            input.input_num = self.num_inputs;
            self.num_inputs += 1;
        }
    }

    pub fn check_output_cache(&mut self) -> LogicState {
        if self.num_inputs > MAX_CACHE_INPUTS {
            return LogicState::INVALID;
        } 

        match self.output_cache.get(&self.input_cache) {
            None => {
                return LogicState::INVALID;
            }
            // found in cache
            Some(&state) => {
                return state;
            }
        }
    }

    
}


mod tests {
    use crate::logic;
    use super::*;
    use std::mem::{self, MaybeUninit};

    #[test]
    fn test_add_input() {
        let mut logic_block = LogicBlock::new();
        logic_block.add_input();
        assert_eq!(logic_block.num_inputs, 1);
    }

    #[test]
    fn test_add_and_remove_input() {
        let mut logic_block = LogicBlock::new();
        let in_1 = logic_block.add_input();
        assert_eq!(logic_block.num_inputs, 1);
        logic_block.remove_input(in_1);
        assert_eq!(logic_block.num_inputs, 0);
    }

    #[test]
    fn test_cache_output() {
        let mut logic_block = LogicBlock::new();
        let in_1: LogicBlockPortKey = logic_block.add_input();
        logic_block.set_input(in_1, LogicState::HIGH);
        
        // check state is correct, 1 input
        assert_eq!(logic_block.inputs[in_1].state, LogicState::HIGH);
        assert_eq!(logic_block.num_inputs, 1);
        // add entry to cache, check if cache entry present
        logic_block.set_output(LogicState::HIGH);
        assert_eq!(logic_block.input_cache, logic_block.input_cache);
        assert!(logic_block.output_cache.contains_key(&logic_block.input_cache));
    }

    #[test]
    fn test_max_cache() {
        let mut logic_block = LogicBlock::new();
        let mut inputs: [LogicBlockPortKey; 32] = unsafe {MaybeUninit::uninit().assume_init()};
        // add inputs
        for i in 0..inputs.len() {
            inputs[i] = logic_block.add_input();
        }
        // set inputs high 1 by 1, add a cache entry, should create 32 entries
        for i in 0..inputs.len() {
            logic_block.set_input(inputs[i], LogicState::HIGH);
            logic_block.set_output(LogicState::HIGH);
        }
        assert_eq!(logic_block.output_cache.len(), 32);
        assert_eq!(logic_block.num_inputs, 32);
    }

    #[test]
    fn test_above_max_cache() {
        let mut logic_block = LogicBlock::new();
        let mut inputs: [LogicBlockPortKey; 33] = unsafe {MaybeUninit::uninit().assume_init()};
        // add inputs
        for i in 0..inputs.len() {
            inputs[i] = logic_block.add_input();
        }
        // set inputs high 1 by 1
        for i in 0..inputs.len() {
            logic_block.set_input(inputs[i], LogicState::HIGH);
            logic_block.set_output(LogicState::HIGH);
        }
        // above max cachable inputs, check if invalidated
        assert_eq!(logic_block.num_inputs, 33);
        assert_eq!(logic_block.output_cache.len(), 0);
    }

}