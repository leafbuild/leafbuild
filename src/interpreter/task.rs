pub trait LeafTask {}

pub trait BuildTask: LeafTask {}

pub trait CreateCompileTimeResource: LeafTask {}

pub trait CreateRuntimeResource: LeafTask {}

pub trait CleanTask: LeafTask {}
