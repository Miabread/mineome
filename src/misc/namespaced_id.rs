use crate::internal_prelude::*;

/**
    Namespaced Ids are used as identifiers for various resources in Minecraft.
    See the [wiki page](https://minecraft.gamepedia.com/Namespaced_ID) for more details.
*/
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NamespacedId {
    namespace: String,
    id: String,
    is_tag: bool,
}

impl NamespacedId {
    /// Create a new NamespacedId
    pub fn new(is_tag: bool, namespace: &str, id: &str) -> Result<Self, InvalidNamespacedIdError> {
        // Validate namespace [a-zA-Z0-9_-]
        for (position, kind) in namespace.chars().enumerate() {
            if !(kind.is_alphanumeric() || kind == '_' || kind == '-') {
                return Err(InvalidNamespacedIdError::InvalidCharInNamespace { kind, position });
            }
        }

        // Validate id [a-zA-Z0-9_./-]
        for (position, kind) in id.chars().enumerate() {
            if !(kind.is_alphanumeric() || kind == '_' || kind == '-' || kind == '/' || kind == '.')
            {
                return Err(InvalidNamespacedIdError::InvalidCharInId { kind, position });
            }
        }

        Ok(NamespacedId {
            namespace: namespace.to_string(),
            id: id.to_string(),
            is_tag,
        })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn is_tag(&self) -> bool {
        self.is_tag
    }
}

impl TryFrom<&str> for NamespacedId {
    type Error = InvalidNamespacedIdError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        // See if tag and remove # if so
        let is_tag = string.starts_with('#');
        let string = if is_tag { &string[1..] } else { string };

        // Parse the rest of the namespace
        let mut split = string.split(':').take(2);
        match (split.next(), split.next()) {
            (Some(ns), Some(id)) => NamespacedId::new(is_tag, ns, id),
            (Some(id), None) => NamespacedId::new(is_tag, "minecraft", id),
            _ => Err(InvalidNamespacedIdError::CouldNotParse {
                text: string.to_string(),
            }),
        }
    }
}

impl Display for NamespacedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}{}:{}",
            if self.is_tag { "#" } else { "" },
            self.namespace,
            self.id
        )
    }
}

impl Debug for NamespacedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}{}:{}",
            if self.is_tag { "#" } else { "" },
            self.namespace,
            self.id
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidNamespacedIdError {
    InvalidCharInNamespace { kind: char, position: usize },
    InvalidCharInId { kind: char, position: usize },
    CouldNotParse { text: String },
}

impl Error for InvalidNamespacedIdError {}

impl Display for InvalidNamespacedIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            InvalidNamespacedIdError::InvalidCharInNamespace { kind, position } => write!(
                f,
                "Invalid char {} in namespace at position {}",
                kind, position
            ),
            InvalidNamespacedIdError::InvalidCharInId { kind, position } => {
                write!(f, "Invalid char {} in id at position {}", kind, position)
            }
            InvalidNamespacedIdError::CouldNotParse { text } => {
                write!(f, "Could not parse {} as a NamespacedId", text)
            }
        }
    }
}

#[cfg(test)]
#[test]
fn namespace_id_from_str_test() -> Result<(), InvalidNamespacedIdError> {
    assert_eq!(
        NamespacedId::try_from("foo:bar")?,
        NamespacedId::new(false, "foo", "bar")?,
    );
    assert_eq!(
        NamespacedId::try_from("#foo:bar/baz")?,
        NamespacedId::new(true, "foo", "bar/baz")?,
    );
    assert_eq!(
        NamespacedId::try_from("abcdef")?,
        NamespacedId::new(false, "minecraft", "abcdef")?,
    );
    Ok(())
}

impl Serialize for NamespacedId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct NamespacedIdVisitor;

impl<'de> serde::de::Visitor<'de> for NamespacedIdVisitor {
    type Value = NamespacedId;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("an NamespacedId")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.try_into().map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for NamespacedId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NamespacedIdVisitor)
    }
}

pub(crate) mod recipe_advancement_serde {
    use super::{
        NamespacedId,
        InvalidNamespacedIdError
    };
    use serde::{
        ser::{SerializeStruct, Serializer},
        de::{self, MapAccess, Visitor},
        Deserializer
    };
    use std::fmt;

    pub fn serialize<S>(namespaced_id: &NamespacedId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("NamespacedId", 1)?;
        state.serialize_field(if namespaced_id.is_tag {"tag"} else {"item"},&namespaced_id.to_string())?;
        state.end()
    }

    fn parse_namespace_and_id(string: &str) -> Result<(&str, &str), InvalidNamespacedIdError> {
        let mut split = string.split(':').take(2);
        match (split.next(), split.next()) {
            (Some(ns), Some(id)) => Ok((ns, id)),
            _ => Err(InvalidNamespacedIdError::CouldNotParse {
                text: string.to_string(),
            }),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NamespacedId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(NamespacedIdVisitor)
    }

    struct NamespacedIdVisitor;

    impl<'de> Visitor<'de> for NamespacedIdVisitor {
        type Value = NamespacedId;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter
                .write_str(r#"an object with a "item" or "tag" field containing a NamespacedId"#)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let (is_tag, value) = match map.next_entry()? {
                Some((key, value)) => (
                    match key {
                        "item" => false,
                        "tag" => true,
                        _ => Err(de::Error::unknown_field(value, &["item", "tag"]))?,
                    },
                    value,
                ),
                None => Err(de::Error::invalid_length(0, &"a length of 1"))?,
            };

            if let Some(key) = map.next_key()? {
                Err(de::Error::unknown_field(key, &["item", "tag"]))?
            }

            let (ns, id) = parse_namespace_and_id(value).map_err(de::Error::custom)?;
            Ok(NamespacedId::new(is_tag, ns, id).unwrap())
        }
    }
}
