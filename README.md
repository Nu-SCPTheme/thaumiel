## thaumiel
[![Build Status](https://travis-ci.org/Nu-SCPTheme/thaumiel.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/thaumiel)

A Wikidot-like web server to provide pages, forums, and other wiki services using backends such as [DEEPWELL](https://github.com/Nu-SCPTheme/deepwell).

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.40.0

You must first install Postgres bindings. On a Debian-based operating system, this would look like:

```sh
$ sudo apt install libpq-dev
```

Then, build using:

```sh
$ cargo build --release
```

Before running the binary, create a cookie key by using 512 bytes of random data:

```sh
$ head -c 32 /dev/urandom > thaumiel-cookie.key
```

`thaumiel-cookie.key` is not a set path and can be modified in the configuration.

This will create the final `thaumiel` binary, which can be executed using the following:

```sh
$ cargo run -- [arguments]
```
