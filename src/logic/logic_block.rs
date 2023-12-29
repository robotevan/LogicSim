use slotmap::{HopSlotMap, new_key_type};
use std::collections::HashMap;
use crate::logic::logic_defs::LogicState;

pub const MAX_CACHE_INPUTS: usize = 32;

new_key_type! {
    pub struct LogicBlockPortKey;
}

pub struct LogicBlockPort {
    pub state: LogicState,
    pub is_inverted: bool,
    input_num: usize,
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
    output: LogicBlockPort,
    input_cache: u32,
    output_cache: HashMap<u32, LogicState>,
    cache_valid: bool,
}

impl LogicBlock {
    pub fn new() -> LogicBlock {
        LogicBlock {
            inputs : HopSlotMap::with_key(),
            output: LogicBlockPort::new(),
            input_cache: 0,
            output_cache: HashMap::new(),
            cache_valid: false,
        }
    }

    pub fn add_input(&mut self) -> LogicBlockPortKey {
        let key = self.inputs.insert(LogicBlockPort::new());
        // reset cache
        self.update_cache();
        key
    }

    pub fn remove_input(&mut self, key: LogicBlockPortKey) {
        self.inputs.remove(key);
        self.update_cache();
    }

    pub fn set_input(&mut self, input_key: LogicBlockPortKey, state: LogicState) {
        // update the input cache
        let input = self.inputs.get_mut(input_key).unwrap();
        // set cached value
        if self.cache_valid {
            if state == LogicState::HIGH {
                self.input_cache |= 1 << input.input_num;
            } else {
                self.input_cache &= 1 << input.input_num;
            }
            
        }
        input.set_state(state);
    }

    pub fn set_output(&mut self, state: LogicState) {
        match(self.output_cache.get(&self.input_cache)) {
            // not found in cache, need to add entry
            None => {
                self.output_cache.insert(self.input_cache, state);
            }
            // found in cache, check if value needs to be updated
            Some(&cached_state) => {
                if cached_state != state {
                    self.output_cache.insert(self.input_cache, state);
                }
            }
        }
        // set the output
        self.output.set_state(state)
    }

    fn update_cache(&mut self) {
        // invalidate previous cache
        self.output_cache = HashMap::new();
        self.input_cache = 0;
        
        // too many inputs to cache, invalidate cache
        if self.inputs.len() > MAX_CACHE_INPUTS {
            self.cache_valid = false;
            return;
        }
        
        self.cache_valid = true;
        // assign input number to inputs
        let mut input_num = 0;
        for (_, input) in self.inputs.iter_mut() {
            input.input_num = input_num;
            input_num += 1;
        }
    }

    pub fn check_output_cache(&mut self) -> LogicState {
        if !self.cache_valid {
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
        assert_eq!(logic_block.inputs.len(), 1);
    }

    #[test]
    fn test_add_and_remove_input() {
        let mut logic_block = LogicBlock::new();
        let in_1 = logic_block.add_input();
        assert_eq!(logic_block.inputs.len(), 1);
        logic_block.remove_input(in_1);
        assert_eq!(logic_block.inputs.len(), 0);
    }

    #[test]
    fn test_cache_output() {
        let mut logic_block = LogicBlock::new();
        let in_1: LogicBlockPortKey = logic_block.add_input();
        logic_block.set_input(in_1, LogicState::HIGH);
        
        // check state is correct, 1 input
        assert_eq!(logic_block.inputs[in_1].state, LogicState::HIGH);
        assert_eq!(logic_block.inputs.len(), 1);
        // add entry to cache, check if cache entry present
        logic_block.set_output(LogicState::HIGH);
        assert_eq!(logic_block.input_cache, logic_block.input_cache);
        assert!(logic_block.output_cache.contains_key(&logic_block.input_cache));
    }

    #[test]
    fn test_max_cache() {
        let mut expected_cache_mask: u32 = 0;
        let mut logic_block = LogicBlock::new();
        let mut inputs: [LogicBlockPortKey; MAX_CACHE_INPUTS] = unsafe {MaybeUninit::uninit().assume_init()};
        // add inputs
        for i in 0..inputs.len() {
            inputs[i] = logic_block.add_input();
        }
        // set inputs high 1 by 1, add a cache entry, should create 32 entries
        for i in 0..inputs.len() {
            logic_block.set_input(inputs[i], LogicState::HIGH);
            logic_block.set_output(LogicState::HIGH);
            // verify input cache mask is correct
            expected_cache_mask |= 1 << i;
            assert_eq!(logic_block.input_cache, expected_cache_mask);
            assert_eq!(logic_block.output_cache.len(), i + 1);
            assert_eq!(logic_block.cache_valid, true);
        }
        assert_eq!(logic_block.output_cache.len(), MAX_CACHE_INPUTS);
        assert_eq!(logic_block.inputs.len(), MAX_CACHE_INPUTS);
    }

    #[test]
    fn test_above_max_cache() {
        let mut expected_cache_mask: u32 = 0;
        let mut logic_block = LogicBlock::new();
        let mut inputs: [LogicBlockPortKey; MAX_CACHE_INPUTS] = unsafe {MaybeUninit::uninit().assume_init()};
        // add 32 inputs
        for i in 0..inputs.len() {
            inputs[i] = logic_block.add_input();
        }
        // set inputs high 1 by 1
        for i in 0..inputs.len() {
            logic_block.set_input(inputs[i], LogicState::HIGH);
            logic_block.set_output(LogicState::HIGH);
            // verify input cache mask is correct
            expected_cache_mask |= 1 << i;
            assert_eq!(logic_block.input_cache, expected_cache_mask);
            assert_eq!(logic_block.output_cache.len(), i + 1);
            assert_eq!(logic_block.cache_valid, true);
        }
        // add one more input
        logic_block.add_input();
        // above max cachable inputs, check if invalidated
        assert_eq!(logic_block.inputs.len(), 33);
        assert_eq!(logic_block.output_cache.len(), 0);
        assert_eq!(logic_block.cache_valid, false);
    }

    #[test]
    fn test_remove_one_after_max() {
        let mut logic_block = LogicBlock::new();
        let mut inputs: [LogicBlockPortKey; MAX_CACHE_INPUTS+1] = unsafe {MaybeUninit::uninit().assume_init()};
        // add 33 inputs
        for i in 0..inputs.len() {
            inputs[i] = logic_block.add_input();
        }
        // make sure cache is invalid
        assert_eq!(logic_block.inputs.len(), 33);
        assert_eq!(logic_block.cache_valid, false);
        // remove one input and check if valid again
        logic_block.remove_input(inputs[0]);
        assert_eq!(logic_block.inputs.len(), 32);
        assert_eq!(logic_block.cache_valid, true); 
    }

    #[test]
    fn test_remove_with_no_inputs() {
        let mut logic_block = LogicBlock::new();
        let in_1 = logic_block.add_input();
        assert_eq!(logic_block.inputs.len(), 1);
        logic_block.remove_input(in_1);
        // try to remove a second time
        assert!(logic_block.inputs.is_empty());
        logic_block.remove_input(in_1);
        assert!(logic_block.inputs.is_empty());
    }
}