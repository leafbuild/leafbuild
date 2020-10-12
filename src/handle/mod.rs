// use crate::interpreter::{Env, EnvConfig};

pub struct Handle {
    // pub(crate) env: Env,
}

impl Handle {
    pub fn new(// cfg: EnvConfig
    ) -> Self {
        Self {
            // env: Env::new(cfg)
        }
    }

    pub(crate) fn write_results(&mut self) {
        // self.env.write_results().expect("Cannot write results");
    }
}
