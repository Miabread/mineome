use crate::internal_prelude::*;

/**
    Namespaced Ids are used as identifiers for various resources in Minecraft.
    See the [wiki page](https://minecraft.gamepedia.com/Namespaced_ID) for more details.
*/
#[derive(Clone, PartialEq)]
pub struct NamespacedId {
    namespace: String,
    id: String,
    is_tag: bool,
}

impl NamespacedId {
    /// Create a new NamespacedId
    pub fn new(is_tag: bool, namespace: &str, id: &str) -> Self {
        NamespacedId {
            namespace: namespace.to_string(),
            id: id.to_string(),
            is_tag,
        }
    }

    /// Get a instance of mineome's marker for invalid NamespacedIds
    pub fn invalid() -> Self {
        Self::new(false, "mineome", "{invalid}")
    }
}

impl From<&str> for NamespacedId {
    fn from(string: &str) -> Self {
        // See if tag and remove # if so
        let is_tag = dbg!(string.starts_with('#'));
        let string = dbg!(if is_tag { &string[1..] } else { string });

        // Parse the rest of the namespace
        let mut split = string.split(':').take(2);
        match (split.next(), split.next()) {
            (Some(ns), Some(id)) => NamespacedId::new(is_tag, ns, id),
            (Some(id), None) => NamespacedId::new(is_tag, "minecraft", id),
            _ => NamespacedId::invalid(),
        }
    }
}

#[cfg(test)]
#[test]
fn namespace_id_from_str_test() {
    assert_eq!(
        NamespacedId::from("foo:bar"),
        NamespacedId::new(false, "foo", "bar"),
    );
    assert_eq!(
        NamespacedId::from("#foo:bar/baz"),
        NamespacedId::new(true, "foo", "bar/baz"),
    );
    assert_eq!(
        NamespacedId::from("abcdef"),
        NamespacedId::new(false, "minecraft", "abcdef"),
    );
}

impl Display for NamespacedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}:{}", self.namespace, self.id)
    }
}

impl Debug for NamespacedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}:{}", self.namespace, self.id)
    }
}

pub enum NamespacedIdValidationError {
    InvalidCharInNamespace { kind: char, position: usize },
    InvalidCharInId { kind: char, position: usize },
}

impl Validate for NamespacedId {
    type Error = NamespacedIdValidationError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut errors = Vec::new();

        // Validate namespace [a-zA-Z0-9_-]
        for (position, kind) in self.namespace.chars().enumerate() {
            if !(kind.is_alphanumeric() || kind == '_' || kind == '-') {
                errors.push(NamespacedIdValidationError::InvalidCharInNamespace { kind, position })
            }
        }

        // Validate id [a-zA-Z0-9_./-]
        for (position, kind) in self.id.chars().enumerate() {
            if !(kind.is_alphanumeric() || kind == '_' || kind == '-' || kind == '/' || kind == '.')
            {
                errors.push(NamespacedIdValidationError::InvalidCharInId { kind, position })
            }
        }

        errors
    }
}
