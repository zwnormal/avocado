use crate::cfg::Config;
use crate::db::SessionStore;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct State {
    pub(crate) session_store: Arc<dyn SessionStore>,
    pub(crate) config: Arc<Config>,
}

impl State {
    pub(crate) fn new(session_store: impl SessionStore + 'static) -> Self {
        State {
            session_store: Arc::new(session_store),
            config: Arc::new(Config::new()),
        }
    }
}
