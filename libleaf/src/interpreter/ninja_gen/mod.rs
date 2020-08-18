use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use libutils::{
    compilers::{cc::*, cxx::*, *},
    generators::{ninja::*, *},
};

use crate::interpreter::types::LibType;
use crate::interpreter::Env;
use itertools::Itertools;
use libutils::compilers::flags::{CompilationFlag, LinkFlag, LinkFlags};
use libutils::utils::{get_ar, Language};

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

    let signal_build_failure = env.mut_.diagnostics_ctx.get_signal_build_failure();

    let internal_compilation_failed_call = if signal_build_failure {
        "|| $lfb_bin internal compilation-failed --exit-code $$? --in \"$in\" --out \"$out\" --module-id $mod_id"
    } else {
        ""
    };
    let internal_linking_failed_call = if signal_build_failure {
        "|| $lfb_bin internal link-failed --exit-code $$? --in \"$in\" --out \"$out\" --module-id $mod_id"
    } else {
        ""
    };

    let cc_compile = gen.new_rule(
        "cc_compile",
        NinjaCommand::new(format!(
            "$CC ${} -MD -MF $out.d $include_dirs $in -c -o $out {}",
            Language::C.get_compilation_flags_varname(),
            internal_compilation_failed_call
        )),
        vec![
            NinjaVariable::new("depfile", "$out.d"),
            NinjaVariable::new("description", "Compiling C object $out"),
        ],
    );
    let cc_link = gen.new_rule(
        "cc_link",
        NinjaCommand::new(format!(
            "$CC $in -o $out ${} {}",
            Language::C.get_link_flags_varname(),
            internal_linking_failed_call
        )),
        vec![NinjaVariable::new("description", "Linking C object $out")],
    );

    let cxx_compile = gen.new_rule(
        "cxx_compile",
        NinjaCommand::new(format!(
            "$CXX ${} -MD -MF $out.d $include_dirs $in -c -o $out {}",
            Language::CPP.get_compilation_flags_varname(),
            internal_compilation_failed_call
        )),
        vec![
            NinjaVariable::new("depfile", "$out.d"),
            NinjaVariable::new("description", "Compiling C++ object $out"),
        ],
    );
    let cxx_link = gen.new_rule(
        "cxx_link",
        NinjaCommand::new(format!(
            "$CXX $in -o $out ${} {}",
            Language::CPP.get_link_flags_varname(),
            internal_linking_failed_call
        )),
        vec![NinjaVariable::new("description", "Linking C++ object $out")],
    );

    let make_static_lib = gen.new_rule(
        "make_static_lib",
        NinjaCommand::new("$AR rcs $out $in"),
        vec![NinjaVariable::new(
            "description",
            "Making static library $out",
        )],
    );
    let cc = env.mut_.get_and_cache_cc();
    let cc_loc = cc.get_location();
    gen.new_global_value("CC", cc_loc.to_string_lossy());

    let cxx = env.mut_.get_and_cache_cxx();
    let cxx_loc = cxx.get_location();
    gen.new_global_value("CXX", cxx_loc.to_string_lossy());

    let ar_loc = get_ar().unwrap();
    gen.new_global_value("AR", ar_loc.to_string_lossy());

    env.mut_.executables.iter().for_each(|exe| {
        let mut need_cxx_linker = false;

        let exe_args: Vec<NinjaRuleArg> = exe
            .get_sources()
            .iter()
            .filter_map(|src| {
                let rl;
                let lng;
                if CC::can_compile(src) {
                    rl = &cc_compile;
                    lng = Language::C;
                } else if CXX::can_compile(src) {
                    rl = &cxx_compile;
                    need_cxx_linker = true;
                    lng = Language::CPP;
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
                    vec![NinjaRuleArg::new(
                        env.get_root_path_for_module(exe.get_mod_id())
                            .unwrap()
                            .clone()
                            .join(PathBuf::from(src))
                            .to_string_lossy()
                            .to_string(),
                    )],
                    vec![],
                    vec![
                        NinjaVariable::new("mod_id", exe.get_mod_id().to_string()),
                        NinjaVariable::new(
                            "include_dirs",
                            exe.get_include_dirs()
                                .iter()
                                .map(|inc_dir| {
                                    let flag = CompilationFlag::IncludeDir {
                                        include_dir: env
                                            .get_root_path_for_module(exe.get_mod_id())
                                            .unwrap()
                                            .clone()
                                            .join(PathBuf::from(inc_dir))
                                            .to_string_lossy()
                                            .to_string(),
                                    };
                                    match lng {
                                        Language::C => cc.get_flag(flag),
                                        Language::CPP => cxx.get_flag(flag),
                                    }
                                })
                                .join(" "),
                        ),
                        NinjaVariable::new(
                            lng.get_compilation_flags_varname(),
                            exe.get_dependencies()
                                .iter()
                                .map(|dep| dep.get_compiler_flags(lng, env))
                                .map(|flags| match lng {
                                    Language::C => cc.get_flags(flags),
                                    Language::CPP => cxx.get_flags(flags),
                                })
                                .join(" "),
                        ),
                    ],
                );
                Some(NinjaRuleArg::new(t.get_name()))
            })
            .collect();
        let lng = if need_cxx_linker {
            Language::CPP
        } else {
            Language::C
        };
        gen.new_target(
            exe.get_name(),
            if need_cxx_linker { &cxx_link } else { &cc_link },
            exe_args,
            exe.get_dependencies()
                .iter()
                .map(|dep| dep.get_implicit_requirements(lng, env))
                .flatten()
                .map(NinjaRuleArg::new)
                .collect_vec(),
            vec![
                NinjaVariable::new("mod_id", exe.get_mod_id().to_string()),
                NinjaVariable::new(
                    lng.get_link_flags_varname(),
                    exe.get_dependencies()
                        .iter()
                        .map(|dep| dep.get_linker_flags(lng, env))
                        .map(|flags| match lng {
                            Language::C => cc.get_linker_flags(flags),
                            Language::CPP => cxx.get_linker_flags(flags),
                        })
                        .join(" "),
                ),
            ],
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
                let lng;
                if CC::can_compile(src) {
                    rl = &cc_compile;
                    lng = Language::C;
                } else if CXX::can_compile(src) {
                    rl = &cxx_compile;
                    need_cxx_linker = true;
                    lng = Language::CPP;
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
                    vec![NinjaRuleArg::new(
                        env.get_root_path_for_module(lib.get_mod_id())
                            .unwrap()
                            .clone()
                            .join(PathBuf::from(src))
                            .to_string_lossy()
                            .to_string(),
                    )],
                    vec![],
                    vec![
                        NinjaVariable::new("mod_id", lib.get_mod_id().to_string()),
                        NinjaVariable::new(
                            "include_dirs",
                            lib.get_internal_include_dirs()
                                .iter()
                                .chain(lib.get_external_include_dirs())
                                .map(|inc_dir| {
                                    let flag = CompilationFlag::IncludeDir {
                                        include_dir: env
                                            .get_root_path_for_module(lib.get_mod_id())
                                            .unwrap()
                                            .clone()
                                            .join(PathBuf::from(inc_dir))
                                            .to_string_lossy()
                                            .to_string(),
                                    };
                                    match lng {
                                        Language::C => cc.get_flag(flag),
                                        Language::CPP => cxx.get_flag(flag),
                                    }
                                })
                                .join(" "),
                        ),
                        NinjaVariable::new(
                            lng.get_compilation_flags_varname(),
                            match lng {
                                Language::C => cc.get_flags(lib.source_compilation_flags()),
                                Language::CPP => cxx.get_flags(lib.source_compilation_flags()),
                            },
                        ),
                    ],
                );
                Some(NinjaRuleArg::new(t.get_name()))
            })
            .collect();
        let (lang, linker) = match lib.get_type() {
            LibType::Static => (
                match lib.get_language() {
                    Some(lang) => lang,
                    None => {
                        if need_cxx_linker {
                            Language::CPP
                        } else {
                            Language::C
                        }
                    }
                },
                &make_static_lib,
            ),
            LibType::Shared => {
                if need_cxx_linker
                    || match lib.get_language() {
                        Some(Language::CPP) => true,
                        _ => false,
                    }
                {
                    (Language::CPP, &cxx_link)
                } else {
                    (Language::C, &cc_link)
                }
            }
        };
        let vars = vec![
            NinjaVariable::new("mod_id", lib.get_mod_id().to_string()),
            NinjaVariable::new(lang.get_link_flags_varname(), {
                let flags = LinkFlags::new(vec![match lib.get_type() {
                    LibType::Shared => LinkFlag::LibShared,
                    LibType::Static => LinkFlag::None,
                }]);
                match lang {
                    Language::C => cc.get_linker_flags(flags),
                    Language::CPP => cxx.get_linker_flags(flags),
                }
            }),
        ];
        gen.new_target(
            lib_type.fmt_name(lib.get_name()),
            linker,
            lib_args,
            vec![],
            vars,
        );
    });

    gen.write_to(f)?;

    Ok(())
}
