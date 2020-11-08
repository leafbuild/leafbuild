// use crate::interpreter::{Env, EnvConfig};

pub mod config;

use crate::interpreter::env::Env;

pub struct Handle<'a> {
    pub(crate) env: Env<'a>,
}

impl<'a> Handle<'a> {
    #[must_use]
    pub fn new(cfg: config::Config) -> Self {
        Self { env: Env::new(cfg) }
    }

    pub(crate) fn write_results(&self) {
        self.env.write_results().expect("Cannot write results");
    }

    pub(crate) const fn get_env(&self) -> &Env {
        &self.env
    }

    pub(crate) fn get_env_mut(&'a mut self) -> &'a mut Env {
        &mut self.env
    }
}
