// use crate::interpreter::{Env, EnvConfig};

pub mod config;

use crate::interpreter::env::Env;
use std::path::PathBuf;

pub struct Handle<'a> {
    pub(crate) env: Env<'a>,
}

impl<'a> Handle<'a> {
    pub fn new(cfg: config::EnvConfig) -> Self {
        Self { env: Env::new(cfg) }
    }

    pub(crate) fn write_results(&mut self) {
        // self.env.write_results().expect("Cannot write results");
    }

    pub(crate) fn get_env(&self) -> &Env {
        &self.env
    }

    pub(crate) fn get_env_mut(&'a mut self) -> &'a mut Env {
        &mut self.env
    }
}
