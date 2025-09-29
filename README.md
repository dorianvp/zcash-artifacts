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

See [cache.rs](./crates/zcash-artifacts/src/cache.rs) for more details.