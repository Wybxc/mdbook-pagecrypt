# User Guide

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

## Configuration

- `password`: The password to encrypt the site with. If not specified, defaults to `pagecrypt`.
- `rounds`: The number of rounds to use for password hashing. Default is 600_000. Bigger numbers are safer but slower.

## Security

The encryption is powered by AES symmetric encryption and salted password hashing. It is hard to crack using brute force.

- Encryption is performed with the [AES-GCM](https://crates.io/crates/aes-gcm) crate.
- Password hashing is performed with the [pbkdf2](https://crates.io/crates/pbkdf2) crate.
