mod error;
pub mod git;
pub mod registry;
mod zcashd;

pub use error::{ArtifactError, Result};

use std::path::{Path, PathBuf};

use crate::registry::Registry;
#[cfg(feature = "local-build")]
use crate::{git::GitPolicy, registry::ServiceId};

#[derive(Debug, Clone)]
pub enum NodeKind {
    Zcashd,
    Zebrad,
}

#[derive(Debug, Clone)]
pub enum IndexerKind {
    Lightwalletd,
    Zainod,
}

#[derive(Debug, Clone)]
pub enum ResolvedArtifact {
    Executable { path: PathBuf },
    // OciImage { reference: String }
}

pub trait ArtifactProvider {
    fn resolve(&self, src: &ArtifactSource) -> Result<ResolvedArtifact>;
}

pub struct DefaultProvider;
impl ArtifactProvider for DefaultProvider {
    fn resolve(&self, _src: &ArtifactSource) -> Result<ResolvedArtifact> {
        todo!()
    }
}

// TODO: Account for the `archive` feature
#[derive(Debug, Clone)]
pub enum ArtifactSource {
    LocalPath(PathBuf),
    Release {
        service: ServiceId,
        version: String,
    },
    #[cfg(feature = "local-build")]
    Build {
        service: ServiceId,
        repo: PathBuf,

        /// Any of <tag>, <branch>, or <commit>. Defaults to <HEAD>.
        refspec: Option<String>,

        /// Whether to allow the local repository to be dirty.
        ///
        /// Use `dirty` when the local repository contains uncommitted changes.
        policy: GitPolicy,

        /// Defaults to `src/zcashd`
        expected_output: Option<PathBuf>,
    },
    #[cfg(feature = "http")]
    Url {
        url: Url,
        checksum: String,
    },
    #[cfg(feature = "oci")]
    OciImage {
        reference: String,
        digest: Option<String>,
    },
}

/// Configuration for zcash-artifacts
pub struct ResolverConfig {
    /// Where to store downloaded artifacts.
    ///
    /// Defaults to ~/.cache/zcash-artifacts/<artifact-kind>/<version-or-rev>/<os>-<arch>
    pub cache_root: PathBuf,

    /// The build configuration to use.
    pub build_config: BuildConfig,
}

#[cfg(feature = "local-build")]
pub struct BuildConfig {
    pub allow_build: bool,
    pub default_jobs: Option<u32>,
    /// Default worktree policy.
    pub default_policy: GitPolicy,
    /// Default expected output (“src/zcashd”).
    pub default_expected_output: PathBuf,
}

/// Minimal provider surface the consumer uses.
pub struct ArtifactResolver {
    config: ResolverConfig,
    registry: Option<Registry>,
}

impl ArtifactResolver {
    pub fn new(cfg: ResolverConfig) -> Self {
        Self {
            config: cfg,
            registry: None,
        }
    }

    pub fn with_registry(cfg: ResolverConfig, registry: Registry) -> Self {
        Self {
            config: cfg,
            registry: Some(registry),
        }
    }

    pub fn resolve(&self, src: &ArtifactSource) -> crate::error::Result<ResolvedArtifact> {
        match src {
            ArtifactSource::LocalPath(path_buf) => todo!(),
            ArtifactSource::Release { service, version } => todo!(),
            #[cfg(feature = "local-build")]
            ArtifactSource::Build {
                service,
                repo,
                refspec,
                policy,
                expected_output,
            } => self.resolve_local_build(
                service,
                repo,
                refspec.as_deref(),
                *policy,
                expected_output.as_deref(),
            ),
        }
    }

