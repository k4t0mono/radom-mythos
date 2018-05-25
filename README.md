# random-mythos

![GitHub release](https://img.shields.io/github/release/k4t0mono/random-mythos.svg?style=flat-square)
[![Travis](https://img.shields.io/travis/k4t0mono/random-mythos/releases.svg?style=flat-square)](https://travis-ci.org/k4t0mono/random-mythos)
[![license](https://img.shields.io/github/license/k4t0mono/random-mythos.svg?style=flat-square)](https://github.com/k4t0mono/random-mythos/blob/master/LICENSE)
![be yourself](https://img.shields.io/badge/Be-yourself-ea51b2.svg?style=flat-square)
![vim](https://img.shields.io/badge/Made%20with-VIM-00f769.svg?style=flat-square)

A procedural mythos generator

## Usage

Just run `./random-mythos [options] <file>`, where `file` is the file to export
the generated mythos.

### Options

| Flag | Description |
| --- | --- |
| `-h --help` | Show help |
| `-v --version` | Show version |
| `-d --gen-dot` | Generate relations' graph dot file |
| `--verbose=<n>` | Set log level |
| `--export=<json>` | Export relations to JSON file |
| `--import=<json>` | Import relations from JSON file |

#### Verbose levels

| Level | Description |
| --- | --- |
| 0 | Off |
| 1 | Error |
| 2 | Warn |
| 3 | Info|
| 4 | Debug |
| 5 | Trace |

## Thanks 💖

- [rust-lang-nursery/rand](https://github.com/rust-lang-nursery/rand)
- [drakulix/simplelog](github.com/drakulix/simplelog.rs)
- [rust-lang/log](github.com/rust-lang/log)
- [serde-rs/json](github.com/serde-rs/json)
- [docopt/docopt.rs](github.com/docopt/docopt.rs)
- [serde-rs/serde](github.com/serde-rs/serde)

## License

Under BSD-3 Clause Licence
Copyright (c) 2018, KatoMono Enkeli. All rights reserved.
