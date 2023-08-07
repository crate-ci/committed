# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [1.0.20] - 2023-08-07

### Compatibility

- Updated MSRV to 1.70.0

## [1.0.19] - 2023-08-07

### Fixes

- Report correct mood of "package" and "added"

## [1.0.18] - 2023-05-22

### Fixes

- *(action)* Don't require `sudo`

## [1.0.17] - 2023-04-19

### Performance

- *(pre-commit)* Build musl wheels

## [1.0.16] - 2023-04-19

### Fixes

- *(pre-commit)* Ensure there is a default target to install for `committed-src`

## [1.0.15] - 2023-04-13

### Internal

- Update dependency

## [1.0.14] - 2023-03-16

### Internal

- Update dependency

## [1.0.13] - 2023-03-14

## [1.0.12] - 2023-03-14

## [1.0.11] - 2023-03-14

### Fixes

- Improve `CLICOLOR` support
- Auto-enable colors for CI
- *(pre-commit)* Use pre-built wheels for more reliability

### Performance

- *(action)* Avoid docker builds through composite actions

## [1.0.10] - 2023-03-08

### Internal

- Updated dependencies

## [1.0.9] - 2023-02-28

## [1.0.8] - 2023-02-24

### Features

- *(action)* Make behavior more configurable

## [1.0.7] - 2023-01-19

## [1.0.6] - 2022-12-01

## [1.0.5] - 2022-09-28

### Fixes

- Polished help output

## [1.0.4] - 2022-08-16

### Fixes

- *(action)* Fix git log view

## [1.0.3] - 2022-04-12

### Fixes

- Hardened github action for errors

## [1.0.2] - 2022-03-31

### Fixes

- Have `--commit-file`  compatible with `commit.verbose`

## [1.0.1] - 2022-01-18

### Fixes

Conventional Parsing
- When a body and footer have extra newlines between them, don't put them at the end of the body
- Handle windows newlines (was missing footers with them)

## [1.0.0] - 2022-01-17

Decided it was well past time to bless this as a 1.0.

### Fixes

- Gracefully handle caseless letters.

## [0.2.9] - 2022-01-11

## [0.2.8] - 2021-12-27

#### Fixes

- Clean up Github Action's output

## [0.2.7] - 2021-12-14

#### Fixes

- Cleaned up output by showing short ids for commits

## [0.2.6] - 2021-12-14

#### Fixes

- Clarified error messages to help guide people to conventional style
- Enabled color in github action so its easier to see the message

## [0.2.5] - 2021-10-19

#### Fixes

- Loosened requirements for parsing Conventional Commits

## [0.2.4] - 2021-10-08

#### Features

- Color output support

## [0.2.3] - 2021-09-07

## [0.2.2] - 2021-09-07

#### Features

- Experimental pre-commit hook support

## [0.2.1] - 2021-09-07

#### Bug Fixes

- Show empty-commit errors as messages for draft commits

## [0.2.0] - 2021-09-07

#### Bug Fixes

- Show empty-commit errors as messages

## [0.1.26] - 2021-09-06

## [0.1.25] - 2021-07-02

#### Bug Fixes

- Be less noisy in Github Action

## [0.1.24] - 2021-07-02

#### Features

- Inaugural Github Action

## [0.1.23] - 2021-07-02

#### Bug Fixes

- Fixing tarball releases

## [0.1.22] - 2021-07-02

#### Bug Fixes

- Flatten released tarball

## [0.1.21] - 2021-07-02

Ignore, just testing Github Actions

## [0.1.20] - 2021-07-02

Ignore, just testing Github Actions

## [0.1.19] - 2021-07-02

Ignore, just testing Github Actions

## [0.1.18] - 2021-07-02

Ignore, just testing Github Actions

## [0.1.17] - 2021-07-02

Ignore, just testing Github Actions

## [0.1.16] - 2021-07-02

Ignore, just testing Github Actions

## [0.1.15] - 2021-07-01

## [0.1.14] - 2021-07-01

## [0.1.13] - 2021-02-04

#### Features

* New `--dump-config` flag to make behavior clearer
* Soften the Subject's line limit
* Cover more WIP prefixes

#### Bug Fixes

* Include defaults in documentation (thanks jmaguire!)
* Fix soft line limits to allow a word to go past.  This was to match the soft
  limit for Subject.  Before, the assumption was that you could just break the
  line at the previous space but you can't with the subject.
* Loosen the punctuation check

## [0.1.12] - 2020-05-07

#### Features

* Support ignoring bot commits via new config flag "ignore_author_re"

#### Bug Fixes

* Fixed case of config values ("Conventional", instead of "conventional" is deprecated and will eventually be removed)
* Update to latest conventional spec.


## 0.1.11 (2019-09-10)

#### Features

