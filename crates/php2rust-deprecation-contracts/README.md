# Symfony Deprecation Contracts

Private workspace crate: a port of `symfony/deprecation-contracts` v3.7.1
(`f3202fa1`). MIT; Symfony source copyright Fabien Potencier. Unofficial,
not affiliated with or endorsed by Symfony.

| Contract | Source | Evidence |
|---|---|---|
| `trigger_deprecation($package, $version, $message)`: "Since $package $version: " when either is truthy (`"0"` is not), then the message, delivered as a silenced `E_USER_DEPRECATED` to the current error handler | `function.php` | unit tests, checked against PHP 8.4.27 |

## Rust shape

PHP's error handler (`set_error_handler()`) is per thread here:
`with_handler(handler, f)` installs one while `f` runs, and `collect(f)`
returns the deprecations `f` triggered. With no handler a deprecation is
dropped, as PHP drops a silenced error nobody handles.

## Not ported

The `...$args` form (`vsprintf()` of the message): no ported caller uses it.
Upstream has no tests.
