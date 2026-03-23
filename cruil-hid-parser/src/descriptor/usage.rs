#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Usage {
    page: u16,
    id: u16,
}

impl Usage {
    pub fn new(page: u16, id: u16) -> Self {
        Usage { page, id }
    }
}

impl From<Usage> for u32 {
    fn from(value: Usage) -> Self {
        ((value.page as u32) << 16) | value.id as u32
    }
}

impl From<u32> for Usage {
    fn from(value: u32) -> Self {
        Usage {
            page: (value >> 16) as u16,
            id: (value & 0xFFFF) as u16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsageSet {
    List(Vec<Usage>),
    Range(u32, Option<u32>),
}

impl Default for UsageSet {
    fn default() -> Self {
        UsageSet::List(vec![])
    }
}

impl UsageSet {
    /// Gets the nth usage in this set. Returns `None` if this is padding.
    pub fn nth(&self, n: u32) -> Option<Usage> {
        match self {
            UsageSet::List(usages) => usages
                .get(usages.len().saturating_sub(1).min(n as usize))
                .copied(),
            &UsageSet::Range(start, end) => {
                let usage = start + n;
                Some(end.unwrap_or(usage).min(usage).into())
            }
        }
    }
}
