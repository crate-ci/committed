# committed

> **Nitpicking commit history since `beabf39`**

[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/committed?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=7&branchName=master)
[![codecov](https://codecov.io/gh/crate-ci/committed/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/committed)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/committed.svg)
[![Crates Status](https://img.shields.io/crates/v/committed.svg)](https://crates.io/crates/committed)

## Install

[Download](https://github.com/crate-ci/committed/releases) a pre-built binary
(installable via [gh-install](https://github.com/crate-ci/gh-install).

Or use rust to install:
```bash
cargo install committed
```

## Basic Usage

Verify your latest commit
```bash
committed HEAD
```

Verify your branch
```bash
committed master..HEAD --no-merge-commit
```

Have your CI verify your PR (assuming it does a no-ff merge into your `master`)
```bash
committed HEAD~..HEAD^2 --no-merge-commit
```
See [`imperative` for an Azure Pipeline
example](https://github.com/crate-ci/imperative/blob/master/azure-pipelines.yml)
or look at `committed`s own [`azure-pipelines.yml`](azure-pipelnes.yml),
[`.travis.yml`](.travis.yml), or [`appveyor.yml`](appveyor.yml).

## [Reference](docs/reference.md)

## [Contribute](CONTRIBUTING.md)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[Crates.io]: https://crates.io/crates/committed
[Documentation]: https://docs.rs/committed
