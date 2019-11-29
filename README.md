## kant-router
[![Build Status](https://travis-ci.org/Nu-SCPTheme/kant-router.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/kant-router)

A Wikidot-compatible router for web applications, forwarding requests to page, forum, or other services and enabling caching and session management.

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.39.0

```sh
$ cargo build --release
```

This will create the final `kant-router` binary, which can be executed using the following:

```sh
$ cargo run -- [arguments]
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
