use std::fs::File;
use std::path::PathBuf;

pub trait ToBuildSystemSyntax {
    /// Translates the struct to the final build system syntax

    fn for_build_system(&self) -> String;
}

pub trait Rule: ToBuildSystemSyntax {
    type ArgType: RuleArg + Sized;
    type OptType: RuleOpt + Sized;
    type RefType: RuleRef + Sized;
    fn get_name(&self) -> &String;
}

pub trait RuleArg: ToBuildSystemSyntax {
    fn new(value: impl Into<String>) -> Self;
    fn get_value(&self) -> &String;
}

pub trait RuleOpt: ToBuildSystemSyntax {
    fn get_arg_name(&self) -> &String;
    fn get_arg_value(&self) -> &String;
}

pub trait RuleRef {}

pub trait Target<'buildsys, TargetRule>: ToBuildSystemSyntax
where
    TargetRule: Rule + Sized,
{
    fn new_from(
        name: impl Into<String>,
        rule: &'buildsys TargetRule::RefType,
        rule_args: Vec<TargetRule::ArgType>,
        rule_opts: Vec<TargetRule::OptType>,
    ) -> Self;
    fn get_name(&self) -> &String;
    fn get_rule(&self) -> &TargetRule::RefType;
    fn get_args(&self) -> &Vec<TargetRule::ArgType>;
    fn get_opts(&self) -> &Vec<TargetRule::OptType>;
}

pub trait Generator<'buildsys, RuleType, TargetType, CommandType>: ToBuildSystemSyntax
where
    RuleType: Rule + Sized,
    TargetType: Target<'buildsys, RuleType> + Sized,
{
    fn new() -> Self;

    fn new_global_value(&mut self, unique_name: impl Into<String>, value: impl Into<String>);

    fn new_rule(
        &mut self,
        unique_name: impl Into<String>,
        command: CommandType,
    ) -> RuleType::RefType;
    fn new_target(
        &mut self,
        name: impl Into<String>,
        rule: &'buildsys RuleType::RefType,
        args: Vec<RuleType::ArgType>,
        opts: Vec<RuleType::OptType>,
    ) -> &TargetType;

    fn filename(&self) -> String;
    fn write_to(&self, file: File) -> std::io::Result<()>;

    fn find_backend() -> Option<PathBuf>;
}

#[path = "ninja/gen.rs"]
pub mod ninja;
#[path = "make/gen.rs"]
pub mod unix_makefiles;
