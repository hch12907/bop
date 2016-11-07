<img src="https://dl.dropboxusercontent.com/u/" alt="bop editor infrastructure" width="255px" />

<p align="center">
  [![Crates][crates-img]][crates-url]
  [![Builds][travis-img]][travis-url]
  [![Code coverage][coveralls-img]][coveralls-url]
</p>

  Bop is an __unopinoinated__ editor infrastructure to make the process of
  development less hassle, and more enjoyable.

## Philosophy
  Bop loads other packages which are written with predefined interfaces and 
  invoked with priority.

  - Bop is modular, comes with intuitive API
  - Cleaner project structure
  - Highly configurable

## Installation
  Bop requires __rust v1.6.0__ or higher. We can install it via __cargo__
  package manager. Or get the precompiled binaries here. [Stable release][release-url].

  ```
  $ cargo install bop
  ```

## Getting started
 - [Introduction][intro-url]

## Bundles
  __Bundles__ are mean to "augments bop", extending the functionality of Bop.
  Here is the aim of bundles:

  - Bundles should expose services
  - Bundles should communicate with other packages via available services
  - Bundles should expose objects, whichs can be configured in config files

## Community

 - [API](docs/api/index.md) documentation
 - [Guide](docs/guide.md)
 - [中文文档](https://)
 - __[#bop]__ on freenode

## License
  
  [![BSD 2-Clause License][license-img]][license-url]

[travis-img]: https://img.shields.io/travis/zypeh/bop.master.svg?style=flat-square
[travis-url]: https://travis.ci.org/zypeh/bop
[crates-img]: https://meritbadge.herokuapp.com/bop?style=flat-square
[crates-url]: https://crates.io/crates/bop
[coveralls-img]: https://img.shields.io/coveralls/zypeh/bop?style=flat-square
[coveralls-url]: https://coveralls.io/zypeh/bop
[licence-img]: https://img.shields.io/badge/license-BSD%202--Clause-blue.svg?style=flat-square
[license-url]: https://opensource.org/licenses/BSD-2-Clause