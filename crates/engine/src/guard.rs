//! Last-resort panic logging for transactional session edits.
//! Adapted from deckcraft be14248577b92059eb2b8c0c9460cceffbc1f01d,
//! MIT OR Apache-2.0. Session::edit restores the full session on unwind.

/// Text of a caught panic payload.
pub fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic".into())
}

/// Install logging once, retaining the application's previous panic hook.
pub fn install_panic_hook() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let at = info.location().map(|l| format!(" at {}:{}", l.file(), l.line())).unwrap_or_default();
            log::error!("panic{at}: {}", panic_message(info.payload()));
            previous(info);
        }));
    });
}

#[cfg(test)]
thread_local! {
    pub(crate) static PANIC_POINT: std::cell::Cell<Option<&'static str>> = const { std::cell::Cell::new(None) };
}

/// Fault injection is absent from default builds, including release artifacts.
#[cfg(any(test, feature = "test-panic-injection"))]
pub(crate) fn inject(point: &'static str) {
    #[cfg(test)]
    let armed = PANIC_POINT.with(|p| p.get() == Some(point));
    #[cfg(not(test))]
    let armed = std::env::var("MISTERDIAGRAM_TEST_PANIC").is_ok_and(|p| p == point);
    if armed {
        std::panic::resume_unwind(Box::new(format!("injected {point} panic")));
    }
}
