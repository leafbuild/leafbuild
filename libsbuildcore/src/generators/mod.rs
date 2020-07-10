use std::fs::File;

pub trait ToBuildSystemSyntax {
    /// Translates the struct to the final build system syntax

    fn for_build_system(&self) -> String;
}

pub trait Rule: ToBuildSystemSyntax {
    fn get_name(&self) -> &String;
}

pub trait RuleArg: ToBuildSystemSyntax {
    fn get_value(&self) -> &String;
}

pub trait RuleOpt: ToBuildSystemSyntax {
    fn get_arg_name(&self) -> &String;
    fn get_arg_value(&self) -> &String;
}

pub trait Target<'buildsys, TargetRule, TargetRuleArg, TargetRuleOpt>: ToBuildSystemSyntax
    where TargetRule: Rule + Sized,
          TargetRuleArg: RuleArg + Sized,
          TargetRuleOpt: RuleOpt + Sized {
    fn new_from(name: String, rule: &'buildsys TargetRule, rule_args: Vec<TargetRuleArg>, rule_opts: Vec<TargetRuleOpt>) -> Self;
    fn get_name(&self) -> &String;
    fn get_rule(&self) -> &TargetRule;
    fn get_args(&self) -> &Vec<TargetRuleArg>;
    fn get_opts(&self) -> &Vec<TargetRuleOpt>;
}

pub trait Generator<'buildsys, RuleType, CommandType>: ToBuildSystemSyntax
    where RuleType: Rule + Sized {
    fn new() -> Self;
    fn new_rule(&self, unique_name: String, command: CommandType) -> RuleType;

    fn filename(&self) -> String;
    fn write_to(&self, file: File) -> std::io::Result<()>;
}

#[path = "make/gen.rs"]
pub mod unix_makefiles;
#[path = "ninja/gen.rs"]
pub mod ninja;
#[cfg(target_os = "windows")]
#[path = "windows_nmake/gen.rs"]
pub mod windows_nmake;