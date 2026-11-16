// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-boot-failure-recovery: boot failure recovery component model.
//!
//! Real implementation: recoverys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recovery {
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
pub enum BootFailureRecoveryError {
    EmptyRecoveryName,
    DuplicateRecovery(String),
    EmptyEventSubject,
}

impl fmt::Display for BootFailureRecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRecoveryName => write!(f, "recovery name must not be empty"),
            Self::DuplicateRecovery(id) => write!(f, "duplicate recovery {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BootFailureRecoveryError {}

pub struct BootFailureRecovery {
    pub name: String,
    recoverys: BTreeMap<String, Recovery>,
    events: BTreeMap<String, Event>,
}

impl BootFailureRecovery {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            recoverys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_recovery(&mut self, item: Recovery) -> Result<(), BootFailureRecoveryError> {
        if item.name.trim().is_empty() {
            return Err(BootFailureRecoveryError::EmptyRecoveryName);
        }
        let id = item.id.clone();
        if self.recoverys.contains_key(&id) {
            return Err(BootFailureRecoveryError::DuplicateRecovery(id.clone()));
        }
        self.recoverys.insert(id, item);
        Ok(())
    }

    pub fn recovery_count(&self) -> usize {
        self.recoverys.len()
    }

    pub fn get_recovery(&self, id: &str) -> Option<&Recovery> {
        self.recoverys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BootFailureRecoveryError> {
        if event.subject.trim().is_empty() {
            return Err(BootFailureRecoveryError::EmptyEventSubject);
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

    fn sample_recovery(id: &str, name: &str) -> Recovery {
        Recovery {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = BootFailureRecovery::new("BootFailureRecovery");
        assert_eq!(m.name, "BootFailureRecovery");
        assert_eq!(m.recovery_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_recovery() {
        let mut m = BootFailureRecovery::new("BootFailureRecovery");
        m.add_recovery(sample_recovery("a1", "Alpha")).unwrap();
        assert_eq!(m.recovery_count(), 1);
        assert!(m.get_recovery("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = BootFailureRecovery::new("BootFailureRecovery");
        assert!(m.add_recovery(sample_recovery("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = BootFailureRecovery::new("BootFailureRecovery");
        m.add_recovery(sample_recovery("x", "One")).unwrap();
        assert!(m.add_recovery(sample_recovery("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = BootFailureRecovery::new("BootFailureRecovery");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
