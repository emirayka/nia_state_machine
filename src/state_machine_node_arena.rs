use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;

use crate::state_machine_node_id::StateMachineNodeId;
use crate::state_machine_node::StateMachineNode;

#[derive(Clone, Debug)]
pub struct StateMachineNodeArena<K, V>
    where K: Clone + Debug + PartialEq + Eq + Hash,
          V: Clone + Debug
{
    nodes: HashMap<StateMachineNodeId, StateMachineNode<K, V>>,
    next_node_id: usize,
}

impl<K, V> StateMachineNodeArena<K, V>
    where K: Clone + Debug + PartialEq + Eq + Hash,
          V: Clone + Debug
{
    pub fn new() -> StateMachineNodeArena<K, V> {
        StateMachineNodeArena {
            nodes: HashMap::new(),
            next_node_id: 0,
        }
    }

    fn register_node(&mut self, node: StateMachineNode<K, V>) -> StateMachineNodeId {
        let node_id = StateMachineNodeId::new(self.next_node_id);
        self.nodes.insert(node_id, node);

        self.next_node_id += 1;

        node_id
    }

    pub fn make_new_ordinary_node(&mut self) -> StateMachineNodeId {
        let node = StateMachineNode::Ordinary(HashMap::new());

        self.register_node(node)
    }

    pub fn make_new_end_node(&mut self, v: V) -> StateMachineNodeId {
        let node = StateMachineNode::End(v);

        self.register_node(node)
    }

    pub fn get_node(&self, node_id: StateMachineNodeId) -> &StateMachineNode<K, V> {
        self.nodes.get(&node_id).unwrap()
    }

    pub fn get_next_node(&self, node_id: StateMachineNodeId, key: &K) -> Option<StateMachineNodeId> {
        let node = self.nodes.get(&node_id).unwrap();

        match node {
            StateMachineNode::Ordinary(next) => {
                match next.get(key) {
                    Some(node_id) => Some(*node_id),
                    _ => None
                }
            }
            _ => {
                None
            }
        }
    }

    pub fn add_next_node(&mut self, node_id: StateMachineNodeId, key: K, next_node_id: StateMachineNodeId) -> Result<(), ()> {
        let current_node = match self.nodes.get_mut(&node_id) {
            Some(node) => node,
            _ => return Err(())
        };

        match current_node {
            StateMachineNode::Ordinary(next_nodes) => {
                next_nodes.insert(key, next_node_id);
                Ok(())
            }
            _ => Err(())
        }
    }
}