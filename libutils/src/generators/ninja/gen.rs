use crate::generators::{Generator, Rule, RuleArg, RuleOpt, RuleRef, Target, ToBuildSystemSyntax};
use itertools::Itertools;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::PathBuf;

pub struct NinjaCommand {
    command: String,
}

impl NinjaCommand {
    pub fn new(command: impl Into<String>) -> NinjaCommand {
        NinjaCommand {
            command: command.into(),
        }
    }
}

pub struct NinjaRule {
    name: String,
    command: NinjaCommand,
    variables: Vec<NinjaVariable>,
}

impl ToBuildSystemSyntax for NinjaRule {
    fn for_build_system(&self) -> String {
        format!(
            "rule {}\n  {}",
            self.name,
            self.variables
                .iter()
                .map(|var| var.for_build_system())
                .chain(
                    [&self.command.command]
                        .iter()
                        .map(|cmd| { format!("command = {}", *cmd) })
                )
                .join("\n  ")
        )
    }
}

impl Rule for NinjaRule {
    type ArgType = NinjaRuleArg;
    type VarType = NinjaVariable;
    type RefType = NinjaRuleRef;

    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct NinjaRuleRef {
    name: String,
}

impl RuleRef for NinjaRuleRef {}

pub struct NinjaRuleArg {
    value: String,
}

impl ToBuildSystemSyntax for NinjaRuleArg {
    fn for_build_system(&self) -> String {
        String::clone(&self.value)
    }
}

impl RuleArg for NinjaRuleArg {
    fn new(value: impl Into<String>) -> NinjaRuleArg {
        NinjaRuleArg {
            value: value.into(),
        }
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}

pub struct NinjaVariable {
    name: String,
    value: String,
}

impl ToBuildSystemSyntax for NinjaVariable {
    fn for_build_system(&self) -> String {
        format!("  {} = {}", self.name, self.value)
    }
}

impl RuleOpt for NinjaVariable {
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    fn get_arg_name(&self) -> &String {
        &self.name
    }

    fn get_arg_value(&self) -> &String {
        &self.value
    }
}

pub struct NinjaTarget<'buildsys> {
    name: String,
    rule: &'buildsys NinjaRuleRef,
    rule_args: Vec<NinjaRuleArg>,
    implicit_args: Vec<NinjaRuleArg>,
    rule_opts: Vec<NinjaVariable>,
}

impl<'buildsys> ToBuildSystemSyntax for NinjaTarget<'buildsys> {
    fn for_build_system(&self) -> String {
        format!(
            "build {}: {} {}{}\n{}",
            self.name,
            self.rule.name,
            self.rule_args
                .iter()
                .map(|arg| { arg.for_build_system() })
                .join(" "),
            if !self.implicit_args.is_empty() {
                format!(
                    " | {}",
                    self.implicit_args.iter().map(|arg| &arg.value).join(" ")
                )
            } else {
                "".to_string()
            },
            self.rule_opts
                .iter()
                .map(|opt| { opt.for_build_system() })
                .join("\n")
        )
    }
}

impl<'buildsys> Target<'buildsys, NinjaRule> for NinjaTarget<'buildsys> {
    fn new_from(
        name: impl Into<String>,
        rule: &'buildsys NinjaRuleRef,
        rule_args: Vec<NinjaRuleArg>,
        implicit_args: Vec<NinjaRuleArg>,
        rule_opts: Vec<NinjaVariable>,
    ) -> Self {
        Self {
            name: name.into(),
            rule,
            rule_args,
            implicit_args,
            rule_opts,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_rule(&self) -> &NinjaRuleRef {
        &self.rule
    }

    fn get_args(&self) -> &Vec<NinjaRuleArg> {
        &self.rule_args
    }

    fn get_implicit_args(&self) -> &Vec<<NinjaRule as Rule>::ArgType> {
        &self.implicit_args
    }

    fn get_opts(&self) -> &Vec<NinjaVariable> {
        &self.rule_opts
    }
}

struct NinjaGlobalValue {
    name: String,
    value: String,
}

impl NinjaGlobalValue {
    fn new(name: impl Into<String>, value: impl Into<String>) -> NinjaGlobalValue {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl ToBuildSystemSyntax for NinjaGlobalValue {
    fn for_build_system(&self) -> String {
        format!("{} = {}", self.name, self.value)
    }
}

pub struct NinjaGen<'buildsys> {
    rules: Vec<NinjaRule>,
    targets: Vec<NinjaTarget<'buildsys>>,
    global_values: Vec<NinjaGlobalValue>,
}

impl<'buildsys> Generator<'buildsys, NinjaRule, NinjaTarget<'buildsys>, NinjaCommand>
    for NinjaGen<'buildsys>
{
    fn new() -> NinjaGen<'buildsys> {
        // we start out with an empty build system
        NinjaGen {
            rules: vec![],
            targets: vec![],
            global_values: vec![],
        }
    }

    fn new_global_value(&mut self, unique_name: impl Into<String>, value: impl Into<String>) {
        self.global_values
            .push(NinjaGlobalValue::new(unique_name, value));
    }

    fn new_rule(
        &mut self,
        unique_name: impl Into<String>,
        command: NinjaCommand,
        variables: Vec<NinjaVariable>,
    ) -> NinjaRuleRef {
        let rule = NinjaRule {
            name: unique_name.into(),
            command,
            variables,
        };
        self.rules.push(rule);
        NinjaRuleRef {
            name: self.rules.last().unwrap().name.clone(),
        }
    }

    fn new_target(
        &mut self,
        name: impl Into<String>,
        rule: &'buildsys <NinjaRule as Rule>::RefType,
        args: Vec<<NinjaRule as Rule>::ArgType>,
        implicit_args: Vec<<NinjaRule as Rule>::ArgType>,
        opts: Vec<<NinjaRule as Rule>::VarType>,
    ) -> &NinjaTarget<'buildsys> {
        let target = NinjaTarget::new_from(name, rule, args, implicit_args, opts);
        self.targets.push(target);
        self.targets.last().unwrap()
    }

    fn filename(&self) -> String {
        "build.ninja".to_string()
    }

    fn write_to(&self, mut file: File) -> IoResult<()> {
        file.write_all(self.for_build_system().as_bytes())
    }

    fn find_backend() -> Option<PathBuf> {
        match which::which("ninja") {
            Ok(buf) => Some(buf),
            Err(_) => None,
        }
    }
}

impl<'buildsys> ToBuildSystemSyntax for NinjaGen<'buildsys> {
    fn for_build_system(&self) -> String {
        format!(
            "{}\n\n{}\n\n\n\n\n{}\n",
            self.global_values
                .iter()
                .map(|v| v.for_build_system())
                .join("\n"),
            self.rules
                .iter()
                .filter_map(|r| {
                    if self.targets.iter().any(|x| x.rule.name == r.name) {
                        Some(r.for_build_system())
                    } else {
                        None
                    }
                })
                .join("\n\n"),
            self.targets
                .iter()
                .map(|t| t.for_build_system())
                .join("\n\n")
        )
    }
}
