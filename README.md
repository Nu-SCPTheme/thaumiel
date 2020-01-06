## thaumiel
[![Build Status](https://travis-ci.org/Nu-SCPTheme/thaumiel.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/thaumiel)

A Wikidot-like web server to provide pages, forums, and other wiki services for backends like [DEEPWELL](https://github.com/Nu-SCPTheme/deepwell).

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.39.0

```sh
$ cargo build --release
```

This will create the final `thaumiel` binary, which can be executed using the following:

```sh
$ cargo run -- [arguments]
```
