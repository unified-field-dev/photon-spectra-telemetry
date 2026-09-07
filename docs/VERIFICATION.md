# photon-spectra-telemetry verification

Re-run after code or doc changes. Photon `OpsLog` over Spectra — covered by
unit + integration tests below.

## Environment

Match [`.github/workflows/ci.yml`](../.github/workflows/ci.yml):

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-photon-spectra-telemetry
export RUSTFLAGS="-D warnings"
```

Toolchain: stable (same as CI).

## PR CI parity

Required PR job `quality`:

| CI step | Local command |
|--------|----------------|
| Formatting | `cargo fmt --all -- --check` |
| Clippy | `cargo clippy --all-targets --all-features -- -D warnings` |
| Test | `cargo test --all-features` |
| cargo doc | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` |

## Unit + integration (CI)

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

### TEST_MAP

| Behavior | Level | Happy | Sad | Notes |
|----------|-------|-------|-----|-------|
| `counter_delta` | unit | integer f64 → i64 | fractional truncates toward zero | `install::tests` |
| `install_ops_log_from_env` | integ | `off`/`0`/`false`/`none`, `console`, `spectra`, default | — | process-wide; under `ENV_LOCK` |
| `SpectraOpsLog` OpsLog methods | integ | counter (incl. fractional), gauge, event | unknown names / empty labels / empty payload accepted | forwards via `try_*` gate |
| Typed recorders / loggers | integ | publish/drain/backlog/errors/handler/storage-port wall clocks/ops log | empty labels accepted | no Spectra sink required |
| Topic constants | integ | all `*_TOPIC` non-empty `spectra.{metric,event}.*` + `photon_` / `continuum_` | — | Photon wire names |

## Notes

- Integration tests serialize env mutations with `ENV_LOCK`.
- Under Spectra `try_*` gates, OpsLog and typed helpers may no-op when Spectra
  is unconfigured; assertions focus on contracts and non-panic forward paths
  rather than captured sink rows.
- Sad-path tests are named with `_sad` / `happy_and_sad` so audits detect them;
  they assert concrete truncation and acceptance defaults, beyond smoke-only checks.
