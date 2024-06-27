# mdbook-pagecrypt

Encrypt your mdbook-built site with password protection.

Inspired by [pagecrypt](https://github.com/Greenheart/pagecrypt).

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
