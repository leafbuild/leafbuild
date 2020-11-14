//! The ninja generator
use crate::buildsys_utils::generators::{
    Generator, Rule, RuleArg, RuleOpt, RuleRef, Target, ToBuildSystemSyntax,
};
use itertools::Itertools;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::PathBuf;

/// A ninja command(just a string)
pub struct NjCommand {
    command: String,
}

impl NjCommand {
    /// Creates a ninja command from a string.
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
        }
    }
}

/// A ninja rule
pub struct NjRule {
    name: String,
    command: NjCommand,
    variables: Vec<NjVariable>,
}

impl ToBuildSystemSyntax for NjRule {
    fn for_build_system(&self) -> String {
        format!(
            "rule {}\n{}",
            self.name,
            self.variables
                .iter()
                .map(|var| var.for_build_system())
                .chain(
                    [&self.command.command]
                        .iter()
                        .map(|cmd| { format!("  command = {}", *cmd) })
                )
                .join("\n")
        )
    }
}

impl Rule for NjRule {
    type ArgType = NjRuleArg;
    type OptType = NjVariable;
    type RefType = NjRuleRef;

    fn get_name(&self) -> &String {
        &self.name
    }
}

/// A ninja rule reference
pub struct NjRuleRef {
    name: String,
}

impl RuleRef for NjRuleRef {}

/// A ninja rule argument
pub struct NjRuleArg {
    value: String,
}

impl ToBuildSystemSyntax for NjRuleArg {
    fn for_build_system(&self) -> String {
        String::clone(&self.value)
    }
}

impl RuleArg for NjRuleArg {
    fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}

/// A ninja variable (corresponds to [`RuleOpt`])
pub struct NjVariable {
    name: String,
    value: String,
}

impl ToBuildSystemSyntax for NjVariable {
    fn for_build_system(&self) -> String {
        format!("  {} = {}", self.name, self.value)
    }
}

impl RuleOpt for NjVariable {
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    fn get_opt_name(&self) -> &String {
        &self.name
    }

    fn get_opt_value(&self) -> &String {
        &self.value
    }
}

/// A ninja build target
pub struct NjTarget<'buildsys> {
    name: String,
    rule: &'buildsys NjRuleRef,
    rule_args: Vec<NjRuleArg>,
    implicit_args: Vec<NjRuleArg>,
    rule_opts: Vec<NjVariable>,
}

impl<'buildsys> ToBuildSystemSyntax for NjTarget<'buildsys> {
    fn for_build_system(&self) -> String {
        format!(
            "build {}: {} {}{}\n{}",
            self.name,
            self.rule.name,
            self.rule_args
                .iter()
                .map(|arg| { arg.for_build_system() })
                .join(" "),
            if self.implicit_args.is_empty() {
                "".to_string()
            } else {
                format!(
                    " | {}",
                    self.implicit_args.iter().map(|arg| &arg.value).join(" ")
                )
            },
            self.rule_opts
                .iter()
                .map(|opt| { opt.for_build_system() })
                .join("\n")
        )
    }
}

impl<'buildsys> Target<'buildsys> for NjTarget<'buildsys> {
    type TargetRule = NjRule;

    fn new_from(
        name: impl Into<String>,
        rule: &'buildsys NjRuleRef,
        rule_args: Vec<NjRuleArg>,
        implicit_args: Vec<NjRuleArg>,
        rule_opts: Vec<NjVariable>,
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

    fn get_rule(&self) -> &NjRuleRef {
        self.rule
    }

    fn get_args(&self) -> &Vec<NjRuleArg> {
        &self.rule_args
    }

    fn get_implicit_args(&self) -> &Vec<<NjRule as Rule>::ArgType> {
        &self.implicit_args
    }

    fn get_opts(&self) -> &Vec<NjVariable> {
        &self.rule_opts
    }
}

struct NinjaGlobalValue {
    name: String,
    value: String,
}

impl NinjaGlobalValue {
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
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

/// The ninja generator
pub struct NjGen<'buildsys> {
    rules: Vec<NjRule>,
    targets: Vec<NjTarget<'buildsys>>,
    global_values: Vec<NinjaGlobalValue>,
}

impl<'buildsys> Generator<'buildsys> for NjGen<'buildsys> {
    type RuleType = NjRule;
    type TargetType = NjTarget<'buildsys>;
    type CommandType = NjCommand;
    fn new() -> NjGen<'buildsys> {
        // we start out with an empty build system
        NjGen {
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
        command: NjCommand,
        variables: Vec<NjVariable>,
    ) -> NjRuleRef {
        let rule = NjRule {
            name: unique_name.into(),
            command,
            variables,
        };
        self.rules.push(rule);
        NjRuleRef {
            name: self.rules.last().unwrap().name.clone(),
        }
    }

    fn new_target(
        &mut self,
        name: impl Into<String>,
        rule: &'buildsys <NjRule as Rule>::RefType,
        args: Vec<<NjRule as Rule>::ArgType>,
        implicit_args: Vec<<NjRule as Rule>::ArgType>,
        opts: Vec<<NjRule as Rule>::OptType>,
    ) -> &NjTarget<'buildsys> {
        let target = NjTarget::new_from(name, rule, args, implicit_args, opts);
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

impl<'buildsys> ToBuildSystemSyntax for NjGen<'buildsys> {
    fn for_build_system(&self) -> String {
        format!(
            "{}\n\n{}\n\n{}\n\n\n\n\n{}\n",
            "# This file was generated by the Leaf Build System and should NOT be modified manually",
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
