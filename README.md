## thaumiel
[![Build Status](https://travis-ci.org/Nu-SCPTheme/thaumiel.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/thaumiel)

A Wikidot-like web server to provide pages, forums, and other wiki services using backends such as [DEEPWELL](https://github.com/Nu-SCPTheme/deepwell).

The lint `#![forbid(unsafe_code)]` is set, and therefore this crate has only safe code. However dependencies may have `unsafe` internals.

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.42.0

You must first install Postgres bindings. On a Debian-based operating system, this would look like:

```sh
$ sudo apt install libpq-dev
```

Then, build using:

```sh
$ cargo build --release
```

Before running the binary, create a random cookie key at least 32 bytes long:

```sh
$ head -c 32 /dev/urandom > thaumiel-cookie.key
```

`thaumiel-cookie.key` is not a set path and can be modified in the configuration.

This will create the final `thaumiel` binary, which can be executed using the following:

```sh
$ cargo run -- [arguments]
```

However, thaumiel requires several services to be running, which in turn have requirements such as a Postgres database to write to. Also be sure the port numbers configured here and in the respective services match.

* [DEEPWELL](https://github.com/Nu-SCPTheme/deepwell-rpc)
* [ftml](https://github.com/Nu-SCPTheme/ftml-rpc)

Example nginx configuration is present in `misc/nginx`.
