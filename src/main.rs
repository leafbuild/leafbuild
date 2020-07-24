extern crate libleaf;
extern crate libleafcore;

use libleafcore::generators::{ninja::*, *};

use libleaf::{grammar, handle::Handle, interpreter};
use std::fs::File;

fn main() {
    let mut handle = Handle::new();
    let program = grammar::parse(
        "x = 2\nx += 2.as_str\nprint(\nx,\n (0 + 1 + 2).to_string(),\n z: x + 2\n)\n",
    )
    .unwrap();
    interpreter::interpret_wrapper(&program, &mut handle);
    let mut gen = NinjaGen::new();
    let rl = gen.new_rule(
        "cc".to_string(),
        NinjaCommand::new("cc $in -o $out".to_string()),
    );
    gen.new_target(
        "main".to_string(),
        &rl,
        vec![NinjaRuleArg::new("main.c".to_string())],
        vec![],
    );
    let file = File::create("build.ninja").unwrap();
    gen.write_to(file).unwrap();
}
