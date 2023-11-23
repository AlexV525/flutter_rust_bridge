use itertools::Itertools;
use serde::__private::ser::serialize_tagged_newtype;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;

/// The Rust files/modules/namespaces.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
#[serde(transparent)]
pub struct Namespace {
    // Represent via this, instead of `Vec<String>`, to avoid extra memory overhead
    joined_path: String,
}

impl Namespace {
    const SEP: &'static str = "::";
    const SELF_CRATE: &'static str = "crate";

    pub fn new(path: Vec<String>) -> Self {
        assert!((path.iter()).all(|item| !item.contains(Self::SEP)));
        Self::new_raw(path.join(Self::SEP))
    }

    pub fn new_raw(joined_path: String) -> Self {
        Self { joined_path }
    }

    pub fn new_self_crate(joined_path: String) -> Self {
        let sep = Self::SEP;
        let self_crate = Self::SELF_CRATE;

        assert!(!joined_path.starts_with(&format!("{self_crate}{sep}")));
        Self::new_raw(format!("{self_crate}{sep}{joined_path}"))
    }

    pub fn path(&self) -> Vec<&str> {
        self.joined_path.split(Self::SEP).collect()
    }

    pub fn path_exclude_self_crate(&self) -> Vec<&str> {
        let mut path = self.path();
        if path.first() == Some(&Self::SELF_CRATE) {
            path.remove(0);
        }
        path
    }

    pub fn to_pseudo_io_path(&self, extension: &str) -> PathBuf {
        PathBuf::from(&format!("{}.{extension}", self.path().join("/")))
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.joined_path)
    }
}

/// A name and the namespace it is in.
///
/// Usually, a name itself (say "Apple") is vague, since it can be `mod_a::Apple`
/// or `mod_b::Apple`. Instead, a namespace + name unambiguously determines the object.
// Note: Do NOT implement `display`, otherwise it is easy to misuse it.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct NamespacedName {
    pub namespace: Namespace,
    pub name: String,
}

impl NamespacedName {
    const SEP: &'static str = "/";

    pub fn new(namespace: Namespace, name: String) -> Self {
        Self { namespace, name }
    }
}

impl Serialize for NamespacedName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}{}{}", self.namespace, Self::SEP, self.name).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NamespacedName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let index = s.rfind(Self::SEP).unwrap();
        Ok(Self::new(
            Namespace::new_raw(s[..index].to_owned()),
            s[index + Self::SEP.len()..].to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_namespace_display() {
        assert_eq!(
            Namespace::new(vec!["crate".into(), "hello".into(), "world".into()]).to_string(),
            "crate::hello::world"
        );
    }

    #[test]
    pub fn test_namespaced_name_serialization() -> anyhow::Result<()> {
        let original = NamespacedName::new(Namespace::new_raw("a::b".into()), "c".into());
        let serialized = serde_json::to_string(&original)?;
        let recovered: NamespacedName = serde_json::from_str(&serialized)?;

        assert_eq!(serialized, r#""a::b/c""#);
        assert_eq!(original, recovered);

        Ok(())
    }

    #[test]
    pub fn test_to_pseudo_io_path() -> anyhow::Result<()> {
        assert_eq!(
            Namespace::new_raw("apple::orange".into()).to_pseudo_io_path("dart"),
            PathBuf::from("/apple/orange.dart")
        );
        Ok(())
    }
}
