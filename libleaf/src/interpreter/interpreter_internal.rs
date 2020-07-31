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
        AstStatement::Declaration(decl) => run_declaration_in_env_frame(decl, env_frame),
        AstStatement::Assignment(assignment) => run_assignment_in_env_frame(assignment, env_frame),
    };
}

fn run_func_call_in_env_frame(call: &AstFuncCall, env_frame: &mut EnvFrame) {
    eval_call(
        call.get_name(),
        call.get_name_loc(),
        call.get_args(),
        env_frame,
        env_frame.get_pools_wrapper().get_global_pool(),
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
        &env_frame
            .get_pools_wrapper()
            .get_type_pool(base_value.get_value().get_type_id()),
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
                    .with_message(errors::get_error("Cannot find call", env_frame))
                    .with_labels(vec![Label::primary(env_frame.file_id, call_loc.as_rng())
                        .with_message(errors::get_error_string(
                            match base_value {
                                Some(val) => format!(
                                    "cannot find method '{}' for type `{}`",
                                    call_name,
                                    val.get_value().get_type_id(),
                                ),
                                None => format!("cannot find function '{}'", call_name),
                            },
                            env_frame,
                        ))]),
            );
            (get_print_executor().func)(args, env_frame, base_value)
        }
    }
}

fn run_declaration_in_env_frame(decl: &AstDeclaration, env_frame: &mut EnvFrame) {
    let value = decl.get_value().eval_in_env(env_frame);
    let name = decl.get_name();
    env_frame
        .variables
        .insert(name.clone(), Variable::new(name.clone(), value));
}

fn run_assignment_in_env_frame(assignment: &AstAssignment, env_frame: &mut EnvFrame) {
    let value = assignment.get_value();
    let bound_name_expr = assignment.get_bound();

    // if old_value.is_none() {
    //     errors::push_diagnostic(
    //         env_frame,
    //         Diagnostic::error()
    //             .with_message(
    //                 "Cannot perform composite assignment with value that doesn't exist",
    //             )
    //             .with_labels(vec![Label::primary(
    //                 env_frame.file_id,
    //                 bound_name_expr.get_rng(),
    //             )
    //             .with_message("doesn't exist")]),
    //     )
    // }
    match &assignment.get_op() {
        AstAtrOp::Atr => {
            let new_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    *val.reference = new_val;
                }
                Err(err) => errors::push_diagnostic(env_frame, err.diagnostic),
            }
        }
        AstAtrOp::AddAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    *val.reference = ops::op_add(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                    );
                }
                Err(err) => errors::push_diagnostic(env_frame, err.diagnostic),
            }
        }
        AstAtrOp::SubAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    *val.reference = ops::op_sub(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                    );
                }
                Err(err) => errors::push_diagnostic(env_frame, err.diagnostic),
            }
        }
        AstAtrOp::MulAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    *val.reference = ops::op_mul(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                    );
                }
                Err(err) => errors::push_diagnostic(env_frame, err.diagnostic),
            }
        }
        AstAtrOp::DivAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    *val.reference = ops::op_div(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                    );
                }
                Err(err) => errors::push_diagnostic(env_frame, err.diagnostic),
            }
        }
        AstAtrOp::ModAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    *val.reference = ops::op_mod(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                    );
                }
                Err(err) => errors::push_diagnostic(env_frame, err.diagnostic),
            }
        }
    };
}

include!("global_functions.rs");
