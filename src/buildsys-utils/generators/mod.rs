use std::fs::File;
use std::path::PathBuf;

pub trait ToBuildSystemSyntax {
    /// Translates the struct to the final build system syntax

    fn for_build_system(&self) -> String;
}

pub trait Rule: ToBuildSystemSyntax {
    type ArgType: RuleArg + Sized;
    type VarType: RuleOpt + Sized;
    type RefType: RuleRef + Sized;
    fn get_name(&self) -> &String;
}

pub trait RuleArg: ToBuildSystemSyntax {
    fn new(value: impl Into<String>) -> Self;
    fn get_value(&self) -> &String;
}

pub trait RuleOpt: ToBuildSystemSyntax {
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self;
    fn get_arg_name(&self) -> &String;
    fn get_arg_value(&self) -> &String;
}

pub trait RuleRef {}

pub trait Target<'buildsys>: ToBuildSystemSyntax {
    type TargetRule: Rule + Sized;
    fn new_from(
        name: impl Into<String>,
        rule: &'buildsys <Self::TargetRule as Rule>::RefType,
        rule_args: Vec<<Self::TargetRule as Rule>::ArgType>,
        implicit_args: Vec<<Self::TargetRule as Rule>::ArgType>,
        rule_opts: Vec<<Self::TargetRule as Rule>::VarType>,
    ) -> Self;
    fn get_name(&self) -> &String;
    fn get_rule(&self) -> &<Self::TargetRule as Rule>::RefType;
    fn get_args(&self) -> &Vec<<Self::TargetRule as Rule>::ArgType>;
    fn get_implicit_args(&self) -> &Vec<<Self::TargetRule as Rule>::ArgType>;
    fn get_opts(&self) -> &Vec<<Self::TargetRule as Rule>::VarType>;
}

pub trait Generator<'buildsys>: ToBuildSystemSyntax {
    type RuleType: Rule + Sized;
    type TargetType: Target<'buildsys, TargetRule = Self::RuleType> + Sized;
    type CommandType;

    fn new() -> Self;

    fn new_global_value(&mut self, unique_name: impl Into<String>, value: impl Into<String>);

    fn new_rule(
        &mut self,
        unique_name: impl Into<String>,
        command: Self::CommandType,
        variables: Vec<<Self::RuleType as Rule>::VarType>,
    ) -> <Self::RuleType as Rule>::RefType;
    fn new_target(
        &mut self,
        name: impl Into<String>,
        rule: &'buildsys <Self::RuleType as Rule>::RefType,
        args: Vec<<Self::RuleType as Rule>::ArgType>,
        implicit_args: Vec<<Self::RuleType as Rule>::ArgType>,
        variables: Vec<<Self::RuleType as Rule>::VarType>,
    ) -> &Self::TargetType;

    fn filename(&self) -> String;
    /// Writes everything to the file, in a format the underlying build system will accept.
    /// # Errors
    /// Any io errors resulting from writing to the file
    fn write_to(&self, file: File) -> std::io::Result<()>;

    fn find_backend() -> Option<PathBuf>;
}

#[path = "ninja/gen.rs"]
pub mod ninja;
#[path = "make/gen.rs"]
pub mod unix_makefiles;
