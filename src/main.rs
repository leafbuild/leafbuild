extern crate libsbuild;
extern crate libsbuildcore;

use libsbuildcore::generators::{
    *,
    ninja::*,
};

fn main() {
    let gen = NinjaGen::new();
    println!("{}", gen.new_rule("cc".to_string(), NinjaCommand::new("cc $in".to_string())).for_build_system());
}