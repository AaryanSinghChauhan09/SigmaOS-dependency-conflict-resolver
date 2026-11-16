// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dependency-conflict-resolver: dependency conflict resolver component model.
//!
//! Real implementation: resolvers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolver {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyConflictResolverError {
    EmptyResolverName,
    DuplicateResolver(String),
    EmptyEventSubject,
}

impl fmt::Display for DependencyConflictResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyResolverName => write!(f, "resolver name must not be empty"),
            Self::DuplicateResolver(id) => write!(f, "duplicate resolver {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DependencyConflictResolverError {}

pub struct DependencyConflictResolver {
    pub name: String,
    resolvers: BTreeMap<String, Resolver>,
    events: BTreeMap<String, Event>,
}

impl DependencyConflictResolver {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            resolvers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_resolver(&mut self, item: Resolver) -> Result<(), DependencyConflictResolverError> {
        if item.name.trim().is_empty() {
            return Err(DependencyConflictResolverError::EmptyResolverName);
        }
        let id = item.id.clone();
        if self.resolvers.contains_key(&id) {
            return Err(DependencyConflictResolverError::DuplicateResolver(id.clone()));
        }
        self.resolvers.insert(id, item);
        Ok(())
    }

    pub fn resolver_count(&self) -> usize {
        self.resolvers.len()
    }

    pub fn get_resolver(&self, id: &str) -> Option<&Resolver> {
        self.resolvers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DependencyConflictResolverError> {
        if event.subject.trim().is_empty() {
            return Err(DependencyConflictResolverError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_resolver(id: &str, name: &str) -> Resolver {
        Resolver {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DependencyConflictResolver::new("DependencyConflictResolver");
        assert_eq!(m.name, "DependencyConflictResolver");
        assert_eq!(m.resolver_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_resolver() {
        let mut m = DependencyConflictResolver::new("DependencyConflictResolver");
        m.add_resolver(sample_resolver("a1", "Alpha")).unwrap();
        assert_eq!(m.resolver_count(), 1);
        assert!(m.get_resolver("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DependencyConflictResolver::new("DependencyConflictResolver");
        assert!(m.add_resolver(sample_resolver("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DependencyConflictResolver::new("DependencyConflictResolver");
        m.add_resolver(sample_resolver("x", "One")).unwrap();
        assert!(m.add_resolver(sample_resolver("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DependencyConflictResolver::new("DependencyConflictResolver");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
