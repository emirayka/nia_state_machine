use std::fmt::Debug;

use crate::state_machine_node_id::StateMachineNodeId;

#[derive(Clone, Debug)]
pub enum StateMachineNode<K, V>
    where K: Clone + Debug + PartialEq + Eq,
          V: Clone + Debug
{
    Ordinary(Vec<(K, StateMachineNodeId)>),
    End(V),
}

impl<K, V> StateMachineNode<K, V>
    where K: Clone + Debug + PartialEq + Eq,
          V: Clone + Debug
{
    pub fn ordinary() -> StateMachineNode<K, V> {
        StateMachineNode::Ordinary(Vec::new())
    }

    pub fn end(v: V) -> StateMachineNode<K, V> {
        StateMachineNode::End(v)
    }
}