use crate::generators::{Generator, Rule, RuleArg, RuleOpt, RuleRef, Target, ToBuildSystemSyntax};
use itertools::Itertools;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::path::PathBuf;

pub struct NinjaCommand {
    command: String,
}

impl NinjaCommand {
    pub fn new(command: String) -> NinjaCommand {
        NinjaCommand { command }
    }
}

pub struct NinjaRule {
    name: String,
    command: NinjaCommand,
}

impl ToBuildSystemSyntax for NinjaRule {
    fn for_build_system(&self) -> String {
        format!("rule {}\n  command = {}", self.name, self.command.command)
    }
}

impl Rule for NinjaRule {
    type ArgType = NinjaRuleArg;
    type OptType = NinjaRuleOpt;
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
    fn new(value: String) -> NinjaRuleArg {
        NinjaRuleArg { value }
    }
    fn get_value(&self) -> &String {
        &self.value
    }
}

pub struct NinjaRuleOpt {
    name: String,
    value: String,
}

impl ToBuildSystemSyntax for NinjaRuleOpt {
    fn for_build_system(&self) -> String {
        format!("  {} = {}", self.name, self.value)
    }
}

impl RuleOpt for NinjaRuleOpt {
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
    rule_opts: Vec<NinjaRuleOpt>,
}

impl<'buildsys> ToBuildSystemSyntax for NinjaTarget<'buildsys> {
    fn for_build_system(&self) -> String {
        format!(
            "build {}: {} {}\n{}",
            self.name,
            self.rule.name,
            self.rule_args
                .iter()
                .map(|arg| { arg.for_build_system() })
                .join(" "),
            self.rule_opts
                .iter()
                .map(|opt| { opt.for_build_system() })
                .join("\n")
        )
    }
}

impl<'buildsys> Target<'buildsys, NinjaRule> for NinjaTarget<'buildsys> {
    fn new_from(
        name: String,
        rule: &'buildsys NinjaRuleRef,
        rule_args: Vec<NinjaRuleArg>,
        rule_opts: Vec<NinjaRuleOpt>,
    ) -> Self {
        Self {
            name,
            rule,
            rule_args,
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

    fn get_opts(&self) -> &Vec<NinjaRuleOpt> {
        &self.rule_opts
    }
}

pub struct NinjaGen<'buildsys> {
    rules: Vec<NinjaRule>,
    targets: Vec<NinjaTarget<'buildsys>>,
}

impl<'buildsys> Generator<'buildsys, NinjaRule, NinjaTarget<'buildsys>, NinjaCommand>
    for NinjaGen<'buildsys>
{
    fn new() -> NinjaGen<'buildsys> {
        // we start out with an empty build system
        NinjaGen {
            rules: vec![],
            targets: vec![],
        }
    }
    fn new_rule(&mut self, unique_name: String, command: NinjaCommand) -> NinjaRuleRef {
        let rule = NinjaRule {
            name: unique_name,
            command,
        };
        self.rules.push(rule);
        NinjaRuleRef {
            name: self.rules.last().unwrap().name.clone(),
        }
    }

    fn new_target(
        &mut self,
        name: String,
        rule: &'buildsys <NinjaRule as Rule>::RefType,
        args: Vec<<NinjaRule as Rule>::ArgType>,
        opts: Vec<<NinjaRule as Rule>::OptType>,
    ) -> &NinjaTarget<'buildsys> {
        let target = NinjaTarget::new_from(name, rule, args, opts);
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
            "{}\n{}",
            self.rules
                .iter()
                .map(|r| { r.for_build_system() })
                .join("\n"),
            self.targets
                .iter()
                .map(|t| {
                    format!(
                        "build {}: {} {}\n{}",
                        t.get_name(),
                        t.get_rule().name,
                        t.get_args()
                            .iter()
                            .map(|arg| { arg.for_build_system() })
                            .join(" "),
                        t.get_opts()
                            .iter()
                            .map(|opt| { opt.for_build_system() })
                            .join("\n")
                    )
                })
                .join("\n\n")
        )
    }
}
