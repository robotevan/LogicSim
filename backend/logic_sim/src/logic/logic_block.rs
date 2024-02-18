use std::collections::HashMap;
use slotmap::{HopSlotMap, new_key_type};
use super::logic_defs::LogicState;


new_key_type! {
    pub struct LogicBlockPortKey;
}




/// input/output port for logic blocks
struct LogicBlockPort {
    is_inverted:  bool,
    state:        LogicState,
    input_number: usize,
}

/// base for all logic components
struct LogicBlock {
    inputs: HopSlotMap::<LogicBlockPortKey, LogicBlock>,
    output: LogicState,
    cache:  LogicBlockCache,
}






impl LogicBlockPort {
    pub fn new() -> LogicBlockPort  {
        LogicBlockPort {
            is_inverted:  false,
            state:        LogicState::INVALID,
            input_number: 0,
        }
    }

    pub fn invert(&mut self, inverted: bool) {
        self.is_inverted = inverted;
    }

    pub fn set_state(&mut self, state: LogicState) {
        if self.is_inverted {
            self.state = !state;
        } else {
            self.state = state;
        }
    }

    pub fn get_output(&self) -> LogicState {
        self.state
    }
}

impl LogicBlockCache {
    fn new() -> LogicBlockCache {
        LogicBlockCache {
            input_mask:   0,
            output_cache: HashMap::new(),
            cache_valid: true,
        }
    }

    fn reset_cache(&mut self) {
        self.input_mask = 0;
        self.output_cache = HashMap::new();
    }

    fn get_cached_output(&self) -> LogicState {
        if self.cache_valid {
            return LogicState::INVALID
        }

        match self.output_cache.get(&self.input_mask) {
            None => LogicState::INVALID,
            Some(&cached_value) => cached_value
        }
    }

    fn update_cache(&mut self, output: LogicState) {
        self.output_cache.insert(self.input_mask, output);
    }

    fn set_input(&mut self, input_number: usize, state: LogicState) {
        if state == LogicState::HIGH {
            self.input_mask |= 1 << input_number;
        } else if state == LogicState::LOW {
            self.input_mask &= 1 << input_number;
        }
    }
}


impl LogicBlock {
    pub fn new() -> LogicBlock {
        LogicBlock {
            inputs: HopSlotMap::with_key(),
            output: LogicState::INVALID,
            cache:  LogicBlockCache::new(),
        }
    }
}