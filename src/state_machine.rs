use std::hash::Hash;
use std::collections::HashMap;

use crate::state_machine_node_id::StateMachineNodeId;
use crate::state_machine_node::StateMachineNode;
use crate::state_machine_node_arena::StateMachineNodeArena;

pub struct StateMachine<K, V>
    where K: PartialEq + Eq + Hash
{
    arena: StateMachineNodeArena<K, V>,
    current_node_id: StateMachineNodeId,
    root_node_id: StateMachineNodeId,
}

impl<K, V> StateMachine<K, V>
    where K: PartialEq + Eq + Hash
{
    pub fn new() -> StateMachine<K, V> {
        let mut arena = StateMachineNodeArena::new();
        let root_node_id = arena.make_new_ordinary_node();
        let current_node_id = root_node_id;

        StateMachine {
            arena,
            current_node_id,
            root_node_id,
        }
    }

    pub fn add(&mut self, keys: Vec<K>, value: V) -> Result<(), ()> {
        let mut current_node_id = self.root_node_id;

        let mut keys = keys;
        let last_key = keys.remove(keys.len() - 1);

        for key in keys {
            current_node_id = match self.arena.get_next_node(current_node_id, &key) {
                Some(next_node_id) => next_node_id,
                None => {
                    let next_node_id = self.arena.make_new_ordinary_node();

                    self.arena.add_next_node(current_node_id, key, next_node_id);

                    next_node_id
                }
            }
        }

        let end_node_id = self.arena.make_new_end_node(value);
        self.arena.add_next_node(current_node_id, last_key, end_node_id);

        Ok(())
    }

    pub fn excite(&mut self, key: &K) -> Option<&V> {
        let current_node_id = self.current_node_id;
        let next_node_id = self.arena.get_next_node(current_node_id, key);

        match next_node_id {
            Some(next_node_id) => {
                match self.arena.get_node(next_node_id) {
                    StateMachineNode::Ordinary(_) => {
                        self.current_node_id = next_node_id;
                        None
                    },
                    StateMachineNode::End(v) => {
                        self.current_node_id = self.root_node_id;
                        Some(v)
                    }
                }
            }
            _ => {
                self.current_node_id = self.root_node_id;
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_paths() {
        let mut state_machine: StateMachine<i32, i32> = StateMachine::new();

        state_machine.add(vec!(1, 2, 3), 1);
        state_machine.add(vec!(1, 2, 4), 2);
        state_machine.add(vec!(1, 4), 3);

        assert_eq!(None, state_machine.excite(&1));
        assert_eq!(None, state_machine.excite(&2));
        assert_eq!(Some(&1), state_machine.excite(&3));

        assert_eq!(None, state_machine.excite(&1));
        assert_eq!(None, state_machine.excite(&2));
        assert_eq!(Some(&2), state_machine.excite(&4));

        assert_eq!(None, state_machine.excite(&1));
        assert_eq!(Some(&3), state_machine.excite(&4));
    }

    #[test]
    fn fallbacks_to_root_state_if_next_node_with_the_key_was_not_found() {
        let mut state_machine: StateMachine<i32, i32> = StateMachine::new();

        state_machine.add(vec!(1, 2, 3), 1);
        state_machine.add(vec!(4), 2);

        assert_eq!(None, state_machine.excite(&1));
        assert_eq!(None, state_machine.excite(&2));
        assert_eq!(None, state_machine.excite(&1));

        assert_eq!(Some(&2), state_machine.excite(&4));
    }

    #[test]
    fn fallbacks_to_root_state_after_returning_value() {
        let mut state_machine: StateMachine<i32, i32> = StateMachine::new();

        state_machine.add(vec!(1, 2, 3), 1);
        state_machine.add(vec!(4), 2);

        assert_eq!(None, state_machine.excite(&1));
        assert_eq!(None, state_machine.excite(&2));
        assert_eq!(Some(&1), state_machine.excite(&3));

        assert_eq!(Some(&2), state_machine.excite(&4));
    }
}