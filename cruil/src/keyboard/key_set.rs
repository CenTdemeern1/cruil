use super::keys::*;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

/// A set of keyboard keys and modifier keys.
#[derive(Debug, Clone, Default)]
pub struct KeySet {
    pub modifiers: Modifiers,
    pub keys: HashSet<Key>,
}

impl KeySet {
    /// Gets the (asymmetric) difference with `other`; so everything that is in `self` but not in `other`.
    pub fn difference(&self, other: &Self) -> KeySet {
        KeySet {
            modifiers: self.modifiers.difference(other.modifiers),
            keys: self.keys.difference(&other.keys).copied().collect(),
        }
    }

    /// Gets the symmetric difference with `other`; so everything that is in exactly one of the two sets, like XOR.
    pub fn symmetric_difference(&self, other: &Self) -> KeySet {
        KeySet {
            modifiers: self.modifiers.symmetric_difference(other.modifiers),
            keys: self
                .keys
                .symmetric_difference(&other.keys)
                .copied()
                .collect(),
        }
    }

    /// Returns whether there are no keys in this set.
    pub fn is_empty(&self) -> bool {
        self.modifiers.is_empty() && self.keys.is_empty()
    }

    /// Returns whether there are any keys in this set.
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

    /// Returns whether all the keys in `self` are also in `superset`.
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
