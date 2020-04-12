#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StateMachineNodeId {
    id: usize,
}

impl StateMachineNodeId {
    pub fn new(id: usize) -> StateMachineNodeId {
        StateMachineNodeId {
            id
        }
    }
}