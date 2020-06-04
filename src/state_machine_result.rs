use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateMachineResult<K, V>
    where K: Clone + Debug + PartialEq + Eq,
          V: Clone + Debug
{
    Transition(),
    Fallback(Vec<K>),
    Excited(V),
}

impl<K, V> StateMachineResult<K, V>
    where K: Clone + Debug + PartialEq + Eq,
          V: Clone + Debug
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