#[cfg(not(feature = "sync"))]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(feature = "sync"))]
use std::sync::Once;

#[cfg(not(feature = "sync"))]
static WORKS: AtomicUsize = AtomicUsize::new(0);
#[cfg(not(feature = "sync"))]
static INIT: Once = Once::new();

pub(crate) fn inside_proc_macro() -> bool {
    // When the `sync` feature is enabled, we always use the fallback
    // implementation because the real proc_macro types are not Send + Sync.
    #[cfg(feature = "sync")]
    return false;

    #[cfg(not(feature = "sync"))]
    {
        match WORKS.load(Ordering::Relaxed) {
            1 => return false,
            2 => return true,
            _ => {}
        }

        INIT.call_once(initialize);
        inside_proc_macro()
    }
}

#[cfg(not(feature = "sync"))]
pub(crate) fn force_fallback() {
    WORKS.store(1, Ordering::Relaxed);
}

#[cfg(feature = "sync")]
pub(crate) fn force_fallback() {
    // No-op when sync is enabled - always uses fallback
}

#[cfg(not(feature = "sync"))]
pub(crate) fn unforce_fallback() {
    initialize();
}

#[cfg(feature = "sync")]
pub(crate) fn unforce_fallback() {
    // No-op when sync is enabled - always uses fallback
}

#[cfg(all(not(feature = "sync"), not(no_is_available)))]
fn initialize() {
    let available = proc_macro::is_available();
    WORKS.store(available as usize + 1, Ordering::Relaxed);
}

// Swap in a null panic hook to avoid printing "thread panicked" to stderr,
// then use catch_unwind to determine whether the compiler's proc_macro is
// working. When proc-macro2 is used from outside of a procedural macro all
// of the proc_macro crate's APIs currently panic.
//
// The Once is to prevent the possibility of this ordering:
//
//     thread 1 calls take_hook, gets the user's original hook
//     thread 1 calls set_hook with the null hook
//     thread 2 calls take_hook, thinks null hook is the original hook
//     thread 2 calls set_hook with the null hook
//     thread 1 calls set_hook with the actual original hook
//     thread 2 calls set_hook with what it thinks is the original hook
//
// in which the user's hook has been lost.
//
// There is still a race condition where a panic in a different thread can
// happen during the interval that the user's original panic hook is
// unregistered such that their hook is incorrectly not called. This is
// sufficiently unlikely and less bad than printing panic messages to stderr
// on correct use of this crate. Maybe there is a libstd feature request
// here. For now, if a user needs to guarantee that this failure mode does
// not occur, they need to call e.g. `proc_macro0::Span::call_site()` from
// the main thread before launching any other threads.
#[cfg(all(not(feature = "sync"), no_is_available))]
fn initialize() {
    use std::panic::{self, PanicInfo};

    type PanicHook = dyn Fn(&PanicInfo) + Sync + Send + 'static;

    let null_hook: Box<PanicHook> = Box::new(|_panic_info| { /* ignore */ });
    let sanity_check = &*null_hook as *const PanicHook;
    let original_hook = panic::take_hook();
    panic::set_hook(null_hook);

    let works = panic::catch_unwind(proc_macro::Span::call_site).is_ok();
    WORKS.store(works as usize + 1, Ordering::Relaxed);

    let hopefully_null_hook = panic::take_hook();
    panic::set_hook(original_hook);
    if sanity_check != &*hopefully_null_hook {
        panic!("observed race condition in proc_macro0::inside_proc_macro");
    }
}
