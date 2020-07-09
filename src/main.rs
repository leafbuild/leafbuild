extern crate libsbuild;

fn main() {
    libsbuild::grammar::parse("a(2,b:2+2-3).c(2+bcd,b:a+c)").unwrap();
}