    fn resolve_local_path(&self, path: &PathBuf) -> crate::error::Result<ResolvedArtifact> {
        use crate::error::{FsError, InputError};
        use std::fs;

        let md = fs::metadata(path).map_err(|e| FsError::Io {
            context: format!("stat {}", path.display()),
            source: e,
        })?;
        if !md.is_file() {
            return Err(InputError::NotFound { path: path.clone() }.into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if md.permissions().mode() & 0o111 == 0 {
                return Err(InputError::NotExecutable { path: path.clone() }.into());
            }
        }

        Ok(ResolvedArtifact::Executable { path: path.clone() })
    }

    /// This methods does the following:
    /// - Preflights git
    /// - Identifies the tree state of the provided repository.
    /// - Generates a cache key.
    /// - If cache misses, builds.
    ///     - TODO: EXPAND
    /// - Returns the executable path.
    fn resolve_local_build(
        &self,
        service: &ServiceId,
        repo: &Path,
        refspec: Option<&str>,
        policy: GitPolicy,
        expected_output: Option<&Path>,
    ) -> crate::error::Result<ResolvedArtifact> {
        todo!()
        // use crate::error::{BuildError, FsError};

        // let refspec = refspec.unwrap_or("HEAD");
        // let expected_output = expected_output.unwrap_or_else(|| Path::new("src/zcashd"));

        // if !self.cfg.build_config.allow_build {
        //     return Err(BuildError::DisabledFeature.into());
        // }

        // let commit = git_resolve_commit(repo, refspec)?; // -> String (full SHA)
        // let dirty = git_is_dirty(repo)?; // -> bool
        // let (allow_dirty, hash_untracked) = match policy {
        //     GitPolicy::RequireClean => (false, false),
        //     GitPolicy::AllowDirty { hash_untracked } => (true, hash_untracked),
        // };
        // if dirty && !allow_dirty {
        //     return Err(BuildError::DirtyWorktree {
        //         repo: repo.to_path_buf(),
        //     }
        //     .into());
        // }
        // let worktree_hash = if dirty && allow_dirty {
        //     Some(hash_worktree(repo, hash_untracked)?) // -> String
        // } else {
        //     None
        // };

        // let host = detect_host_triple(self.cfg.platform_override.as_deref()); // "linux-x86_64" etc.
        // let key = build_key(&commit, worktree_hash.as_deref(), &host); // "zcashd|<sha>[+h]|host|v1"
        // let paths = cache_paths(&self.cfg.cache_root, "zcashd", &key); // {root, out, logs, meta}
        // std::fs::create_dir_all(&paths.out).map_err(|e| FsError::Io {
        //     context: format!("mkdir {}", paths.out.display()),
        //     source: e,
        // })?;
        // std::fs::create_dir_all(&paths.logs).map_err(|e| FsError::Io {
        //     context: format!("mkdir {}", paths.logs.display()),
        //     source: e,
        // })?;

        // let out_bin = paths.out.join("zcashd");
        // if looks_executable(&out_bin)? {
        //     return Ok(ResolvedArtifact { path: out_bin });
        // }

        // preflight_tools(&[
        //     "git",
        //     "bash",
        //     "make",
        //     "gcc",
        //     "g++",
        //     "ar",
        //     "ranlib",
        //     "perl",
        //     "autoconf",
        //     "libtool",
        //     "pkg-config",
        // ])?;

        // let _lock = acquire_lock(paths.root.join(".lock"))?; // drops on scope end

        // // Re-check cache after lock (another thread/process may have built it)
        // if looks_executable(&out_bin)? {
        //     return Ok(ResolvedArtifact { path: out_bin });
        // }

        // let log_path = paths.logs.join(format!("build-{}.log", now_ts()));
        // run_build_script(repo, jobs, &log_path)?; // wraps running ./zcutil/build.sh -j<jobs>

        // let repo_bin = repo.join(expected_output);
        // if !looks_executable(&repo_bin)? {
        //     return Err(BuildError::MissingOutput { expected: repo_bin }.into());
        // }

        // atomic_copy(&repo_bin, &out_bin)?; // temp file + rename
        // chmod_exec(&out_bin)?; // ensure +x

        // let version_str = probe_version_string(&out_bin).ok();
        // write_meta(
        //     &paths.meta,
        //     Meta {
        //         service: "zcashd".into(),
        //         source: "local-repo".into(),
        //         repo: repo.to_path_buf(),
        //         refspec: refspec.to_string(),
        //         commit,
        //         dirty,
        //         worktree_hash,
        //         jobs,
        //         host,
        //         built_at: now_ts(),
        //         builder_schema: 1,
        //         version_string: version_str,
        //     },
        // )?;

        // Ok(ResolvedArtifact { path: out_bin })
    }
}

/// How to build from a local repo.
#[cfg(feature = "local-build")]
pub trait BuildRecipe: Send + Sync + 'static {
    /// Run the build and return the repo-relative path to the binary (or absolute path).
    fn build(
        &self,
        repo: &std::path::Path,
        jobs: usize,
        log: &std::path::Path,
    ) -> crate::error::Result<std::path::PathBuf>;
}

/// How to convert (service, version, platform) to a URL+checksum (post-MVP).
#[cfg(feature = "http")]
pub trait ReleaseIndex: Send + Sync + 'static {
    fn asset_for(&self, version: &str, platform: &str) -> Option<(url::Url, String /* sha256 */)>;
}

/// How to extract a human-readable version string from a binary.
pub trait VersionProbe: Send + Sync + 'static {
    fn probe(&self, exe: &std::path::Path) -> Option<String>;
}
