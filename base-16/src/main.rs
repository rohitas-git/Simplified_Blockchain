mod lib;
use lib::*;

fn main() {
    let n = Number::new(16);
    n.to_base16();
}
