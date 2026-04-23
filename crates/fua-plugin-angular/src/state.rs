use std::sync::{Mutex, OnceLock};

#[derive(Debug, Default)]
pub(crate) struct AngularState {
    pub(crate) condition_depth: usize,
    pub(crate) condition_base_indent: usize,
    pub(crate) awaiting_condition_start: bool,
}

fn state() -> &'static Mutex<AngularState> {
    static STATE: OnceLock<Mutex<AngularState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(AngularState::default()))
}

pub(crate) fn reset_state() {
    if let Ok(mut state) = state().lock() {
        *state = AngularState::default();
    }
}

pub(crate) fn with_state<T>(f: impl FnOnce(&mut AngularState) -> T) -> Option<T> {
    state().lock().ok().map(|mut state| f(&mut state))
}

pub(crate) fn read_state<T>(f: impl FnOnce(&AngularState) -> T) -> Option<T> {
    state().lock().ok().map(|state| f(&state))
}
