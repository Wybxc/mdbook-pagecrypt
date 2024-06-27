# mdbook-pagecrypt

[![docs.rs](https://img.shields.io/docsrs/mdbook-pagecrypt)](https://docs.rs/mdbook-pagecrypt)
[![Crates.io Version](https://img.shields.io/crates/v/mdbook-pagecrypt)](https://crates.io/crates/mdbook-pagecrypt)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Wybxc/mdbook-pagecrypt/mdbook.yml)](https://github.com/Wybxc/mdbook-pagecrypt/actions)

Encrypt your mdbook-built site with password protection.

Inspired by [pagecrypt](https://github.com/Greenheart/pagecrypt).

Documentation can be found [here](https://wybxc.github.io/mdbook-pagecrypt/). The documentation site is also a live demo of the library; the password is `pagecrypt`.

## Installation

```shell
cargo install mdbook-pagecrypt
```

## Usage

Add the following to your `book.toml`:

```toml
[output.pagecrypt]
password = "secret"
rounds = 600_000 # optional, rounds in password hashing
```

And then run `mdbook build` to encrypt the site.
