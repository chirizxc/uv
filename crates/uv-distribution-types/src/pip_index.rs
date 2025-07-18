//! Compatibility structs for converting between [`IndexUrl`] and [`Index`]. These structs are
//! parsed and deserialized as [`IndexUrl`], but are stored as [`Index`] with the appropriate
//! flags set.

use serde::{Deserialize, Deserializer, Serialize};
#[cfg(feature = "schemars")]
use std::borrow::Cow;
use std::path::Path;

use crate::{Index, IndexUrl};

macro_rules! impl_index {
    ($name:ident, $from:expr) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $name(Index);

        impl $name {
            pub fn relative_to(self, root_dir: &Path) -> Result<Self, crate::IndexUrlError> {
                Ok(Self(self.0.relative_to(root_dir)?))
            }
        }

        impl From<$name> for Index {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<Index> for $name {
            fn from(value: Index) -> Self {
                Self(value)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.0.url().serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: Deserializer<'de>,
            {
                IndexUrl::deserialize(deserializer).map($from).map(Self)
            }
        }

        #[cfg(feature = "schemars")]
        impl schemars::JsonSchema for $name {
            fn schema_name() -> Cow<'static, str> {
                IndexUrl::schema_name()
            }

            fn json_schema(
                generator: &mut schemars::generate::SchemaGenerator,
            ) -> schemars::Schema {
                IndexUrl::json_schema(generator)
            }
        }
    };
}

impl_index!(PipIndex, Index::from_index_url);
impl_index!(PipExtraIndex, Index::from_extra_index_url);
impl_index!(PipFindLinks, Index::from_find_links);
