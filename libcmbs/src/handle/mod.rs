use crate::interpreter::{FuncCallPool, FuncCallExecutor, Value, EnvFrameReturns};

pub struct Handle {
    global_func_call_pool: FuncCallPool,
    env_frame_returns: EnvFrameReturns,
}

impl Handle {
    pub fn new() -> Self {
        Self {
            global_func_call_pool: FuncCallPool::new(
                vec![
                    FuncCallExecutor::new("f".to_string(),
                                          |args, frame| {
                                              println!("Called f!");
                                              Value::new(Box::new(0))
                                          },
                    )
                ]
            ),
            env_frame_returns: EnvFrameReturns::empty(),
        }
    }
    pub(crate) fn get_global_func_call_pool(&self) -> &FuncCallPool { &self.global_func_call_pool }

    pub(crate) fn push_env_frame_returns(&mut self, env_frame_returns: EnvFrameReturns) {
        self.env_frame_returns = env_frame_returns
    }
}