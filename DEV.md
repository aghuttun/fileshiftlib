# fileshiftlib — Developer Guide

- [Overview](#overview)
- [Requirements](#requirements)
- [Version Access](#version-access)
- [Project Structure](#project-structure)
- [Building](#building)
- [API Reference](#api-reference)
- [Tests](#tests)
- [CI/CD](#cicd)
- [Publishing](#publishing)

## Overview

fileshiftlib is implemented in Rust using [PyO3](https://pyo3.rs/) for Python bindings and
the [ssh2](https://crates.io/crates/ssh2) crate for SFTP. [Maturin](https://www.maturin.rs/)
is used as the build and packaging tool.

## Requirements

- Python 3.10 or higher
- Rust 1.70 or higher — install from <https://rustup.rs/>
- Maturin 1.7 or higher

```bash
pip install maturin
```

## Version Access

The recommended market standard is using installed package metadata:

```python
from importlib.metadata import version

print(version("fileshiftlib"))
```

The package also exposes `fileshiftlib.__version__` as a convenience alias.

## Project Structure

```
fileshiftlib/
├── src/
│   ├── lib.rs              # PyO3 module entry point
│   ├── models.rs           # Rust SFTP client implementation
│   └── python_bindings.rs  # Python class wrappers
├── tests/
│   ├── test_smoke.py       # Public API smoke tests
│   └── test_live_sftp.py   # Optional integration tests (require a live server)
├── old_python_version/     # Archived pure-Python implementation (paramiko-based)
├── Cargo.toml              # Rust crate configuration
├── pyproject.toml          # Python packaging configuration (maturin backend)
├── .github/
│   └── workflows/
│       └── CI.yml          # GitHub Actions CI/CD pipeline
├── README.md               # User-facing documentation
└── DEV.md                  # This file
```

## Building

### Development build (fast iteration)

```bash
maturin develop --features python
```

### Release build (optimised)

```bash
maturin develop --release --features python
```

### Build distributable wheel

```bash
maturin build --release --features python --out dist
```

### Build source distribution

```bash
maturin sdist --out sdisthouse
```

## API Reference

### `SFTP(host, username, password, port=22, logger=None)`

Initialise the client and establish the connection.

| Parameter  | Type  | Default | Description                              |
|------------|-------|---------|------------------------------------------|
| host       | str   | —       | Hostname or IP of the SFTP server        |
| username   | str   | —       | Authentication username                  |
| password   | str   | —       | Authentication password                  |
| port       | int   | 22      | Port number                              |
| logger     | any   | None    | Ignored (kept for API compatibility)     |

### Methods

| Method                                      | Returns    | Description                          |
|---------------------------------------------|------------|--------------------------------------|
| `is_connected()`                            | `bool`     | Check whether the session is active  |
| `reconnect()`                               | `None`     | Close and re-establish the session   |
| `list_dir(path=".")`                        | `list[str]`| List filenames in a remote directory |
| `change_dir(path=".")`                      | `None`     | Change the remote working directory  |
| `upload_file(local_path, remote_path)`      | `None`     | Upload a local file to the server    |
| `download_file(remote_path, local_path)`    | `None`     | Download a remote file locally       |
| `delete_file(filename)`                     | `None`     | Delete a file on the server          |

### Errors

All methods raise a Python `Exception` if the underlying SSH/SFTP operation fails.
The message is propagated directly from the Rust `SftpError` type.

## Tests

### Smoke tests (no server required)

Run the public API surface tests:

```bash
python -m pytest tests/test_smoke.py -q
```

### Integration tests (live SFTP server required)

Set the environment variables before running:

```bash
export FILESHIFTLIB_HOST=sftp.example.com
export FILESHIFTLIB_USERNAME=user
export FILESHIFTLIB_PASSWORD=password

python -m pytest tests/test_live_sftp.py -q
```

Tests are skipped automatically if the environment variables are not set.

### Running all tests

```bash
python -m pytest tests/ -q
```

### Rust tests

```bash
cargo test
```

## CI/CD

The GitHub Actions pipeline in [.github/workflows/CI.yml](.github/workflows/CI.yml) runs on
every push to `main`, on every pull request, and on version tags (`v*`).

| Job           | Trigger                  | Platforms                        |
|---------------|--------------------------|----------------------------------|
| test          | push / PR / tag          | ubuntu, windows, macos           |
| build-wheel   | after test passes        | ubuntu, windows, macos           |
| build-sdist   | after test passes        | ubuntu                           |
| publish-pypi  | version tag (`v*`) only  | ubuntu                           |

## Publishing

Publishing is automated via CI when a version tag is pushed:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The pipeline uses the `PYPI_API_TOKEN` secret stored in the repository settings.

To publish manually:

```bash
maturin publish --username __token__ --password "$PYPI_API_TOKEN"
```
