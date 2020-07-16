fn run_in_env_frame(handle: &Handle, statement: &AstStatement, env_frame: &mut EnvFrame) {
    match statement {
        AstStatement::FuncCall(call) => {
            run_func_call_in_env_frame(handle, call, env_frame);
        }
        AstStatement::MethodCall(call) => {}
        AstStatement::Assignment(assignment) => {}
    };
}

fn run_func_call_in_env_frame(handle: &Handle, call: &AstFuncCall, env_frame: &mut EnvFrame) {
    eval_func_call(call, env_frame, handle.get_global_func_call_pool());
}

fn eval_func_call(call: &AstFuncCall, env_frame: &mut EnvFrame, func_call_poll: &FuncCallPool) -> Value<Box<dyn ValueTypeMarker>> {
    (
        func_call_poll.executors.iter().find(
            |executor| { executor.name == *call.get_name() }
        ).unwrap().func
    )(call.get_args(), env_frame)
}