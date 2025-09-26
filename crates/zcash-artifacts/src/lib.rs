mod error;
pub use error::{ArtifactError, Result};

use std::path::PathBuf;

use url::Url;

#[derive(Debug, Clone)]
pub enum ServiceKind {
    Node(NodeKind),
    Indexer(IndexerKind),
}

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
pub enum ArtifactSource {
    LocalPath(PathBuf),
    Release { kind: ServiceKind, version: String },
    Url { url: Url, checksum: String },
    // OciImage {
    //     reference: String,
    //     digest: Option<String>,
    // },
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
