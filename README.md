# symfony-deprecation-contracts-rs

A Rust port of [symfony/deprecation-contracts](https://github.com/symfony/deprecation-contracts)
v3.7.1: `trigger_deprecation()`, delivered as PHP delivers a silenced
`E_USER_DEPRECATED` to the current error handler. Unofficial, not
affiliated with or endorsed by Symfony. MIT, as upstream.

Part of the php2rust project. The crate came from
[symfony-rs](https://github.com/php2rust/symfony-rs) `crates/deprecation-contracts`,
with its git history. It has no dependencies.

## Use

The crate is not on crates.io. Use a git dependency pinned to a rev:

```toml
[dependencies]
symfony-deprecation-contracts = { package = "php2rust-symfony-deprecation-contracts", git = "https://github.com/php2rust/symfony-deprecation-contracts-rs", rev = "<commit>" }
```

Twig, the Symfony components and other ports call `trigger_deprecation()`,
as their PHP packages require `symfony/deprecation-contracts`. A build must
take this crate from one git source (the same URL), so that one handler
sees every deprecation.

## Checks

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

See [crates/php2rust-symfony-deprecation-contracts](crates/php2rust-symfony-deprecation-contracts/README.md)
for the contract and the Rust shape.
