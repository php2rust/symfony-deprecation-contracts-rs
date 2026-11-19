//! Port of symfony/deprecation-contracts' function.php:
//! `trigger_deprecation()`.
//!
//! Upstream raises a silenced `E_USER_DEPRECATED` with `@trigger_error()`.
//! A silenced error still reaches the error handler set with
//! `set_error_handler()`, and is otherwise dropped. PHP's error handler
//! is per request, on one thread; here it is per thread: [`with_handler`]
//! installs one for the duration of a call, as a test does with
//! `set_error_handler()` / `restore_error_handler()`. With no handler, a
//! deprecation is dropped.
//!
//! Symfony source copyright (c) Fabien Potencier; distributed under MIT.
//!
//! Upstream: symfony/deprecation-contracts function.php

#![forbid(unsafe_code)]

use std::cell::RefCell;

/// A deprecation as the error handler receives it: `E_USER_DEPRECATED`
/// with the message `trigger_deprecation()` built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deprecation {
    pub message: Vec<u8>,
}

type Handler = Box<dyn FnMut(&Deprecation)>;

thread_local! {
    static HANDLERS: RefCell<Vec<Handler>> = const { RefCell::new(Vec::new()) };
}

/// `trigger_deprecation($package, $version, $message)`: "Since $package
/// $version: " (when either is truthy in PHP, so `"0"` counts as empty)
/// followed by the message. The `...$args` form (`vsprintf()`) is not
/// ported; no ported caller uses it.
pub fn trigger_deprecation(package: &[u8], version: &[u8], message: &[u8]) {
    let mut text = Vec::new();
    if truthy(package) || truthy(version) {
        text.extend_from_slice(b"Since ");
        text.extend_from_slice(package);
        text.push(b' ');
        text.extend_from_slice(version);
        text.extend_from_slice(b": ");
    }
    text.extend_from_slice(message);
    let deprecation = Deprecation { message: text };
    // The innermost handler gets it, as PHP calls only the current one. It
    // is taken out while it runs, so a deprecation it triggers itself goes
    // to the one below it rather than re-entering it.
    let handler = HANDLERS.with(|h| h.borrow_mut().pop());
    if let Some(mut handler) = handler {
        handler(&deprecation);
        HANDLERS.with(|h| h.borrow_mut().push(handler));
    }
}

/// Runs `f` with `handler` receiving the deprecations triggered on this
/// thread, then restores the previous handler (also when `f` panics).
pub fn with_handler<R>(handler: impl FnMut(&Deprecation) + 'static, f: impl FnOnce() -> R) -> R {
    struct Restore;
    impl Drop for Restore {
        fn drop(&mut self) {
            HANDLERS.with(|h| h.borrow_mut().pop());
        }
    }
    HANDLERS.with(|h| h.borrow_mut().push(Box::new(handler)));
    let _restore = Restore;
    f()
}

/// Runs `f` and returns what it returned with the deprecations it
/// triggered, in order.
pub fn collect<R>(f: impl FnOnce() -> R) -> (R, Vec<Deprecation>) {
    let seen = std::rc::Rc::new(RefCell::new(Vec::new()));
    let sink = seen.clone();
    let result = with_handler(move |d| sink.borrow_mut().push(d.clone()), f);
    let deprecations = seen.borrow().clone();
    (result, deprecations)
}

/// PHP's truthiness of a string: false for `""` and `"0"`.
fn truthy(s: &[u8]) -> bool {
    !(s.is_empty() || s == b"0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_prefix_follows_php_truthiness() {
        let (_, seen) = collect(|| {
            trigger_deprecation(b"vendor/pkg", b"1.1", b"gone");
            trigger_deprecation(b"", b"", b"bare");
            trigger_deprecation(b"0", b"0", b"zeros");
            trigger_deprecation(b"", b"2.0", b"version only");
        });
        let texts: Vec<_> = seen.iter().map(|d| d.message.clone()).collect();
        assert_eq!(
            texts,
            vec![
                b"Since vendor/pkg 1.1: gone".to_vec(),
                b"bare".to_vec(),
                b"zeros".to_vec(),
                b"Since  2.0: version only".to_vec(),
            ]
        );
    }

    #[test]
    fn dropped_without_handler_and_handler_restored() {
        trigger_deprecation(b"a", b"1", b"nobody listens");
        let (_, outer) = collect(|| {
            let (_, inner) = collect(|| trigger_deprecation(b"a", b"1", b"inner"));
            assert_eq!(inner.len(), 1);
            trigger_deprecation(b"a", b"1", b"outer");
        });
        assert_eq!(outer.len(), 1);
        assert_eq!(outer[0].message, b"Since a 1: outer".to_vec());
    }
}
