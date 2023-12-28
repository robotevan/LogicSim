use super::logic_defs::LogicState;
use super::logic_block::{LogicBlock, MAX_CACHE_INPUTS};

trait GenericGate {
    fn update(&mut self);
    fn logic_fn(&self) -> LogicState;
}

struct AndGate {
    gate_base: LogicBlock,
}

impl AndGate {
    fn new() -> AndGate {
        AndGate {
           gate_base: LogicBlock::new(), 
        }
    }
}

impl GenericGate for AndGate {
    fn logic_fn(&self) -> LogicState {
        for (_, input) in self.gate_base.inputs.iter() {
            if input.state != LogicState::HIGH {
                return LogicState::LOW
            }
        }
        LogicState::HIGH
    }

    fn update(&mut self) {
        let mut output: LogicState;
        // check if cached, will update output if found
        let cached_output = self.gate_base.check_output_cache();
        // check if logic_fn needs to be used
        if cached_output == LogicState::INVALID {
            output = self.logic_fn();
        } else { // will add to cache
            output = cached_output;
        }
        self.gate_base.set_output(output)    
    }
}




mod tests {
    use crate::logic::{logic_defs::LogicState, generic_logic::GenericGate};

    use super::AndGate;

    #[test] 
    fn test_and_gate() {
        let mut and_gate = AndGate::new();
        let in_1 = and_gate.gate_base.add_input();
        let in_2 = and_gate.gate_base.add_input();

        and_gate.gate_base.set_input(in_1, LogicState::HIGH);
        and_gate.gate_base.set_input(in_2, LogicState::HIGH);

        assert_eq!(and_gate.gate_base.check_output_cache(), LogicState::INVALID);

        and_gate.update();

        assert_eq!(and_gate.gate_base.check_output_cache(), LogicState::HIGH);
    }
}