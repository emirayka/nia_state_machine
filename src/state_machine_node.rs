use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;

use crate::state_machine_node_id::StateMachineNodeId;

#[derive(Clone, Debug)]
pub enum StateMachineNode<K, V>
    where K: Clone + Debug + PartialEq + Eq + Hash,
          V: Clone + Debug
{
    Ordinary(HashMap<K, StateMachineNodeId>),
    End(V),
}

impl<K, V> StateMachineNode<K, V>
    where K: Clone + Debug + PartialEq + Eq + Hash,
          V: Clone + Debug
{
    pub fn ordinary() -> StateMachineNode<K, V> {
        StateMachineNode::Ordinary(HashMap::new())
    }

    pub fn end(v: V) -> StateMachineNode<K, V> {
        StateMachineNode::End(v)
    }
}