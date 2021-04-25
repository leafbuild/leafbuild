//! Contains the `execute_on` function.
use std::io;
use std::path::PathBuf;

use tracing::{span, Level};

use leafbuild_core::lf_buildsys::{ConfigurationError, WriteResultsError};

use crate::diagnostics::errors::LeafParseError;
use crate::env::FileFrame;
use crate::handle::Handle;

use super::LfModName;

mod internal;

/// Couldn't interpret something or validate something
#[derive(Error, Debug)]
pub enum InterpretFailure {
    /// Cannot read the source file
    #[error("cannot read file {0:?}: {1}")]
    CannotReadFile(PathBuf, #[source] io::Error),

    /// Cannot validate the configuration at the end.
    #[error(transparent)]
    Validate(#[from] ConfigurationError),

    /// Cannot write the results
    #[error(transparent)]
    CannotWriteResults(#[from] WriteResultsError),
}
/// Starts the interpreter on the given path, with the given handle and modifies the handle at the end.
/// The caller is responsible for validating and writing the results, by calling [`Handle::validate`]
/// and [`Handle::write_results`] after calling this.
/// # Errors
/// See [`InterpretFailure`]
pub fn execute_on(
    handle: &mut Handle,
    root_path: &PathBuf,
    mod_path: LfModName,
) -> Result<(), InterpretFailure> {
    let span = span!(Level::TRACE, "execute_on", path = %mod_path.0.as_str());
    let _span_guard = span.enter();
    info!("Entered {}", mod_path.0.as_str());

    let build_decl_file = root_path.join("build.leaf");
    let content = std::fs::read_to_string(build_decl_file)
        .map_err(|err| InterpretFailure::CannotReadFile(root_path.join("build.leaf"), err))?;
    let (node, errors) = leafbuild_syntax::parser::parse(&content);

    if errors.is_empty() {
        info!("Interpreting stuff");
        use leafbuild_syntax::ast::*;
        // use visitor::ThreadSafeVisitor;
        let fid = handle
            .buildsys
            .register_new_file(root_path.to_string_lossy().to_string(), content);
        let mut _frame = FileFrame::new(fid, mod_path);
        let root_node = BuildDefinition::cast(SyntaxNode::new_root(node)).unwrap();
        leafbuild_syntax::ast::print(0, SyntaxElement::Node(root_node.syntax().clone()));
        for lang_item in root_node.lang_items() {
            info!("Interpreting: ");
            leafbuild_syntax::ast::print(4, SyntaxElement::Node(lang_item.syntax().clone()));
            let statement = match lang_item {
                LangItem::StmtWrap(s) => s,
                _ => panic!(),
            };
            let expr = match statement.stmt().unwrap() {
                Stmt::ExprEvalStmt(e) => e,
                _ => panic!(),
            };
            let e = expr.expr().unwrap();
            // use internal::evaluator::ExprEvaluator;
            // let evaluator = ExprEvaluator::new();
            // let v = evaluator.visit_expr(e);
            // info!("{:?}", v);
        }
        // internal::run_build_def(&mut frame, build_definition);
    } else {
        handle.buildsys.register_file_and_report_chain(
            &root_path.to_string_lossy().to_string(),
            &content,
            |fid| {
                errors
                    .into_iter()
                    .map(move |err| LeafParseError::from((fid, err)))
            },
        );
    }

    info!("Leaving folder {:?}", root_path);

    Ok(())
}
