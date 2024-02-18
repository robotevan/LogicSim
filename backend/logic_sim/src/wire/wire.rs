use crate::logic::{logic_defs::LogicState, logic_block::{LogicBlock, LogicFn, LogicBlockPortKey}};


struct WireOutput  {
    pub logic_block:  LogicBlock<LogicFn>,
    pub input_key:    LogicBlockPortKey,
}

impl WireOutput {
    fn new(logic_block: LogicBlock<LogicFn>, key: LogicBlockPortKey) -> WireOutput {
        WireOutput {
            logic_block: logic_block,
            input_key: key,
        }
    }
}

pub struct Wire {
    state: LogicState,
    pub outputs: Vec<WireOutput>, // switch to hopslotmap?
}

impl Wire {
    pub fn new() -> Wire {
        Wire { 
            state: LogicState::INVALID,
            outputs: Vec::new(),
        }
    }

    pub fn add_output(&mut self, output: WireOutput) {
        self.outputs.push(output);
    }

    pub fn set_state(&mut self, state: LogicState) {
        self.state = state;
    }

    /// update all logic blocks in the wire output, return a vector 
    /// of wires to be updated
    fn update(&self) -> Vec<WireOutput> {
        let mut next_wires: Vec<Wire>::new();
        for output_gate in self.outputs.iter_mut() {
            output_gate.logic_block.set_input(output.input_key, self.state);
            next_wires.push(output_gate.output)
        }
        next_wires
    }
}