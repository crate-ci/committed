# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/v0.1.20...HEAD
[0.1.20]: https://github.com/assert-rs/predicates-rs/compare/v0.1.19...v0.1.20
[0.1.19]: https://github.com/assert-rs/predicates-rs/compare/v0.1.18...v0.1.19
[0.1.18]: https://github.com/assert-rs/predicates-rs/compare/v0.1.17...v0.1.18
[0.1.17]: https://github.com/assert-rs/predicates-rs/compare/v0.1.16...v0.1.17
[0.1.16]: https://github.com/assert-rs/predicates-rs/compare/v0.1.15...v0.1.16
[0.1.15]: https://github.com/assert-rs/predicates-rs/compare/v0.1.14...v0.1.15
[0.1.14]: https://github.com/assert-rs/predicates-rs/compare/v0.1.13...v0.1.14
[0.1.13]: https://github.com/assert-rs/predicates-rs/compare/v0.1.12...v0.1.13
[0.1.12]: https://github.com/assert-rs/predicates-rs/compare/v0.1.11...v0.1.12
