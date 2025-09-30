# zcash-artifacts

Hello friend. This crate is not yet ready for use.

## Caching

General structure:

```
<cache_root>/<service>/<key>/
  out/<service>
  logs/build-<timestamp>.log
  meta/META.json
```

### First run behavior

On the first resolve **per cache key** (service + commit [+ worktree hash] + platform), 
the resolver invokes `./zcutil/build.sh`. 
If the repo is already fully built and unchanged, the upstream build is incremental and will **no-op quickly**. 

The resolver then:
- computes the cache key,
- **copies** the existing `src/zcashd` into `<cache_root>/zcashd/<key>/out/zcashd`,
- writes `meta/META.json`,
- and returns the cached path.

Subsequent resolves for the same key are **cache hits** and do **not** run the build.

### Binary digest

We do **not** include the binary’s checksum in the cache key, because keys
must be derivable *before* any build runs. Instead, after producing
`out/zcashd`, we compute a content digest (using BLAKE3) and
store it in `meta/META.json` along with size/mtime.

Uses:
- audit/provenance in CI logs,
- optional cache verification on read (`Config::verify_cache`),
- quick detection of local corruption or manual tampering.

The cache key remains `service | commit [+ worktree_hash] | platform | v<schema>`, 
which is stable from inputs and avoids unnecessary rebuilds.

See [cache.rs](./crates/zcash-artifacts/src/cache.rs) for more details.