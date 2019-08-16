# committed

> **Nitpicking commit history since `beabf39`**

[![Travis Status](https://travis-ci.org/crate-ci/committed.svg?branch=master)](https://travis-ci.org/crate-ci/committed)
[![Appveyor Status](https://ci.appveyor.com/api/projects/status/nmndw68dl9f8s5k2/branch/master?svg=true)](https://ci.appveyor.com/project/epage/committed/branch/master)
[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/committed?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=5&branchName=master)
[![codecov](https://codecov.io/gh/crate-ci/committed/branch/master/graph/badge.svg)](https://codecov.io/gh/crate-ci/committed)
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/committed.svg)
[![Crates Status](https://img.shields.io/crates/v/committed.svg)](https://crates.io/crates/committed)

## Install

For now, `rust` is required for install:

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
committed HEAD~..HEAD^^2 --no-merge-commit
```
See also our [`.travis.yml`](.travis.yml) and [`appveyor.yml`](appveyor.yml).

## [Reference](docs/reference.md)

## [Contribute](CONTRIBUTING.md)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[Crates.io]: https://crates.io/crates/committed
[Documentation]: https://docs.rs/committed
