# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3](https://github.com/francisdb/pinmame-nvram/compare/v0.3.2...v0.3.3) - 2024-12-16

### Added

- raw dip switch access (#27)
- null terminated strings (#23)

### Other

- st_161h (#26)

## [0.3.2](https://github.com/francisdb/pinmame-nvram/compare/v0.3.1...v0.3.2) - 2024-12-13

### Added

- global _nibble (#18)

### Fixed

- apply int scale (#21)

### Other

- improve centaur dip switch test
- Bally MPU AS-2518-35 (#22)
- tweak out of range message

## [0.3.1](https://github.com/francisdb/pinmame-nvram/compare/v0.3.0...v0.3.1) - 2024-12-12

### Added

- value range validation (#16)

### Other

- dependabot gitsubmodule pr's

## [0.3.0](https://github.com/francisdb/pinmame-nvram/compare/v0.2.2...v0.3.0) - 2024-12-11

### Added

- dip-switches and checksum8 validation (#13)

## [0.2.2](https://github.com/francisdb/pinmame-nvram/compare/v0.2.1...v0.2.2) - 2024-12-08

### Added

- better error messages (#12)

### Other

- some more afm nvrams
- update to latest maps (#11)
- add extra fields for wip maps

## [0.2.1](https://github.com/francisdb/pinmame-nvram/compare/v0.2.0...v0.2.1) - 2024-12-06

### Fixed

- score resolve endian

## [0.2.0](https://github.com/francisdb/pinmame-nvram/compare/v0.1.1...v0.2.0) - 2024-12-05

### Added

- resolve checksum16 ([#7](https://github.com/francisdb/pinmame-nvram/pull/7))
- map resolve ([#5](https://github.com/francisdb/pinmame-nvram/pull/5))

### Other

- williams system 11B

## [0.1.1](https://github.com/francisdb/pinmame-nvram/compare/v0.1.0...v0.1.1) - 2024-12-04

### Added

- compressed nvram maps ([#4](https://github.com/francisdb/pinmame-nvram/pull/4))

### Other

- strapids
- hod
- release v0.1.0 ([#1](https://github.com/francisdb/pinmame-nvram/pull/1))

## [0.1.0](https://github.com/francisdb/pinmame-nvram/releases/tag/v0.1.0) - 2024-12-04

### Added

- game state
- last game retrieval
- mode champions
- implementing nibble and scale
- support for _roms
- embed nvram maps
- reading / writing highscores

### Fixed

- use _char_map

### Other

- add missing cargo attributes
- enable submodules for release-plz
- enable release-plz
- funhouse
- ignore jokerz for now
- ignore gmine for now
- zaccaria
- more zaccaria nvrams
- more roms
- fh_l9
- add williams system 11B nvrams
- btmn_106
- btmn_106
- IntegerOrFloat -> Number
- enable st_162h
- tom_13
- ww_l5
- update script
- submodule tweak
- williams.system7
- ww_lh6
- upload more nvram files, tests and maps should follow
- enable dependabot
- add more nvram files and tests
- test more nvram files
- readme usage
- adding more test nvram files
- https for submodule
- add maps as submodule
- wip
- wip
- clippy fixes
- clippy
- rust.yml
- Initial commit, wip
- Initial commit
