use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use libutils::{
    compilers::{cc::*, cxx::*, *},
    generators::{ninja::*, *},
};

use crate::interpreter::Env;
use itertools::Itertools;

pub(crate) fn write_to(env: &mut Env, dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let ninja_file = dir.join("build.ninja");
    let f = File::create(ninja_file)?;

    let mut gen = NinjaGen::new();

    gen.new_global_value(
        "lfb_bin",
        std::env::current_exe()
            .expect("Cannot get current exe")
            .to_string_lossy(),
    );

    let signal_build_failure = env.imut.diagnostics_ctx.get_signal_build_failure();

    let internal_compilation_failed_call = " || $lfb_bin internal compilation-failed --exit-code $$? --in \"$in\" --out \"$out\" --module-id $mod_id";
    let internal_linking_failed_call = " || $lfb_bin internal link-failed --exit-code $$? --in \"$in\" --out \"$out\" --module-id $mod_id";

    let cc_compile = gen.new_rule(
        "cc_compile",
        NinjaCommand::new(format!(
            "$CC -MD -MF $out.d $include_dirs $in -c -o $out $CC_FLAGS{}",
            if signal_build_failure {
                internal_compilation_failed_call
            } else {
                ""
            }
        )),
        vec![NinjaVariable::new("depfile", "$out.d")],
    );
    let cc_link = gen.new_rule(
        "cc_link",
        NinjaCommand::new(format!(
            "$CC $in -o $out $CCLD_FLAGS{}",
            if signal_build_failure {
                internal_linking_failed_call
            } else {
                ""
            }
        )),
        vec![],
    );

    let cxx_compile = gen.new_rule(
        "cxx_compile",
        NinjaCommand::new(format!(
            "$CXX -MD -MF $out.d $include_dirs $in -c -o $out $CXX_FLAGS{}",
            if signal_build_failure {
                internal_compilation_failed_call
            } else {
                ""
            }
        )),
        vec![NinjaVariable::new("depfile", "$out.d")],
    );
    let cxx_link = gen.new_rule(
        "cxx_link",
        NinjaCommand::new(format!(
            "$CXX $in -o $out $CXXLD_FLAGS{}",
            if signal_build_failure {
                internal_linking_failed_call
            } else {
                ""
            }
        )),
        vec![],
    );

    let mut need_cc = false;
    let mut need_cxx = false;

    env.mut_.executables.iter().for_each(|exe| {
        let mut need_cxx_linker = false;

        let exe_args: Vec<NinjaRuleArg> = exe
            .get_sources()
            .iter()
            .filter_map(|src| {
                let rl;
                if CC::can_compile(src) {
                    rl = &cc_compile;
                    need_cc = true;
                } else if CXX::can_compile(src) {
                    rl = &cxx_compile;
                    need_cxx = true;
                    need_cxx_linker = true;
                } else if CC::can_consume(src) || CXX::can_consume(src) {
                    return None;
                } else {
                    println!(
                        "Warning: ignoring file '{}' while building executable '{}'",
                        src,
                        exe.get_name()
                    );
                    return None;
                }
                let t = gen.new_target(
                    format!("{}@Exe/{}.o", exe.get_name(), src),
                    rl,
                    vec![NinjaRuleArg::new(format!("../{}", src))],
                    vec![],
                    vec![
                        NinjaVariable::new("mod_id", exe.get_mod_id().to_string()),
                        NinjaVariable::new(
                            "include_dirs",
                            exe.get_include_dirs()
                                .iter()
                                .map(|inc_dir| format!("-I../{}", inc_dir))
                                .join(" "),
                        ),
                    ],
                );
                Some(NinjaRuleArg::new(t.get_name()))
            })
            .collect();
        gen.new_target(
            exe.get_name(),
            if need_cxx_linker { &cxx_link } else { &cc_link },
            exe_args,
            vec![],
            vec![NinjaVariable::new("mod_id", exe.get_mod_id().to_string())],
        );
    });

    env.mut_.libraries.iter().for_each(|lib| {
        let mut need_cxx_linker = false;

        let lib_type = lib.get_type();

        let lib_args: Vec<NinjaRuleArg> = lib
            .get_sources()
            .iter()
            .filter_map(|src| {
                let rl;
                if CC::can_compile(src) {
                    rl = &cc_compile;
                    need_cc = true;
                } else if CXX::can_compile(src) {
                    rl = &cxx_compile;
                    need_cxx = true;
                    need_cxx_linker = true;
                } else if CC::can_consume(src) || CXX::can_consume(src) {
                    return None;
                } else {
                    println!(
                        "Warning: ignoring file '{}' while building library '{}'",
                        src,
                        lib.get_name()
                    );
                    return None;
                }
                let t = gen.new_target(
                    format!("{}@Lib/{}.o", lib.get_name(), src),
                    rl,
                    vec![NinjaRuleArg::new(format!("../{}", src))],
                    vec![],
                    vec![
                        NinjaVariable::new("mod_id", lib.get_mod_id().to_string()),
                        NinjaVariable::new(
                            "include_dirs",
                            lib.get_internal_include_dirs()
                                .iter()
                                .map(|inc_dir| format!("-I../{}", inc_dir))
                                .join(" "),
                        ),
                    ],
                );
                Some(NinjaRuleArg::new(t.get_name()))
            })
            .collect();
        let vars = vec![NinjaVariable::new("mod_id", lib.get_mod_id().to_string())];
        gen.new_target(
            lib_type.fmt_name(lib.get_name()),
            if need_cxx_linker { &cxx_link } else { &cc_link },
            lib_args,
            vec![],
            vars,
        );
    });

    if need_cc {
        let cc_loc = env.mut_.get_and_cache_cc().get_location();
        gen.new_global_value("CC", cc_loc.to_string_lossy())
    }

    if need_cxx {
        let cxx_loc = env.mut_.get_and_cache_cxx().get_location();
        gen.new_global_value("CXX", cxx_loc.to_string_lossy())
    }

    gen.write_to(f)?;

    Ok(())
}
