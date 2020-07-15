use crate::interpreter::{FuncCallPool, FuncCallExecutor, Value};

pub struct Handle {
    global_func_call_pool: FuncCallPool
}

impl Handle {
    pub fn new() -> Self {
        Self {
            global_func_call_pool: FuncCallPool::new(
                vec![
                    FuncCallExecutor::new(
                        |args, frame| {
                            Value::new(Box::new(0))
                        }
                    )
                ]
            )
        }
    }
    pub(crate) fn get_global_func_call_pool(&self) -> &FuncCallPool { &self.global_func_call_pool }
}