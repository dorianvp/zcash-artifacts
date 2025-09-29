use crate::{BuildRecipe, VersionProbe, zcashd::spec_zcashd};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ServiceId(std::borrow::Cow<'static, str>);

impl ServiceId {
    pub const fn new_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
    pub fn new_owned(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

// TODO: Add constants for `zebrad`, `lightwaleltd` and `zainod`
pub const ZCASHD: ServiceId = ServiceId::new_static("zcashd");
// pub const ZEBRAD: ServiceId = ServiceId::new_static("zebrad");

/// Describes how to handle a service: what binary to expect, how to find it, etc.
pub struct ToolSpec {
    pub id: ServiceId,

    /// Candidate binary names per platform (used to locate executables in archives or after builds).
    pub binary_names: fn(&str /* platform triple */) -> &'static [&'static str],

    /// Default relative path to the built binary inside a repo (for local-build).
    pub default_expected_output: std::path::PathBuf,

    /// Optional strategies (all are optional in MVP).
    #[cfg(feature = "local-build")]
    pub build: Option<&'static dyn BuildRecipe>,
    #[cfg(feature = "http")]
    pub releases: Option<&'static dyn ReleaseIndex>, // post-MVP if you want
    pub version_probe: Option<&'static dyn VersionProbe>,
}

pub struct Registry {
    tools: std::collections::HashMap<ServiceId, ToolSpec>,
}

impl Default for Registry {
    fn default() -> Self {
        let mut tools: std::collections::HashMap<ServiceId, ToolSpec> =
            std::collections::HashMap::new();
        tools.insert(ZCASHD, spec_zcashd());
        Self { tools: tools }
    }
}

impl Registry {
    pub fn with_builtins() -> Self {
        Registry::default()
    }

    pub fn empty() -> Self {
        Self {
            tools: std::collections::HashMap::new(),
        }
    }

    pub fn register(mut self, spec: ToolSpec) -> Self {
        todo!()
    }
    pub fn get(&self, id: &ServiceId) -> Option<&ToolSpec> {
        todo!()
    }
}
