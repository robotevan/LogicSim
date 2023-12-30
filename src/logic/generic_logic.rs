use super::logic_defs::LogicState;
use super::logic_block::{LogicBlock, LogicBlockPort, LogicBlockPortKey};
use slotmap::HopSlotMap;


fn logic_fn(inputs: &HopSlotMap<LogicBlockPortKey, LogicBlockPort>) -> LogicState {
    for (_, input) in inputs {
        if input.state == LogicState::LOW {
            return LogicState::LOW
        }
    }
    LogicState::HIGH
}


struct AndGate {
    gate_base: LogicBlock<fn(&HopSlotMap<LogicBlockPortKey, LogicBlockPort>) -> LogicState>,
}

impl AndGate {
    fn new() -> AndGate {
        AndGate {
            gate_base: LogicBlock::new(logic_fn),
        }
    }

    fn set_input(&mut self, input_key: LogicBlockPortKey, state: LogicState) {
        self.gate_base.set_input(input_key, state)
    }

    fn get_output(&self) -> LogicState {
        self.gate_base.get_output()
    }
}




mod tests {
    use crate::logic::logic_defs::LogicState;

    use super::AndGate;

    #[test]
    fn test_and_gate() {
        let mut and_gate = AndGate::new();
        let in_1 = and_gate.gate_base.add_input();
        let in_2 = and_gate.gate_base.add_input();
        // manually poke update
        and_gate.gate_base.update();
        assert_eq!(and_gate.get_output(), LogicState::LOW)
        
        
    }
}