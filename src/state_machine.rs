use std::fmt::Debug;

use log::{
    debug
};

use crate::state_machine_node_id::StateMachineNodeId;
use crate::state_machine_node::StateMachineNode;
use crate::state_machine_node_arena::StateMachineNodeArena;
use crate::state_machine_result::StateMachineResult;

#[derive(Clone, Debug)]
pub struct StateMachine<K, V>
    where K: Clone + Debug + PartialEq + Eq,
          V: Clone + Debug
{
    arena: StateMachineNodeArena<K, V>,
    current_node_id: StateMachineNodeId,
    root_node_id: StateMachineNodeId,
    current_path: Vec<K>,
}

impl<K, V> StateMachine<K, V>
    where K: Clone + Debug + PartialEq + Eq,
          V: Clone + Debug
{
    pub fn new() -> StateMachine<K, V> {
        let mut arena = StateMachineNodeArena::new();
        let root_node_id = arena.make_new_ordinary_node();
        let current_node_id = root_node_id;
        let current_path = Vec::new();

        let state_machine = StateMachine {
            arena,
            current_node_id,
            root_node_id,
            current_path,
        };

        debug!("Constructed state machine:\n");
        debug!("{:?}", state_machine);

        state_machine
    }

    pub fn add(&mut self, keys: Vec<K>, value: V) -> Result<(), ()> {
        debug!("Adding path: {:?} -> {:?}", keys, value);

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
        debug!("Exciting with key: {:?}.", key);
        debug!("Current path: {:?}.", self.current_path);

        let current_node_id = self.current_node_id;
        let next_node_id = self.arena.get_next_node(current_node_id, &key);

        match next_node_id {
            Some(next_node_id) => {
                debug!("Found next node: {:?}.", next_node_id);

                match self.arena.get_node(next_node_id) {
                    StateMachineNode::Ordinary(_) => {
                        debug!("It's an ordinary node. Transition to {:?}.", next_node_id);

                        self.current_node_id = next_node_id;
                        self.current_path.push(key);

                        debug!("Current path: {:?}.", self.current_path);

                        StateMachineResult::Transition()
                    },
                    StateMachineNode::End(v) => {
                        debug!("It's an end node. Transition to root node.");

                        self.current_node_id = self.root_node_id;
                        self.current_path.clear();

                        debug!("Current path: {:?}.", self.current_path);

                        StateMachineResult::Excited(v.clone())
                    }
                }
            }
            _ => {
                debug!("No next node found for key: {:?}. Fallback to the root node.", key);
                debug!("Fallback value: {:?}.", self.current_path);

                self.current_node_id = self.root_node_id;
                self.current_path.push(key);

                debug!("Current path: {:?}.", self.current_path);

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
    fn works_on_one_item_path() {
        let mut state_machine: StateMachine<i32, i32> = StateMachine::new();

        state_machine.add(vec!(1), 11).unwrap();
        state_machine.add(vec!(2), 12).unwrap();
        state_machine.add(vec!(3), 13).unwrap();

        assert_eq!(StateMachineResult::Excited(11), state_machine.excite(1));
        assert_eq!(StateMachineResult::Excited(12), state_machine.excite(2));
        assert_eq!(StateMachineResult::Excited(13), state_machine.excite(3));
    }

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
        assert_eq!(StateMachineResult::Fallback(vec!(1, 2, 1)), state_machine.excite(1));

        assert_eq!(StateMachineResult::Excited(2), state_machine.excite(4));
        assert_eq!(StateMachineResult::Fallback(vec!(2)), state_machine.excite(2));
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