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
        AstStatement::Conditional(conditional) => {
            run_conditional_in_env_frame(conditional, env_frame)
        }
        AstStatement::ControlStatement(_) => {
            //TODO: will need loops to implement this
        }
        AstStatement::Repetitive(repetitive) => {
            run_repetitive_statement_in_env_frame(repetitive, env_frame)
        }
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
    let executor = func_call_poll.executors.iter().find(|executor| {
        executor.name == call_name || executor.aliases.iter().any(|name| *name == call_name)
    });
    match executor {
        Some(exe) => (exe.func)(call_loc.as_rng(), args, env_frame, base_value),
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
            (functions::get_print_executor().func)(call_loc.as_rng(), args, env_frame, base_value)
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
        ValRefMut<Box<dyn ValueTypeMarker>>,
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
            let expr = bound_name_expr.eval_mut_ref(env_frame);
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
            let expr = bound_name_expr.eval_mut_ref(env_frame);
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
            let expr = bound_name_expr.eval_mut_ref(env_frame);
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
            let expr = bound_name_expr.eval_mut_ref(env_frame);
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
            let expr = bound_name_expr.eval_mut_ref(env_frame);
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
            let expr = bound_name_expr.eval_mut_ref(env_frame);
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

fn run_conditional_in_env_frame(conditional: &AstConditionalStatement, frame: &mut EnvFrame) {
    if run_if(conditional.get_initial_if(), frame) {
        return;
    }
    for else_if in conditional.get_else_ifs() {
        if run_if(else_if.get_if(), frame) {
            return;
        }
    }
    if let Some(else_) = conditional.get_else() {
        for stat in else_.get_statements() {
            run_in_env_frame(stat, frame);
        }
    }
}

/// returns true if the condition evaluated to true and it executed the branch
fn run_if(if_: &AstIf, frame: &mut EnvFrame) -> bool {
    match if_
        .get_condition()
        .eval_in_env(frame)
        .get_type_id_and_value()
        .get_bool()
        .unwrap_or_else(|tpid| {
            diagnostics::push_diagnostic(
                ExpectedTypeError::new(
                    TypeId::Bool.typename(),
                    ExprLocAndType::new(if_.get_condition().get_rng(), tpid.typename()),
                ),
                frame,
            );
            false
        }) {
        true => {
            for stat in if_.get_statements() {
                run_in_env_frame(stat, frame);
            }
            true
        }
        false => false,
    }
}

fn run_repetitive_statement_in_env_frame(
    repetitive: &AstRepetitiveStatement,
    frame: &mut EnvFrame,
) {
    let for_in_expr = repetitive.get_for_in_expr();
    let name = for_in_expr.get_name();
    let iterable_expr = for_in_expr.get_expr();
    let iterable = iterable_expr.eval_in_env(frame);
    match iterable.get_type_id_and_value() {
        TypeIdAndValue::Vec(v) => v.iter().for_each(|x| {
            frame.variables.insert(
                name.0.clone(),
                Variable::new(name.0.clone(), x.clone_to_value()),
            );
            repetitive
                .get_statements()
                .iter()
                .for_each(|s| run_in_env_frame(s, frame))
        }),
        TypeIdAndValue::Map(m) => m.iter().for_each(|x| {
            frame.variables.insert(
                name.0.clone(),
                Variable::new(
                    name.0.clone(),
                    Value::new(Box::new(MapPair::new(x.0.clone(), x.1.clone_to_value()))),
                ),
            );
            repetitive
                .get_statements()
                .iter()
                .for_each(|s| run_in_env_frame(s, frame))
        }),
        tp => {
            diagnostics::push_diagnostic(
                ExpectedTypeError::new(
                    format!("{} or {}", TypeId::Vec.typename(), TypeId::Map.typename()),
                    ExprLocAndType::new(iterable_expr.get_rng(), tp.degrade().typename()),
                ),
                frame,
            );
            return;
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
