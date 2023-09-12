use crate::cfg::Config;
use crate::db::UserStore;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub(crate) struct State {
    pub(crate) user_store: Arc<dyn UserStore>,
    pub(crate) config: Arc<Config>,
}

impl State {
    pub(crate) fn new(user_store: impl UserStore + 'static) -> Self {
        State {
            user_store: Arc::new(user_store),
            config: Arc::new(Config::new()),
        }
    }
}
