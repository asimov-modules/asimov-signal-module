# ASIMOV Signal Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-signal-module)](https://crates.io/crates/asimov-signal-module)
[![Documentation](https://docs.rs/asimov-signal-module/badge.svg)](https://docs.rs/asimov-signal-module)

[ASIMOV] module for importing [Signal] chats.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install signal -v
```

### Installation from Source Code

```bash
cargo install asimov-signal-module
```

## 👉 Examples

```bash
asimov-signal-reader
```

## ⚙ Configuration

Signal Desktop stores data in an encrypted [SQLCipher] database. The encryption
key for this database is stored in a `config.json` file in Signal's application
data directory, and that key is itself encrypted using an encryption password
stored in the (platform-specific) system keychain.

This module can be configured to decrypt the Signal database using either the
encryption password or the encryption key. (You don't need both, just one.)

### Encryption Password

The simplest way to configure the module is to set the `ASIMOV_SIGNAL_PASSWORD`
environment variable to the encryption password stored in the system keychain:

```bash
# macOS
export ASIMOV_SIGNAL_PASSWORD=$(security find-generic-password -a "Signal Key" -s "Signal Safe Storage" -w)
```

### Encryption Key

Alternatively, for advanced users, you could set the `ASIMOV_SIGNAL_KEY`
environment variable to the actual decrypted value of `encryptedKey` found in
the `config.json` file:

```bash
export ASIMOV_SIGNAL_KEY=feedc0dedecafbadcafebabecafed00dfeedc0dedecafbadcafebabecafed00d
```

This key must be 64 hexadecimal characters, meaning 32 bytes (256 bits).
Deriving this key manually is well beyond the scope of these instructions.

## 📚 Reference

### Installed Binaries

- `asimov-signal-reader`: reads chats from the Signal data directory

### `asimov-signal-reader`

```
asimov-signal-reader

Usage: asimov-signal-reader [OPTIONS] [SIGNAL-DIR]

Arguments:
  [SIGNAL-DIR]  Path to the Signal data directory

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  Set the output format [default: jsonl] [possible values: jsonl]
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-signal-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-signal-module&text=asimov-signal-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-signal-module&title=asimov-signal-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-signal-module&t=asimov-signal-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-signal-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-signal-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
[Signal]: https://signal.org
[Signal Desktop]: https://github.com/signalapp/Signal-Desktop
[SQLCipher]: https://www.zetetic.net/sqlcipher/
