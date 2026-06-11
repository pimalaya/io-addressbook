# I/O Addressbook [![Documentation](https://img.shields.io/docsrs/io-addressbook?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/io-addressbook/latest/io_addressbook) [![Matrix](https://img.shields.io/badge/chat-%23pimalaya-blue?style=flat&logo=matrix&logoColor=white)](https://matrix.to/#/#pimalaya:matrix.org) [![Mastodon](https://img.shields.io/badge/news-%40pimalaya-blue?style=flat&logo=mastodon&logoColor=white)](https://fosstodon.org/@pimalaya)

Addressbook client library, written in Rust.

This library is composed of 2 feature-gated layers:

- Low-level **I/O-free** coroutines: these `no_std`-compatible state machines wrap the underlying [io-vdir] and [io-webdav] coroutines and surface a shared least-common-denominator type on completion
- Mid-level **std client**: a standard, blocking client; an addressbook account speaks one protocol at a time, so the client is an enum holding the single active backend

## Table of contents

- [Features](#features)
- [Backend coverage](#backend-coverage)
- [Usage](#usage)
  - [Coroutines](#coroutines)
  - [Std client](#std-client)
- [Examples](#examples)
- [AI disclosure](#ai-disclosure)
- [License](#license)
- [Social](#social)
- [Sponsoring](#sponsoring)

## Features

- **Shared LCD types**: `Addressbook` and `Card` that fit both local Vdir and CardDAV, with byte-oriented card contents.
- **I/O-free** coroutines: `no_std` state machines per (backend, operation), wrapping the underlying io-* coroutine and producing a shared type on completion.
- **Std client** (`client` feature): blocking client built as an enum over the active backend; construct it from a backend client via `From`.
- **TLS** for the CardDAV backend (gated by the same `rustls-ring` / `rustls-aws` / `native-tls` features as [io-webdav]).
- Optional **vCard parsing** (`parser` feature, calcard-backed) and **serde** round-trip on every shared type (`serde` feature).

> [!TIP]
> I/O Addressbook is written in [Rust](https://www.rust-lang.org/) and uses [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to gate backend support. The default feature set is declared in [Cargo.toml](./Cargo.toml) or on [docs.rs](https://docs.rs/crate/io-addressbook/latest/features).

[io-vdir]: https://github.com/pimalaya/io-vdir
[io-webdav]: https://github.com/pimalaya/io-webdav

## Backend coverage

| Operation             | Vdir | WebDAV |
|-----------------------|:----:|:------:|
| `list_addressbooks`   |  yes |   yes  |
| `create_addressbook`  |  yes |   yes  |
| `update_addressbook`  |  yes |   yes  |
| `delete_addressbook`  |  yes |   yes  |
| `list_cards`          |  yes |   yes  |
| `get_card`            |  yes |   yes  |
| `create_card`         |  yes |   yes  |
| `update_card`         |  yes |   yes  |
| `delete_card`         |  yes |   yes  |

## Usage

I/O Addressbook can be consumed two ways, depending on how much of the I/O stack you want to own. Each mode is gated by cargo features.

Every shared-API coroutine implements the backend trait of the protocol it targets (`io_vdir::coroutine::VdirCoroutine` for Vdir, `io_webdav::coroutine::WebdavCoroutine` for CardDAV). The `resume(...)` method returns the matching `<Backend>CoroutineState<Yield, Return>` with two variants:

- `Yielded(Y)`: intermediate. `Y` is the backend's standard yield (`WantsDirCreate` / `WantsFileRead` / `WantsRename` etc. for Vdir, `WantsRead` / `WantsWrite` for CardDAV).
- `Complete(R)`: terminal. By convention `R = Result<Output, Error>` carrying the operation's final value typed against the shared `Addressbook` / `Card`.

The std client owns the resume loop for you; the I/O-free mode hands it back so you can drive the same coroutine under any blocking, async, or fuzz harness.

### Coroutines

No `client` feature required: every wrapper lives under `<domain>::<protocol>::<op>` (for example `addressbook::vdir::create::VdirAddressbookCreate`, `card::webdav::get::WebdavCardGet`) and is built straight from the shared inputs. You own the loop and the syscalls; the library only produces operations and consumes their results.

Create a Vdir addressbook against a blocking caller (the same shape works under async or in-memory replay):

```rust,no_run
use std::fs;

use io_addressbook::addressbook::vdir::create::VdirAddressbookCreate;
use io_vdir::{coroutine::*, path::VdirPath};

let root = VdirPath::new("/home/alice/contacts");

let mut coroutine = VdirAddressbookCreate::new(root, "personal", None, None).unwrap();
let mut arg: Option<VdirReply> = None;

let id = loop {
    match coroutine.resume(arg.take()) {
        VdirCoroutineState::Complete(Ok(id)) => break id,
        VdirCoroutineState::Complete(Err(err)) => panic!("{err}"),
        VdirCoroutineState::Yielded(VdirYield::WantsDirCreate(paths)) => {
            for path in paths {
                fs::create_dir_all(path.as_str()).unwrap();
            }
            arg = Some(VdirReply::DirCreate);
        }
        VdirCoroutineState::Yielded(VdirYield::WantsFileCreate(files)) => {
            for (path, bytes) in files {
                fs::write(path.as_str(), &bytes).unwrap();
            }
            arg = Some(VdirReply::FileCreate);
        }
        VdirCoroutineState::Yielded(other) => unreachable!("unexpected {other:?}"),
    }
};

println!("created addressbook {id}");
```

The CardDAV backend follows the same pattern but yields `WantsRead` / `WantsWrite(Vec<u8>)` instead; see [io-webdav] for the full TCP / TLS / discovery setup that connects the stream before the wrapper coroutine runs.

### Std client

Enable the `client` feature (pulled in by every backend feature) and at least one backend. Build a per-backend client (`VdirClient`, `WebdavClient`) around its inner io-* client, then wrap it into the unified `AddressbookClientStd` via `From`.

```toml,ignore
[dependencies]
io-addressbook = "0.0.2"
```

```rust,no_run
use io_addressbook::{client::AddressbookClientStd, vdir::client::VdirClient};
use io_vdir::{client::VdirClient as InnerVdirClient, path::VdirPath};

let root = VdirPath::new("/home/alice/contacts");
let mut client = AddressbookClientStd::from(VdirClient::new(InnerVdirClient::new(root)));

for book in client.list_addressbooks().unwrap() {
    println!("{}: {}", book.id, book.name);
}
```

## Examples

Have a look at real-world projects built on top of this library:

- [Cardamum](https://github.com/pimalaya/cardamum): CLI to manage contacts

## AI disclosure

This project is developed with AI assistance. This section documents how, so users and downstream packagers can make informed decisions.

- **Tools**: Claude Code (Anthropic), Opus 4.8, invoked locally with a persistent project-scoped memory and a small set of repo-specific rules.

- **Used for**: Refactors, mechanical multi-file edits, boilerplate (feature gates, error enums, derive macros, trait impls), test scaffolding, doc polish, exploratory design conversations.

- **Not used for**: Engineering, critical code, git manipulation (commit, merge, rebase…), real-world tests.

- **Verification**: Every AI-assisted change is read, compiled, tested, and formatted before commit (`nix develop --command cargo check / cargo test / cargo fmt`). Behavioural correctness is verified against the relevant RFC or upstream spec, not assumed from the model output. Tests are never adjusted to fit AI-generated code; the code is adjusted to fit correct behaviour.

- **Limitations**: AI models occasionally produce code that compiles and passes tests but is subtly wrong: off-by-one errors, missed edge cases, plausible but nonexistent APIs, stale RFC references. The verification workflow catches most of this; it does not catch all of it. Bug reports are welcome and taken seriously.

- **Last reviewed**: 11/06/2026

## License

This project is licensed under either of:

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Social

- Chat on [Matrix](https://matrix.to/#/#pimalaya:matrix.org)
- News on [Mastodon](https://fosstodon.org/@pimalaya) or [RSS](https://fosstodon.org/@pimalaya.rss)
- Mail at [pimalaya.org@posteo.net](mailto:pimalaya.org@posteo.net)

## Sponsoring

[![nlnet](https://nlnet.nl/logo/banner-160x60.png)](https://nlnet.nl/)

Special thanks to the [NLnet foundation](https://nlnet.nl/) and the [European Commission](https://www.ngi.eu/) that have been financially supporting the project for years:

- 2022 → 2023: [NGI Assure](https://nlnet.nl/project/Himalaya/)
- 2023 → 2024: [NGI Zero Entrust](https://nlnet.nl/project/Pimalaya/)
- 2024 → 2026: [NGI Zero Core](https://nlnet.nl/project/Pimalaya-PIM/)
- *2027 in preparation…*

If you appreciate the project, feel free to donate using one of the following providers:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/soywod)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/soywod)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
