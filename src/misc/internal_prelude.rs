//! Used to wrangle common internally used imports
pub use {
    super::{namespaced_id::NamespacedId, validate::Validate},
    serde::{Deserialize, Serialize},
    std::{
        collections::HashMap,
        fmt::{Debug, Display, Formatter, Result as FmtResult},
    },
};
