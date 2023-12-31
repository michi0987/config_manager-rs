use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub enum GroupOperation {
    AddEntry(String, String),
    RemoveEntry(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroup {
    #[serde(skip)]
    name: String,
    #[serde(flatten)]
    pub entries: HashMap<String, String>,
}

impl ConfigurationGroup {
    pub fn new(name: &str) -> Self {
        ConfigurationGroup { name: name.to_string(), entries: HashMap::new() }
    }
    pub fn set_entries(&mut self, entries: HashMap<String, String>) {
        self.entries = entries;
    }
    pub fn add_entry (&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }
    pub fn remove_entry (&mut self, key: &str) {
        self.entries.remove(key);
    }
    pub fn process(&mut self, operation: GroupOperation) {
        match operation {
            GroupOperation::AddEntry(n, v) => self.entries.insert(n, v),
            GroupOperation::RemoveEntry(key) => self.entries.remove(&key),
        };
    }
}


impl Into<crate::proto::ConfigurationGroup> for ConfigurationGroup {
    fn into(self) -> crate::proto::ConfigurationGroup {
        crate::proto::ConfigurationGroup {
            entries: self.entries.into_iter().map(|(k, v)| (k,v)).collect(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_group(){
        let mut group = ConfigurationGroup::new("group_name");
        assert_eq!(group, ConfigurationGroup{ name: "group_name".to_string(), entries: HashMap::new()});

        group.process(GroupOperation::AddEntry("key".to_string(), "value".to_string()));

        assert!(group.entries.contains_key(&"key".to_string()));
    }

    #[test]
    fn test_add_entry() {
        let mut group = ConfigurationGroup::new("test_group");
        group.process(GroupOperation::AddEntry("key1".to_string(), "value1".to_string()));
        assert_eq!(group.entries.get(&"key1".to_string()), Some(&"value1".to_string()));
    }

    #[test]
    fn test_remove_entry() {
        let mut group = ConfigurationGroup::new("test_group");
        group.process(GroupOperation::AddEntry("key1".to_string(), "value1".to_string()));
        group.process(GroupOperation::RemoveEntry("key1".to_string()));
        assert!(!group.entries.contains_key(&"key1".to_string()));
    }

    #[test]
    fn test_remove_nonexistent_entry() {
        let mut group = ConfigurationGroup::new("test_group");
        group.process(GroupOperation::RemoveEntry("nonexistent".to_string()));
        assert_eq!(group.entries.len(), 0);
    }

    #[test]
    fn test_multiple_operations() {
        let mut group = ConfigurationGroup::new("test_group");
        group.process(GroupOperation::AddEntry("key1".to_string(), "value1".to_string()));
        group.process(GroupOperation::AddEntry("key2".to_string(), "value2".to_string()));
        assert_eq!(group.entries.len(), 2);
        group.process(GroupOperation::RemoveEntry("key1".to_string()));
        assert!(!group.entries.contains_key(&"key1".to_string()));
        assert!(group.entries.contains_key(&"key2".to_string()));
    }

}