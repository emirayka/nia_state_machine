use std::hash::Hash;

use crate::state_machine_node_id::StateMachineNodeId;
use crate::state_machine_node::StateMachineNode;
use crate::state_machine_node_arena::StateMachineNodeArena;
use crate::state_machine_result::StateMachineResult;

pub struct StateMachine<K, V>
    where K: Clone + PartialEq + Eq + Hash,
          V: Clone
{
    arena: StateMachineNodeArena<K, V>,
    current_node_id: StateMachineNodeId,
    root_node_id: StateMachineNodeId,
    current_path: Vec<K>,
}

impl<K, V> StateMachine<K, V>
    where K: Clone + PartialEq + Eq + Hash,
          V: Clone
{
    pub fn new() -> StateMachine<K, V> {
        let mut arena = StateMachineNodeArena::new();
        let root_node_id = arena.make_new_ordinary_node();
        let current_node_id = root_node_id;
        let current_path = Vec::new();

        StateMachine {
            arena,
            current_node_id,
            root_node_id,
            current_path,
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

                    self.arena.add_next_node(current_node_id, key, next_node_id)?;

                    next_node_id
                }
            }
        }

        let end_node_id = self.arena.make_new_end_node(value);
        self.arena.add_next_node(current_node_id, last_key, end_node_id)?;

        Ok(())
    }

    pub fn excite(&mut self, key: K) -> StateMachineResult<K, V> {
        let current_node_id = self.current_node_id;
        let next_node_id = self.arena.get_next_node(current_node_id, &key);

        match next_node_id {
            Some(next_node_id) => {
                match self.arena.get_node(next_node_id) {
                    StateMachineNode::Ordinary(_) => {
                        self.current_node_id = next_node_id;
                        self.current_path.push(key);

                        StateMachineResult::Transition()
                    },
                    StateMachineNode::End(v) => {
                        self.current_node_id = self.root_node_id;
                        self.current_path.clear();

                        StateMachineResult::Excited(v.clone())
                    }
                }
            }
            _ => {
                self.current_node_id = self.root_node_id;

                let result = StateMachineResult::fallback(
                    self.current_path.clone()
                );

                self.current_path.clear();

                result
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

        state_machine.add(vec!(1, 2, 3), 1).unwrap();
        state_machine.add(vec!(1, 2, 4), 2).unwrap();
        state_machine.add(vec!(1, 4), 3).unwrap();

        assert_eq!(StateMachineResult::Transition(), state_machine.excite(1));
        assert_eq!(StateMachineResult::Transition(), state_machine.excite(2));
        assert_eq!(StateMachineResult::Excited(1), state_machine.excite(3));

        assert_eq!(StateMachineResult::Transition(), state_machine.excite(1));
        assert_eq!(StateMachineResult::Transition(), state_machine.excite(2));
        assert_eq!(StateMachineResult::Excited(2), state_machine.excite(4));

        assert_eq!(StateMachineResult::Transition(), state_machine.excite(1));
        assert_eq!(StateMachineResult::Excited(3), state_machine.excite(4));
    }

    #[test]
    fn fallbacks_to_root_state_if_next_node_with_the_key_was_not_found() {
        let mut state_machine: StateMachine<i32, i32> = StateMachine::new();

        state_machine.add(vec!(1, 2, 3), 1).unwrap();
        state_machine.add(vec!(4), 2).unwrap();

        assert_eq!(StateMachineResult::Transition(), state_machine.excite(1));
        assert_eq!(StateMachineResult::Transition(), state_machine.excite(2));
        assert_eq!(StateMachineResult::Fallback(vec!(1, 2)), state_machine.excite(1));

        assert_eq!(StateMachineResult::Excited(2), state_machine.excite(4));
        assert_eq!(StateMachineResult::Fallback(vec!()), state_machine.excite(2));
    }

    #[test]
    fn fallbacks_to_root_state_after_returning_value() {
        let mut state_machine: StateMachine<i32, i32> = StateMachine::new();

        state_machine.add(vec!(1, 2, 3), 1).unwrap();
        state_machine.add(vec!(4), 2).unwrap();

        assert_eq!(StateMachineResult::Transition(), state_machine.excite(1));
        assert_eq!(StateMachineResult::Transition(), state_machine.excite(2));
        assert_eq!(StateMachineResult::Excited(1), state_machine.excite(3));

        assert_eq!(StateMachineResult::Excited(2), state_machine.excite(4));
    }
}