#[cfg(not(feature = "sync"))]
use alloc::rc::Rc;
#[cfg(feature = "sync")]
use alloc::sync::Arc;
use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};

// Zero sized marker with the correct set of autotrait impls we want all proc
// macro types to have.
//
// When the `sync` feature is enabled, we use Arc instead of Rc to make
// types Send + Sync for thread-safe usage.
#[derive(Copy, Clone)]
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
#[cfg(not(feature = "sync"))]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);

#[derive(Copy, Clone)]
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
#[cfg(feature = "sync")]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Arc<()>>);

pub(crate) const MARKER: ProcMacroAutoTraits = ProcMacroAutoTraits(PhantomData);

impl UnwindSafe for ProcMacroAutoTraits {}
impl RefUnwindSafe for ProcMacroAutoTraits {}
