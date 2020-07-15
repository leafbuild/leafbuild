extern crate libcmbs;
extern crate libcmbscore;

use libcmbscore::generators::{
    *,
    ninja::*,
};
use std::fs::File;

fn main() {
    let mut gen = NinjaGen::new();
    let rl = gen.new_rule("cc".to_string(), NinjaCommand::new("cc $in -o $out".to_string()));
    gen.new_target("main".to_string(), &rl, vec![NinjaRuleArg::new("main.c".to_string())], vec![]);
    let file = File::create("build.ninja").unwrap();
    gen.write_to(file).unwrap();
}