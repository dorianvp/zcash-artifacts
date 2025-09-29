//! # Cache behavior & layout (for the local-build path)
//!
//! `zcash-artifacts` maintains a **content-addressed cache** for artifacts it
//! produces. The cache makes repeated resolutions fast and deterministic, avoids
//! rebuilding the same commit over and over, and provides provenance metadata
//! you can inspect or ship to CI.
//!
//! ## What gets cached?
//! For the `local-build` flow, the cache stores **the final executable** (e.g. `zcashd`),
//! **build logs**, and a small `META.json` with provenance (commit, dirty status,
//! worktree hash, platform triple, jobs, version string, timestamps).
//!
//! ## Where is it?
//! The root directory is `Config::cache_root`. By default, callers set this to a
//! per-user cache dir (e.g., `~/.cache/zcash-artifacts`). Under it, this crate
//! creates a stable hierarchy keyed by the artifact identity.
//!
//! ### Directory layout
//! ```text
//! <cache_root>/
//!   zcashd/                                  # service name
//!     <key>/                                 # unique key for this build (see below)
//!       out/                                 # final runnable binaries returned to the caller
//!         zcashd
//!       logs/                                # stdout/stderr captured during build
//!         build-2025-09-29T14-21-03.log
//!       meta/                                # provenance
//!         META.json
//! ```
//!
//! ## How is the cache key computed?
//! The **build key** uniquely identifies the bits you asked for on the current
//! host platform. It is derived as:
//!
//! - **service**: `"zcashd"` (MVP focus)
//! - **commit**: full SHA resolved from the requested `refspec` (e.g. `HEAD`)
//! - **worktree hash** *(optional)*: when the worktree is dirty and policy allows
//!   dirty builds, we compute a deterministic hash of tracked files (and, if
//!   requested, untracked files). This keeps each local edit isolated.
//! - **platform triple**: e.g. `"linux-x86_64"`, `"linux-aarch64"`, `"macos-arm64"`
//! - **builder schema version**: an internal integer you can bump if you change
//!   cache layout or the build recipe in a way that invalidates old entries.
//!
//! Conceptually:
//! ```text
//! key = "zcashd|" + <commit> + ( "+" + <worktree_hash> if dirty ) + "|" + <platform> + "|v" + <schema>
//! ```
//!
//! Using a per-key directory means concurrent runs that target *different keys*
//! never contend, and rebuilding the same commit just becomes a cache hit.
//!
//! ## Atomicity & concurrency
//! - All writes are done via **temp files + atomic rename** into place to avoid
//!   partial artifacts (especially under crashes).
//! - A **per-key lockfile** is used around the build-and-finalize phase so that
//!   concurrent processes resolving the same key won’t run the build twice. The
//!   winner builds; the others observe the finalized `out/zcashd` after the lock
//!   is released and return a cache hit.
//! - After acquiring the lock, the code re-checks for `out/zcashd` to avoid a
//!   “thundering herd” of redundant work.
//!
//! ## When do we reuse vs. rebuild?
//! - **Reuse (cache hit)** when `out/zcashd` exists for the computed key and looks
//!   executable (regular file, exec bit set). The build script is **not** run.
//! - **Rebuild** when any of these change:
//!   - `refspec` resolves to a different commit,
//!   - dirty/clean policy flips (or worktree contents changed, altering the hash),
//!   - the platform triple changes (different OS/arch),
//!   - you bump the **builder schema version**,
//!   - the cached `out/zcashd` is missing or fails the executable sanity check.
//!
//! ## Executable sanity
//! On Unix platforms we:
//! - ensure the file is a regular file,
//! - ensure the exec bit is set (and set it if we own the file),
//! - optionally sniff the file header (ELF/Mach-O) to catch obviously corrupt outputs.
//!
//! ## Provenance (`meta/META.json`)
//! A tiny JSON file written next to the artifact records the most important facts
//! for reproducibility and debugging. A typical document looks like:
//!
//! ```json
//! {
//!   "service": "zcashd",
//!   "source":  "local-repo",
//!   "repo":    "/home/dario/src/zcashd",
//!   "refspec": "HEAD",
//!   "commit":  "1a2b3c4d5e6f...",
//!   "dirty":   false,
//!   "worktree_hash": null,
//!   "jobs":    8,
//!   "host":    "linux-x86_64",
//!   "built_at": "2025-09-29T14:21:03Z",
//!   "builder_schema": 1,
//!   "version_string": "Zcashd version v5.9.0 (…)"
//! }
//! ```
//!
//! This metadata is **advisory** (the returned executable path is the source of
//! truth) but extremely useful for CI logs, bug reports, and auditing.
//!
//! ## Security posture
//! - The cache **never executes scripts from inside the cache**. Scripts are run
//!   only from your repository (e.g., `./zcutil/build.sh`) during a build.
//! - Downloaded content is **not** part of the local-build path in the MVP, but
//!   if upstream’s build pulls dependencies, that happens within the upstream
//!   script, not the cache itself.
//! - The cache layout segregates artifacts by commit & platform; copying an
//!   artifact between machines should only be done when the platform matches.
//!
//! ## Cleaning & size management
//! The MVP leaves eviction to callers (it’s just a directory). Typical patterns:
//! - remove a single key: delete `<cache_root>/zcashd/<key>/`,
//! - nuke everything: delete `<cache_root>/zcash-artifacts/zcashd/`,
//! - keep a retention policy in your tooling (e.g., “keep last N keys”). Future
//!   versions may add helpers, but manual removal is safe: keys are immutable.
//!
//! ## Example (end-to-end, local build with cache)
//! ```no_run
//! use std::path::PathBuf;
//! # use zcash_artifacts::{Provider, Config};
//! # #[cfg(feature = "local-build")]
//! # use zcash_artifacts::{ArtifactSource, BuildDefaults, GitPolicy};
//!
//! // Configure the library (no env vars).
//! let cfg = Config {
//!     cache_root: dirs::cache_dir().unwrap().join("zcash-artifacts"),
//!     platform_override: None,
//!     # #[cfg(feature = "local-build")]
//!     build_defaults: BuildDefaults {
//!         allow_build: true,
//!         default_jobs: None, // auto: CPU cores
//!         default_policy: GitPolicy::RequireClean,
//!         default_expected_output: PathBuf::from("src/zcashd"),
//!     },
//! };
//! let provider = Provider::new(cfg);
//!
//! // Ask to build from a local clone; subsequent calls hit the cache.
//! # #[cfg(feature = "local-build")]
//! let resolved = provider.resolve(&ArtifactSource::Build {
//!     repo: "/home/me/src/zcashd".into(),
//!     refspec: None,                       // HEAD
//!     policy: GitPolicy::RequireClean,     // or AllowDirty { hash_untracked: true }
//!     expected_output: None,               // default "src/zcashd"
//!     jobs: Some(8),
//! }).expect("build or cache hit");
//!
//! // Use the executable path with your launcher:
//! // zcash_services::launch_zcashd(... resolved.path ...);
//! ```
//!
//! ## More succinctly
//! - The cache key is derived from **commit**, optional **worktree hash**, **platform**, and a **schema** version.
//! - If the key exists, you get a **cache hit** (no build).
//! - Writes are **atomic**; concurrent builds of the same key are serialized.
//! - `META.json` provides the provenance you’ll want in CI and bug reports.
