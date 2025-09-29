#[cfg(feature = "local-build")]
use crate::BuildRecipe;
use crate::registry::{ToolSpec, ZCASHD};

#[cfg(feature = "local-build")]
struct ZcashdBuild;

#[cfg(feature = "local-build")]
impl BuildRecipe for ZcashdBuild {
    fn build(
        &self,
        repo: &std::path::Path,
        jobs: usize,
        log: &std::path::Path,
    ) -> crate::error::Result<std::path::PathBuf> {
        // ./zcutil/build.sh -j{jobs}, logs to `log`,
        // returns PathBuf::from("src/zcashd") on success.
        unimplemented!()
    }
}

pub fn spec_zcashd() -> ToolSpec {
    fn names(platform: &str) -> &'static [&'static str] {
        match platform {
            "linux-x86_64" | "linux-aarch64" | "macos-x86_64" | "macos-arm64" => &["zcashd"],
            _ => &["zcashd"],
        }
    }

    #[cfg(feature = "local-build")]
    static ZCASHD_BUILD: ZcashdBuild = ZcashdBuild;

    ToolSpec {
        id: ZCASHD,
        binary_names: names,
        default_expected_output: "src/zcashd".into(),
        #[cfg(feature = "local-build")]
        build: Some(&ZCASHD_BUILD),     // runs ./zcutil/build.sh -jN
        #[cfg(not(feature = "local-build"))]
        // when the feature is off, the field doesn't exist
        version_probe: Some(&DEFAULT_PROBE),
        #[cfg(feature = "http")]
        releases: todo!(),
        version_probe: todo!(),    // optional; see below
    }
}
