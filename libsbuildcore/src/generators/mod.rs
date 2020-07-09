use std::fs::File;

trait Rule {
    fn get_name(&self) -> &String;
    fn set_name(&mut self);
}

trait Target {
    fn get_rule(&self) -> &dyn Rule;
}

trait Generator {
    fn new_rule(&self, unique_name: &str) -> Box<dyn Rule>;

    fn filename(&self) -> &String;
    fn write_to(&self, file: File);
}

#[path = "make/gen.rs"]
mod unix_makefiles;
#[path = "ninja/gen.rs"]
mod ninja;
#[path = "windows_nmake/gen.rs"]
mod windows_nmake;