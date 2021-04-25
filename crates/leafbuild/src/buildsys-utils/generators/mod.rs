//! Generators for all the build systems leafbuild supports.
//! Mostly modeled around ninja's syntax.
use std::fs::File;
use std::path::PathBuf;

/// A trait for translating a particular structure to the syntax of the underlying build system.
pub trait ToBuildSystemSyntax {
    /// Translates the struct to the final build system syntax

    fn for_build_system(&self) -> String;
}

/// A rule in the specified syntax
pub trait Rule: ToBuildSystemSyntax {
    /// The type of the arguments
    type ArgType: RuleArg + Sized;
    /// The type of the options
    type OptType: RuleOpt + Sized;
    /// The type of the references
    type RefType: RuleRef + Sized;
    /// Returns the name of the rule
    fn get_name(&self) -> &String;
}

/// A rule argument
pub trait RuleArg: ToBuildSystemSyntax {
    /// Constructor from value
    fn new(value: impl Into<String>) -> Self;
    /// Returns the value
    fn get_value(&self) -> &String;
}

/// A rule option
pub trait RuleOpt: ToBuildSystemSyntax {
    /// Constructor from name and value
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self;
    /// Returns the name
    fn get_opt_name(&self) -> &String;
    /// Returns the value
    fn get_opt_value(&self) -> &String;
}

/// A rule reference
pub trait RuleRef {}

/// A target in the build system
/// (make calls this a recipe)
pub trait Target<'buildsys>: ToBuildSystemSyntax {
    /// The type of rule
    type TargetRule: Rule + Sized;
    /// Constructor from name, rule, rule args, implicit args and rule options
    fn new_from(
        name: impl Into<String>,
        rule: &'buildsys <Self::TargetRule as Rule>::RefType,
        rule_args: Vec<<Self::TargetRule as Rule>::ArgType>,
        implicit_args: Vec<<Self::TargetRule as Rule>::ArgType>,
        rule_opts: Vec<<Self::TargetRule as Rule>::OptType>,
    ) -> Self;
    /// Returns the name
    fn get_name(&self) -> &String;
    /// Returns the rule
    fn get_rule(&self) -> &<Self::TargetRule as Rule>::RefType;
    /// Returns the args
    fn get_args(&self) -> &Vec<<Self::TargetRule as Rule>::ArgType>;
    /// Returns the implicit args
    fn get_implicit_args(&self) -> &Vec<<Self::TargetRule as Rule>::ArgType>;
    /// Returns the options
    fn get_opts(&self) -> &Vec<<Self::TargetRule as Rule>::OptType>;
}

/// The generator
pub trait Generator<'buildsys>: ToBuildSystemSyntax {
    /// The type of rules
    type RuleType: Rule + Sized;
    /// The type of targets
    type TargetType: Target<'buildsys, TargetRule = Self::RuleType> + Sized;
    /// The type of commands
    type CommandType;

    /// Constructor
    fn new() -> Self;

    /// Creates a new global "variable", given the name and value
    fn new_global_value(&mut self, unique_name: impl Into<String>, value: impl Into<String>);

    /// Creates a new rule from the name, command and variables
    fn new_rule(
        &mut self,
        unique_name: impl Into<String>,
        command: Self::CommandType,
        options: Vec<<Self::RuleType as Rule>::OptType>,
    ) -> <Self::RuleType as Rule>::RefType;
    /// Creates a new target from the name, rule, arguments, implicit arguments and options
    fn new_target(
        &mut self,
        name: impl Into<String>,
        rule: &'buildsys <Self::RuleType as Rule>::RefType,
        args: Vec<<Self::RuleType as Rule>::ArgType>,
        implicit_args: Vec<<Self::RuleType as Rule>::ArgType>,
        options: Vec<<Self::RuleType as Rule>::OptType>,
    ) -> &Self::TargetType;

    /// Returns the file name this should output to.
    /// `Makefile` for make,
    /// `build.ninja` for ninja,
    /// and so on.
    fn filename(&self) -> String;
    /// Writes everything to the file, in a format the underlying build system will accept.
    /// # Errors
    /// Any io errors resulting from writing to the file
    fn write_to(&self, file: File) -> std::io::Result<()>;

    /// Find the backend (ninja/make executable) or return none if it wasn't found.
    fn find_backend() -> Option<PathBuf>;
}

#[path = "ninja/gen.rs"]
pub mod ninja;
#[path = "make/gen.rs"]
pub mod unix_makefiles;
