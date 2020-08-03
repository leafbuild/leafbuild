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
            diagnostics::push_diagnostic(
                CannotFindCallError::new(
                    call_loc.as_rng(),
                    call_name,
                    match base_value {
                        Some(v) => Some(v.value.get_type_id()),
                        None => None,
                    },
                ),
                env_frame,
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

    let ctx = env_frame.get_diagnostics_ctx();
    let incompatible_assignment_handler: &dyn Fn(
        ValRef<Box<dyn ValueTypeMarker>>,
        Value<Box<dyn ValueTypeMarker>>,
        Location,
    ) = &|val, new_val, rng| {
        diagnostics::push_diagnostic_ctx(
            IncompatibleAssignmentError::new(
                bound_name_expr.get_rng(),
                rng,
                val.reference.get_value().stringify(),
                val.reference.base_type_id.typename(),
                new_val.base_type_id.typename(),
            ),
            ctx,
        );
    };
    match &assignment.get_op() {
        AstAtrOp::Atr => {
            let new_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    if is_assignable(
                        val.reference.get_base_type(),
                        &new_val.get_value().get_type_id(),
                    ) {
                        *val.reference = new_val;
                    } else {
                        incompatible_assignment_handler(val, new_val, value.get_rng());
                    }
                }
                Err(err) => diagnostics::push_diagnostic_ctx(err, ctx),
            }
        }
        AstAtrOp::AddAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    let new_val = ops::op_add(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                        ctx,
                    );
                    if is_assignable(
                        val.reference.get_base_type(),
                        &new_val.get_value().get_type_id(),
                    ) {
                        *val.reference = new_val;
                    } else {
                        incompatible_assignment_handler(val, new_val, assignment.get_rng());
                    }
                }
                Err(err) => diagnostics::push_diagnostic(err, env_frame),
            }
        }
        AstAtrOp::SubAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    let new_val = ops::op_sub(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                        ctx,
                    );
                    if is_assignable(
                        val.reference.get_base_type(),
                        &new_val.get_value().get_type_id(),
                    ) {
                        *val.reference = new_val;
                    } else {
                        incompatible_assignment_handler(val, new_val, assignment.get_rng());
                    }
                }
                Err(err) => diagnostics::push_diagnostic(err, env_frame),
            }
        }
        AstAtrOp::MulAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    let new_val = ops::op_mul(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                        ctx,
                    );
                    if is_assignable(
                        val.reference.get_base_type(),
                        &new_val.get_value().get_type_id(),
                    ) {
                        *val.reference = new_val;
                    } else {
                        incompatible_assignment_handler(val, new_val, assignment.get_rng());
                    }
                }
                Err(err) => diagnostics::push_diagnostic(err, env_frame),
            }
        }
        AstAtrOp::DivAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    let new_val = ops::op_div(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                        ctx,
                    );
                    if is_assignable(
                        val.reference.get_base_type(),
                        &new_val.get_value().get_type_id(),
                    ) {
                        *val.reference = new_val;
                    } else {
                        incompatible_assignment_handler(val, new_val, assignment.get_rng());
                    }
                }
                Err(err) => diagnostics::push_diagnostic(err, env_frame),
            }
        }
        AstAtrOp::ModAtr => {
            let right_val = value.eval_in_env(env_frame);
            let expr = bound_name_expr.eval_ref(env_frame);
            match expr {
                Ok(val) => {
                    let new_val = ops::op_mod(
                        &val.reference,
                        bound_name_expr.get_rng(),
                        &right_val,
                        value.get_rng(),
                        ctx,
                    );
                    if is_assignable(
                        val.reference.get_base_type(),
                        &new_val.get_value().get_type_id(),
                    ) {
                        *val.reference = new_val;
                    } else {
                        incompatible_assignment_handler(val, new_val, assignment.get_rng());
                    }
                }
                Err(err) => diagnostics::push_diagnostic(err, env_frame),
            }
        }
    };
}

fn is_assignable(prev_value_type: &TypeId, new_value_type: &TypeId) -> bool {
    match new_value_type {
        TypeId::Error => true,
        n => *n == *prev_value_type,
    }
}

include!("global_functions.rs");
