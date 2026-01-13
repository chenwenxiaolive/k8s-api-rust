//! Common types used across Kubernetes API

/// UID is a type that holds unique ID values.
pub type UID = String;

/// NamespacedName comprises a namespace and name that uniquely identify a resource.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NamespacedName {
    pub namespace: String,
    pub name: String,
}

impl NamespacedName {
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    /// Creates a NamespacedName with an empty namespace (for cluster-scoped resources).
    pub fn cluster_scoped(name: impl Into<String>) -> Self {
        Self {
            namespace: String::new(),
            name: name.into(),
        }
    }
}

impl std::fmt::Display for NamespacedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.namespace.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{}/{}", self.namespace, self.name)
        }
    }
}

impl From<(&str, &str)> for NamespacedName {
    fn from((namespace, name): (&str, &str)) -> Self {
        Self::new(namespace, name)
    }
}
