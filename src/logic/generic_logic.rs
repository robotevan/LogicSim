use super::logic_defs::LogicState;
use super::logic_block::{LogicBlock, LogicBlockPort, LogicBlockPortKey, LogicFn};
use slotmap::HopSlotMap;



/// All logic functions defined below
/// 


fn logic_and_fn(inputs: &HopSlotMap<LogicBlockPortKey, LogicBlockPort>) -> LogicState {
    for (_, input) in inputs {
        if input.state == LogicState::LOW {
            return LogicState::LOW
        }
    }
    LogicState::HIGH
}

fn logic_or_fn(inputs: &HopSlotMap<LogicBlockPortKey, LogicBlockPort>) -> LogicState {
    for (_, input) in inputs {
        if input.state == LogicState::HIGH {
            return LogicState::HIGH
        }
    }
    LogicState::LOW
}


/// helpers to return logic blocks for generic gates
pub fn new_and_gate() -> LogicFn {
    return LogicBlock::new(logic_and_fn)
}

pub fn new_or_gate() -> LogicFn {
    return LogicBlock::new(logic_or_fn)
}




mod tests {
    use super::*;

    #[test]
    fn test_and_gate() {
        let mut and_gate = new_and_gate();
        let in_1 = and_gate.add_input();
        let in_2 = and_gate.add_input();
        // manually poke update
        and_gate.update();
        assert_eq!(and_gate.get_output(), LogicState::LOW);

        // set 1 input high
        and_gate.set_input(in_1, LogicState::HIGH);    
        assert_eq!(and_gate.get_output(), LogicState::LOW);    
        
        // set 2 inputs high
        and_gate.set_input(in_2, LogicState::HIGH);
        assert_eq!(and_gate.get_output(), LogicState::HIGH);    
    }

    #[test]
    fn test_and_gate_invert_output() {
        let mut and_gate = new_and_gate();
        and_gate.invert_output(true);
        let in_1 = and_gate.add_input();
        let in_2 = and_gate.add_input();
        // manually poke update
        and_gate.update();
        assert_eq!(and_gate.get_output(), LogicState::HIGH);

        // set 1 input high
        and_gate.set_input(in_1, LogicState::HIGH);    
        assert_eq!(and_gate.get_output(), LogicState::HIGH);    
        
        // set 2 inputs high
        and_gate.set_input(in_2, LogicState::HIGH);
        assert_eq!(and_gate.get_output(), LogicState::LOW);      
    }

    #[test]
    fn test_or_gate() {
        let mut or_gate = new_or_gate();
        let in_1 = or_gate.add_input();
        let in_2 = or_gate.add_input();
        // manually poke update
        or_gate.update();
        assert_eq!(or_gate.get_output(), LogicState::LOW);

        // set 1 input high
        or_gate.set_input(in_1, LogicState::HIGH);    
        assert_eq!(or_gate.get_output(), LogicState::HIGH);    
        
        // set 2 inputs high
        or_gate.set_input(in_2, LogicState::HIGH);
        assert_eq!(or_gate.get_output(), LogicState::HIGH);    
    }

    #[test]
    fn test_or_gate_invert_output() {
        let mut or_gate = new_or_gate();
        or_gate.invert_output(true);
        let in_1 = or_gate.add_input();
        let in_2 = or_gate.add_input();
        // manually poke update
        or_gate.update();
        assert_eq!(or_gate.get_output(), LogicState::HIGH);

        // set 1 input high
        or_gate.set_input(in_1, LogicState::HIGH);    
        assert_eq!(or_gate.get_output(), LogicState::LOW);    
        
        // set 2 inputs high
        or_gate.set_input(in_2, LogicState::HIGH);
        assert_eq!(or_gate.get_output(), LogicState::LOW);    
    }
}