* **checks:**
  *  Limit allowed types ([965985f3](https://github.com/crate-ci/committed/commit/965985f3dcd616a36f65e981ee3973b6ca1524fc), closes [#8](https://github.com/crate-ci/committed/issues/8))
  *  Distinguish hard / soft line lengths ([1dcc9681](https://github.com/crate-ci/committed/commit/1dcc96813e0301928431cc164c464262117464bb), closes [#29](https://github.com/crate-ci/committed/issues/29))


## 0.1.10 (2019-08-29)

#### Bug Fixes

* **checks:**  Fail on style errors ([3d7c8e51](https://github.com/crate-ci/committed/commit/3d7c8e5119e16f9af71f9b643b131d1f644398a4))


## 0.1.9 (2019-08-19)

#### Bug Fixes

* **docs:**  Link to correct pipeline ([b5c1512f](https://github.com/crate-ci/committed/commit/b5c1512f0739c980559eb2eceed06c8b511ab99a))
* **files:**  Avoid short-circuiting ([f5c190ff](https://github.com/crate-ci/committed/commit/f5c190ff0fdd7adddc882f4b04a41cb6334cc26f))


## 0.1.8 (2019-08-18)

#### Bug Fixes

* **checks:**
  *  Reverse polarity of the neutron flow ([10f37145](https://github.com/crate-ci/committed/commit/10f3714578da516829d0f5663b8af71dfcbe1caa))
  *  Avoid short-circuiting ([40ed26a4](https://github.com/crate-ci/committed/commit/40ed26a453893e1b4555f2dceb7f56fcdb774762))


## 0.1.1 (2019-08-06)

#### Features

*   Create pre-built binaries ([86264b85](https://github.com/crate-ci/committed/commit/86264b8557fea00435aa92f1345f61dcf923650b), closes [#12](https://github.com/crate-ci/committed/issues/12))


<!-- next-url -->
[Unreleased]: https://github.com/crate-ci/committed/compare/v1.0.20...HEAD
[1.0.20]: https://github.com/crate-ci/committed/compare/v1.0.19...v1.0.20
[1.0.19]: https://github.com/crate-ci/committed/compare/v1.0.18...v1.0.19
[1.0.18]: https://github.com/crate-ci/committed/compare/v1.0.17...v1.0.18
[1.0.17]: https://github.com/crate-ci/committed/compare/v1.0.16...v1.0.17
[1.0.16]: https://github.com/crate-ci/committed/compare/v1.0.15...v1.0.16
[1.0.15]: https://github.com/crate-ci/committed/compare/v1.0.14...v1.0.15
[1.0.14]: https://github.com/crate-ci/committed/compare/v1.0.13...v1.0.14
[1.0.13]: https://github.com/crate-ci/committed/compare/v1.0.12...v1.0.13
[1.0.12]: https://github.com/crate-ci/committed/compare/committed-v1.0.11...v1.0.12
[1.0.11]: https://github.com/crate-ci/committed/compare/v1.0.10...committed-v1.0.11
[1.0.10]: https://github.com/crate-ci/committed/compare/v1.0.9...v1.0.10
[1.0.9]: https://github.com/crate-ci/committed/compare/v1.0.8...v1.0.9
[1.0.8]: https://github.com/crate-ci/committed/compare/v1.0.7...v1.0.8
[1.0.7]: https://github.com/crate-ci/committed/compare/v1.0.6...v1.0.7
[1.0.6]: https://github.com/crate-ci/committed/compare/v1.0.5...v1.0.6
[1.0.5]: https://github.com/crate-ci/committed/compare/v1.0.4...v1.0.5
[1.0.4]: https://github.com/crate-ci/committed/compare/v1.0.3...v1.0.4
[1.0.3]: https://github.com/crate-ci/committed/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/crate-ci/committed/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/crate-ci/committed/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/crate-ci/committed/compare/v0.2.9...v1.0.0
[0.2.9]: https://github.com/crate-ci/committed/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/crate-ci/committed/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/crate-ci/committed/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/crate-ci/committed/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/crate-ci/committed/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/crate-ci/committed/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/crate-ci/committed/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/crate-ci/committed/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/crate-ci/committed/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/crate-ci/committed/compare/v0.1.26...v0.2.0
[0.1.26]: https://github.com/crate-ci/committed/compare/v0.1.25...v0.1.26
[0.1.25]: https://github.com/crate-ci/committed/compare/v0.1.24...v0.1.25
[0.1.24]: https://github.com/crate-ci/committed/compare/v0.1.23...v0.1.24
[0.1.23]: https://github.com/crate-ci/committed/compare/v0.1.22...v0.1.23
[0.1.22]: https://github.com/crate-ci/committed/compare/v0.1.21...v0.1.22
[0.1.21]: https://github.com/crate-ci/committed/compare/v0.1.20...v0.1.21
[0.1.20]: https://github.com/crate-ci/committed/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/crate-ci/committed/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/crate-ci/committed/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/crate-ci/committed/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/crate-ci/committed/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/crate-ci/committed/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/crate-ci/committed/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/crate-ci/committed/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/crate-ci/committed/compare/v0.1.11...v0.1.12
