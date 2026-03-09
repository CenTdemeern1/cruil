use super::keys::*;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, Default)]
pub struct KeySet {
    pub modifiers: Modifiers,
    pub keys: HashSet<Key>,
}

impl KeySet {
    pub fn difference(&self, other: &Self) -> KeySet {
        KeySet {
            modifiers: self.modifiers.difference(other.modifiers),
            keys: self.keys.difference(&other.keys).copied().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty() && self.keys.is_empty()
    }

    pub fn any(&self) -> bool {
        !self.is_empty()
    }

    /// Returns whether the set contains the given key.
    pub fn contains_key(&self, key: &Key) -> bool {
        self.keys.contains(key)
    }

    /// Returns whether the set contains the given modifier keys.
    pub fn contains_modifiers(&self, modifiers: Modifiers) -> bool {
        self.modifiers.contains(modifiers)
    }

    pub fn is_subset(&self, superset: &KeySet) -> bool {
        superset.modifiers.contains(self.modifiers) && self.keys.is_subset(&superset.keys)
    }
}

impl Display for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "None");
        }
        let mut keys: Vec<Key> = self.keys.iter().copied().collect();
        keys.sort_unstable();
        let names: Vec<String> = [format!("{}", self.modifiers)]
            .into_iter()
            .skip_while(String::is_empty)
            .chain(keys.into_iter().map(|key| format!("{key:?}")))
            .collect();
        write!(f, "{}", names.join("+"))
    }
}
