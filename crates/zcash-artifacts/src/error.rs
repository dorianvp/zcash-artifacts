//! Error types for `zcash-artifacts`.

#![forbid(unsafe_code)]

use std::path::PathBuf;
use thiserror::Error;

use crate::ServiceKind;

pub type Result<T> = std::result::Result<T, ArtifactError>;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum InputError {
    #[error("path does not exist or is not a file: {path}")]
    NotFound { path: PathBuf },

    #[error("path is not executable: {path}")]
    NotExecutable { path: PathBuf },

    #[error("invalid source for {service:?}: {reason}")]
    InvalidSource {
        service: ServiceKind,
        reason: String,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LocateError {
    #[error("no asset for {service:?} {version} on {platform}")]
    NoAsset {
        service: ServiceKind,
        version: String,
        platform: String,
    },

    #[error("failed to resolve release index for {service:?} {version}: {why}")]
    ReleaseIndex {
        service: ServiceKind,
        version: String,
        why: String,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FetchError {
    #[cfg(feature = "http")]
    #[error("http error fetching {url}")]
    Http {
        url: String,
        #[source]
        source: reqwest::Error,
    },

    #[cfg(feature = "http")]
    #[error("timeout fetching {url}")]
    Timeout { url: String },

    #[cfg(feature = "http")]
    #[error("network error fetching {url}")]
    Network {
        url: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[cfg(not(feature = "http"))]
    #[error("http support disabled; cannot fetch {url}")]
    Disabled { url: String },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("checksum mismatch for {url}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        url: String,
        expected: String,
        actual: String,
    },

    #[error("missing checksum for {url}")]
    MissingChecksum { url: String },

    #[error("signature verification failed for {what}")]
    SignatureInvalid {
        what: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum UnpackError {
    #[error("unsupported archive format: {archive}")]
    UnsupportedFormat { archive: String },

    #[error("failed to extract {entry} from {archive}")]
    Entry {
        archive: String,
        entry: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("archive tool error for {archive}")]
    Tool {
        archive: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum OciError {
    #[cfg(feature = "oci")]
    #[error("invalid OCI reference: {reference}")]
    InvalidReference { reference: String },

    #[cfg(feature = "oci")]
    #[error("failed to pull image {reference}")]
    Pull {
        reference: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[cfg(feature = "oci")]
    #[error("unauthorized for image {reference}")]
    Unauthorized { reference: String },

    #[cfg(not(feature = "oci"))]
    #[error("oci support disabled; cannot use {reference}")]
    Disabled { reference: String },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FsError {
    #[error("io at {context}")]
    Io {
        context: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to set executable bit: {path}")]
    Chmod {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("unsupported platform for {service:?}: {platform}")]
    Unsupported {
        service: ServiceKind,
        platform: String,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ArtifactError {
    #[error(transparent)]
    Input(#[from] InputError),
    #[error(transparent)]
    Locate(#[from] LocateError),
    #[error(transparent)]
    Fetch(#[from] FetchError),
    #[error(transparent)]
    Verify(#[from] VerifyError),
    #[error(transparent)]
    Unpack(#[from] UnpackError),
    #[error(transparent)]
    Oci(#[from] OciError),
    #[error(transparent)]
    Fs(#[from] FsError),
    #[error(transparent)]
    Platform(#[from] PlatformError),
}

#[cfg(feature = "local-build")]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("missing build tool(s): {missing}")]
    PreflightMissingTools { missing: String },

    #[error("build disabled at runtime; set ZCASH_ARTIFACTS_ALLOW_BUILD=1")]
    DisabledRuntime,

    #[error("build feature not enabled at compile time")]
    DisabledFeature,

    #[error("build script failed with exit code {exit_code}; see log at {log_path}")]
    ScriptFailed {
        exit_code: i32,
        log_path: std::path::PathBuf,
    },

    #[error("unknown build output; expected binary at {expected}")]
    MissingOutput { expected: std::path::PathBuf },

    #[error("worktree is dirty; cannot build")]
    DirtyWorktree { repo: std::path::PathBuf },
}
