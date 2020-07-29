fn run_in_env_frame(statement: &AstStatement, env_frame: &mut EnvFrame) {
    match statement {
        AstStatement::FuncCall(call) => {
            run_func_call_in_env_frame(call, env_frame);
        }
        AstStatement::MethodCall(call) => run_method_call_in_env_frame(
            &call.get_base_expr().eval_in_env(env_frame),
            call.get_name(),
            call.get_name_loc(),
            call.get_args(),
            env_frame,
        ),
        AstStatement::Assignment(assignment) => run_assignment_in_env_frame(assignment, env_frame),
    };
}

fn run_func_call_in_env_frame(call: &AstFuncCall, env_frame: &mut EnvFrame) {
    eval_call(
        call.get_name(),
        call.get_name_loc(),
        call.get_args(),
        env_frame,
        &get_global_functions(),
        None,
    );
}

fn run_method_call_in_env_frame(
    base_value: &Value<Box<dyn ValueTypeMarker>>,
    call_name: &str,
    call_loc: &TokLoc,
    call_args: &AstFuncCallArgs,
    env_frame: &mut EnvFrame,
) {
    eval_call(
        call_name,
        call_loc,
        call_args,
        env_frame,
        &get_global_functions(),
        Some(base_value),
    );
}

/// Evaluate a method or a function call, depending on base_value

fn eval_call(
    call_name: &str,
    call_loc: &TokLoc,
    args: &AstFuncCallArgs,
    env_frame: &mut EnvFrame,
    func_call_poll: &CallPool,
    base_value: Option<&Value<Box<dyn ValueTypeMarker>>>,
) -> Value<Box<dyn ValueTypeMarker>> {
    let executor = func_call_poll
        .executors
        .iter()
        .find(|executor| executor.name == call_name);
    match executor {
        Some(exe) => (exe.func)(args, env_frame, base_value),
        None => {
            errors::push_diagnostic(
                env_frame,
                Diagnostic::error()
                    .with_message("Cannot find call")
                    .with_labels(vec![Label::primary(env_frame.file_id, call_loc.as_rng())
                        .with_message(format!(
                            "cannot find {} '{}'",
                            match base_value {
                                Some(_) => "method",
                                None => "function",
                            },
                            call_name
                        ))]),
            );
            (get_print_executor().func)(args, env_frame, base_value)
        }
    }
}

fn run_assignment_in_env_frame(assignment: &AstAssignment, env_frame: &mut EnvFrame) {
    let value = assignment.get_value().eval_in_env(env_frame);
    let name = assignment.get_name().clone();
    let old_value = env_frame.variables.get(&name);
    let value = match &assignment.get_op() {
        AstAtrOp::Atr => value,
        AstAtrOp::AddAtr => ops::op_add(
            old_value
                .unwrap_or_else(|| {
                    panic!("Cannot composite assignment on variable that doesn't exist")
                })
                .value
                .get_value()
                .clone_to_value(),
            value,
        ),
        AstAtrOp::SubAtr => ops::op_sub(
            old_value
                .unwrap_or_else(|| {
                    panic!("Cannot composite assignment on variable that doesn't exist")
                })
                .value
                .get_value()
                .clone_to_value(),
            value,
        ),
        AstAtrOp::MulAtr => ops::op_add(
            old_value
                .unwrap_or_else(|| {
                    panic!("Cannot composite assignment on variable that doesn't exist")
                })
                .value
                .get_value()
                .clone_to_value(),
            value,
        ),
        AstAtrOp::DivAtr => ops::op_add(
            old_value
                .unwrap_or_else(|| {
                    panic!("Cannot composite assignment on variable that doesn't exist")
                })
                .value
                .get_value()
                .clone_to_value(),
            value,
        ),
        AstAtrOp::ModAtr => ops::op_add(
            old_value
                .unwrap_or_else(|| {
                    panic!("Cannot composite assignment on variable that doesn't exist")
                })
                .value
                .get_value()
                .clone_to_value(),
            value,
        ),
    };
    let var = Variable::new(assignment.get_name().clone(), value);
    env_frame
        .variables
        .insert(assignment.get_name().clone(), var);
}

include!("global_functions.rs");
