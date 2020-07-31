use crate::interpreter::EnvFrameReturns;
use crate::interpreter::{Env, EnvConfig};

pub struct Handle {
    pub(crate) env: Env,
    env_frame_returns: EnvFrameReturns,
}

impl Handle {
    pub fn new(cfg: EnvConfig) -> Self {
        Self {
            env: Env::new(cfg),
            env_frame_returns: EnvFrameReturns::empty(),
        }
    }

    pub(crate) fn push_env_frame_returns(&mut self, env_frame_returns: EnvFrameReturns) {
        self.env_frame_returns = env_frame_returns
    }
}
