use std::ops::Not;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicState {
    LOW = 0,
    HIGH = 1,
    INVALID,
}

impl Not for LogicState {
    type Output = Self;

    fn not(self) -> Self {
        if self == LogicState::LOW {
            Self::HIGH
        } else {
            Self::LOW
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_not_state() {
        assert_eq!(LogicState::HIGH.not(), LogicState::LOW);
        assert_eq!(LogicState::LOW.not(), LogicState::HIGH);
    }
}