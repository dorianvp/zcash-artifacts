#[cfg(feature = "local-build")]
#[derive(Debug, Clone, Copy)]
pub enum GitPolicy {
    /// Refuse to build if there are uncommitted changes.
    RequireClean,
    /// Allow dirty builds; cache key includes a worktree content hash.
    AllowDirty { hash_untracked: bool },
}
