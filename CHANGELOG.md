# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Refactored the crate to a no_std core plus an opt-in std client, dropping the bespoke `carddav/` module and the legacy `io-fs` / `io-stream` deps.

  Shared types `Addressbook` and `Card` are now strict least-common-denominator structs with byte-oriented contents; the optional `parser` feature gates the calcard-backed vCard helpers.

- Restructured the crate to mirror `io-email`: a domain-first, per-backend coroutine layout (`addressbook/<backend>/<op>.rs`, `card/<backend>/<op>.rs`), shared types under `<domain>/types.rs`, per-backend conversion utils under `<backend>/convert.rs`.

  Every shared operation now has its own per-backend coroutine that wraps the backend library's coroutine and implements that library's coroutine trait (`io_vdir::coroutine::VdirCoroutine`, `io_webdav::coroutine::WebdavCoroutine`). A per-backend client (`VdirClient`, `WebdavClient`) holds the inner io-vdir / io-webdav client and drives those coroutines through a `run` pump.

- Reworked `AddressbookClientStd` from a multi-backend struct into an enum holding the single active backend client (an addressbook account speaks one protocol at a time). Dispatch is a plain `match`; build one from a backend client via `From`, e.g. `AddressbookClientStd::from(VdirClient::new(inner))`.

- Replaced the inline CardDAV implementation with a dependency on the new `io-webdav` crate, and rewrote the vdir module on top of the new coroutine convention from `io-vdir` (`VdirPath`, `VdirClient`).

### Removed

- Removed `src/carddav/**` (now in io-webdav) and the legacy `io-fs` / `io-stream` / `io-http` / `quick-xml` / `base64` / `secrecy` / `memchr` direct dependencies.

## [0.0.1]

### Added

- Initial release.
