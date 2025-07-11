# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/francisdb/pinmame-nvram/compare/v0.4.0...v0.4.1) - 2025-07-11

### Added

- ignore non-nvram-descriptors ([#92](https://github.com/francisdb/pinmame-nvram/pull/92))

## [0.4.0](https://github.com/francisdb/pinmame-nvram/compare/v0.3.18...v0.4.0) - 2025-07-10

### Added

- v0.7 model updates ([#90](https://github.com/francisdb/pinmame-nvram/pull/90))

### Other

- submodule master -> main
- make update script nixos compatible

## [0.3.18](https://github.com/francisdb/pinmame-nvram/compare/v0.3.17...v0.3.18) - 2025-06-18

### Other

- *(deps)* bump pinmame-nvram-maps from 59629e1 to bfab5fc ([#84](https://github.com/francisdb/pinmame-nvram/pull/84))

## [0.3.17](https://github.com/francisdb/pinmame-nvram/compare/v0.3.16...v0.3.17) - 2025-06-10

### Other

- *(deps)* bump pinmame-nvram-maps from `ed00b40` to `59629e1` ([#81](https://github.com/francisdb/pinmame-nvram/pull/81))
- *(deps)* update brotli requirement from 7.0.0 to 8.0.0 ([#79](https://github.com/francisdb/pinmame-nvram/pull/79))

## [0.3.16](https://github.com/francisdb/pinmame-nvram/compare/v0.3.15...v0.3.16) - 2025-04-16

### Other

- *(deps)* bump pinmame-nvram-maps from `a856b07` to `ed00b40` ([#77](https://github.com/francisdb/pinmame-nvram/pull/77))

## [0.3.15](https://github.com/francisdb/pinmame-nvram/compare/v0.3.14...v0.3.15) - 2025-04-14

### Other

- *(deps)* bump pinmame-nvram-maps from `bae83a4` to `a856b07` ([#75](https://github.com/francisdb/pinmame-nvram/pull/75))

## [0.3.14](https://github.com/francisdb/pinmame-nvram/compare/v0.3.13...v0.3.14) - 2025-04-11

### Other

- *(deps)* bump pinmame-nvram-maps from `68dbfc4` to `bae83a4` ([#72](https://github.com/francisdb/pinmame-nvram/pull/72))
- *(deps)* bump pinmame-nvram-maps from `bf7cb4e` to `68dbfc4` ([#71](https://github.com/francisdb/pinmame-nvram/pull/71))

## [0.3.13](https://github.com/francisdb/pinmame-nvram/compare/v0.3.12...v0.3.13) - 2025-04-05

### Other

- *(deps)* bump pinmame-nvram-maps from `e940aec` to `bf7cb4e` ([#69](https://github.com/francisdb/pinmame-nvram/pull/69))

## [0.3.12](https://github.com/francisdb/pinmame-nvram/compare/v0.3.11...v0.3.12) - 2025-03-25

### Other

- update maps to e940aec ([#68](https://github.com/francisdb/pinmame-nvram/pull/68))
- *(deps)* bump pinmame-nvram-maps from `2f42c2f` to `2d7d2e8` ([#65](https://github.com/francisdb/pinmame-nvram/pull/65))

## [0.3.11](https://github.com/francisdb/pinmame-nvram/compare/v0.3.10...v0.3.11) - 2025-02-01

### Other

- bump pinmame-nvram-maps to `3f6d799` (#61)

## [0.3.10](https://github.com/francisdb/pinmame-nvram/compare/v0.3.9...v0.3.10) - 2025-01-24

### Other

- bally as-2518-35 batch 3 (#59)
- map update, victory dips (#56)
- excaliba, st_162
- add most missing nvrams (#52)

## [0.3.9](https://github.com/francisdb/pinmame-nvram/compare/v0.3.8...v0.3.9) - 2025-01-11

### Other

- _note -> _notes, optional champ initials, maps update (#51)
- list missing test nvram files
- pinball

## [0.3.8](https://github.com/francisdb/pinmame-nvram/compare/v0.3.7...v0.3.8) - 2025-01-10

### Added

- unify model descriptors (#46)

### Other

- value lookup refactor (#49)
- more wpc nvrams (#48)
- disable test map generation

## [0.3.7](https://github.com/francisdb/pinmame-nvram/compare/v0.3.6...v0.3.7) - 2025-01-03

### Added

- maps in subdirs (#44)
- use nvram index (#42)
- resolving dip switches (#40)

### Other

- excalibr

## [0.3.6](https://github.com/francisdb/pinmame-nvram/compare/v0.3.5...v0.3.6) - 2024-12-27

### Other

- maps 8e22b33 (#37)

## [0.3.5](https://github.com/francisdb/pinmame-nvram/compare/v0.3.4...v0.3.5) - 2024-12-23

### Added

- update to latest maps (#31)

### Other

- dollyptb & eballdlx (#34)
- update readme

## [0.3.4](https://github.com/francisdb/pinmame-nvram/compare/v0.3.3...v0.3.4) - 2024-12-21

### Other

- *(deps)* bump pinmame-nvram-maps from `9bcfb87` to `a2121db` (#28)

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
