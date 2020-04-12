use std::hash::Hash;
use std::collections::HashMap;

use crate::state_machine_node_id::StateMachineNodeId;

pub enum StateMachineNode<K, V>
    where K: PartialEq + Eq + Hash
{
    Ordinary(HashMap<K, StateMachineNodeId>),
    End(V),
}

impl<K, V> StateMachineNode<K, V>
    where K: PartialEq + Eq + Hash
{
    pub fn ordinary() -> StateMachineNode<K, V> {
        StateMachineNode::Ordinary(HashMap::new())
    }

    pub fn end(v: V) -> StateMachineNode<K, V> {
        StateMachineNode::End(v)
    }
}