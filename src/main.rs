extern crate libsbuild;

fn main() {
    libsbuild::grammar::parse("a(b:2+2-3)");
}
