# photon-spectra-telemetry

[![CI](https://github.com/unified-field-dev/photon-spectra-telemetry/actions/workflows/ci.yml/badge.svg)](https://github.com/unified-field-dev/photon-spectra-telemetry/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[GitHub](https://github.com/unified-field-dev/photon-spectra-telemetry) · `cargo doc -p photon-spectra-telemetry --open`

Spectra-backed telemetry for [Photon](https://github.com/unified-field-dev/photon): DSL schemas, typed topic helpers for Photon self-metrics, and OpsLog install for host bootstrap.

```toml
photon-spectra-telemetry = { git = "https://github.com/unified-field-dev/photon-spectra-telemetry" }
```

```rust
use photon_spectra_telemetry::install_ops_log_from_env;

// Reads `PHOTON_TELEMETRY` (off / console / Spectra).
// Install the Spectra sink first, then OpsLog install, then build Photon.
install_ops_log_from_env();
```

## About

- Spectra DSL schemas under `schemas/` (inventory-registered when linked)
- Generated topic helpers for Photon self-metrics (`*Recorder` / `*Logger`, payloads)
- `install_ops_log_from_env` / `SpectraOpsLog` for host bootstrap

## Examples

Runnable smoke: [examples/README.md](examples/README.md).
## Verify

```bash
export CARGO_BUILD_JOBS=1
cargo test
```

## License

MIT. See [LICENSE](LICENSE), [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
