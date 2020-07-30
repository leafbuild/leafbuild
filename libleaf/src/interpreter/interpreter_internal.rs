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
    let value = assignment.get_value();
    let bound_name_expr = assignment.get_bound();
    let empty_name = "".to_string();
    let name = match bound_name_expr {
        Expr::Atom(atom) => match atom {
            Atom::Id((id, _)) => id,
            Atom::Number(_) => &empty_name,
            Atom::Str(_) => &empty_name,
        },
        Expr::Op(_, _, _) => &empty_name,
        Expr::FuncCall(_) => &empty_name,
        Expr::MethodCall(_) => &empty_name,
        Expr::PropertyAccess(_) => &empty_name,
    };
    if *name == "" {
        errors::push_diagnostic(
            env_frame,
            Diagnostic::error()
                .with_message("Assignment on something that isn't an id")
                .with_labels(vec![Label::primary(
                    env_frame.file_id,
                    bound_name_expr.get_rng(),
                )
                .with_message("not a variable")]),
        )
    }
    let old_value = env_frame.variables.get(name);
    let value = match &assignment.get_op() {
        AstAtrOp::Atr => value.eval_in_env(env_frame),
        AstAtrOp::AddAtr => {
            if let None = old_value {
                errors::push_diagnostic(
                    env_frame,
                    Diagnostic::error()
                        .with_message(
                            "Cannot perform composite assignment with value that doesn't exist",
                        )
                        .with_labels(vec![Label::primary(
                            env_frame.file_id,
                            bound_name_expr.get_rng(),
                        )
                        .with_message("doesn't exist")]),
                )
            }
            ops::op_add(bound_name_expr, value, env_frame)
        }
        AstAtrOp::SubAtr => {
            if let None = old_value {
                errors::push_diagnostic(
                    env_frame,
                    Diagnostic::error()
                        .with_message(
                            "Cannot perform composite assignment with value that doesn't exist",
                        )
                        .with_labels(vec![Label::primary(
                            env_frame.file_id,
                            bound_name_expr.get_rng(),
                        )
                        .with_message("doesn't exist")]),
                )
            }
            ops::op_sub(bound_name_expr, value, env_frame)
        }
        AstAtrOp::MulAtr => {
            if let None = old_value {
                errors::push_diagnostic(
                    env_frame,
                    Diagnostic::error()
                        .with_message(
                            "Cannot perform composite assignment with value that doesn't exist",
                        )
                        .with_labels(vec![Label::primary(
                            env_frame.file_id,
                            bound_name_expr.get_rng(),
                        )
                        .with_message("doesn't exist")]),
                )
            }
            ops::op_mul(bound_name_expr, value, env_frame)
        }
        AstAtrOp::DivAtr => {
            if let None = old_value {
                errors::push_diagnostic(
                    env_frame,
                    Diagnostic::error()
                        .with_message(
                            "Cannot perform composite assignment with value that doesn't exist",
                        )
                        .with_labels(vec![Label::primary(
                            env_frame.file_id,
                            bound_name_expr.get_rng(),
                        )
                        .with_message("doesn't exist")]),
                )
            }
            ops::op_div(bound_name_expr, value, env_frame)
        }
        AstAtrOp::ModAtr => {
            if let None = old_value {
                errors::push_diagnostic(
                    env_frame,
                    Diagnostic::error()
                        .with_message(
                            "Cannot perform composite assignment with value that doesn't exist",
                        )
                        .with_labels(vec![Label::primary(
                            env_frame.file_id,
                            bound_name_expr.get_rng(),
                        )
                        .with_message("doesn't exist")]),
                )
            }
            ops::op_mod(bound_name_expr, value, env_frame)
        }
    };
    let var = Variable::new(name.clone(), value);
    env_frame.variables.insert(name.clone(), var);
}

include!("global_functions.rs");
