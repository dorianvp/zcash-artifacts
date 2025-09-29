use std::{path::PathBuf, str::FromStr};

use zcash_artifacts::{
    ArtifactResolver, ArtifactSource, BuildConfig, ResolvedArtifact, ResolverConfig, git::GitPolicy,
};

fn main() {
    println!("Hello friend");

    let cfg = ResolverConfig {
        cache_root: tempfile::tempdir().unwrap().keep(),
        build_config: BuildConfig {
            allow_build: true,
            default_jobs: Some(2),
            default_policy: GitPolicy::RequireClean,
            default_expected_output: PathBuf::from("src/zcashd"),
        },
    };
    let provider = ArtifactResolver::new(cfg);

    let src = ArtifactSource::Build {
        repo: PathBuf::from_str("<path>").unwrap(),
        refspec: None,
        policy: GitPolicy::RequireClean,
        expected_output: None,
        service: todo!(),
    };

    let zcashd_path = match provider.resolve(&src).unwrap() {
        ResolvedArtifact::Executable { path } => path,
        _ => panic!(),
    };

    dbg!(&zcashd_path);
    assert!(zcashd_path.exists());
}
