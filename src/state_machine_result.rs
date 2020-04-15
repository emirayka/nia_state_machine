use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateMachineResult<K, V>
    where K: Clone + PartialEq + Eq + Hash,
          V: Clone
{
    Transition(),
    Fallback(Vec<K>),
    Excited(V),
}

impl<K, V> StateMachineResult<K, V>
    where K: Clone + PartialEq + Eq + Hash,
          V: Clone
{
    pub fn transition() -> StateMachineResult<K, V> {
        StateMachineResult::Transition()
    }

    pub fn fallback(fallback_value: Vec<K>) -> StateMachineResult<K, V> {
        StateMachineResult::Fallback(fallback_value)
    }

    pub fn excited(v: V) -> StateMachineResult<K, V> {
        StateMachineResult::Excited(v)
    }
}